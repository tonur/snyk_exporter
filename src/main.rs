use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics, RequestTracing};
use async_std::task;
use clap::Parser;
use lazy_static::lazy_static;
use openapi::models::{Issue, ScanItemType};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};
use prometheus::{IntGaugeVec, Opts, Registry};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref SNYK_VULNERABILITIES_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            "snyk_vulnerabilities_total",
            "Gauge of Snyk vulnerabilities"
        ),
        &[
            "severity",
            "issue_type",
            "title",
            "ignored",
            "upgradeable",
            "patchable",
            "count"
        ],
    )
    .unwrap();
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about, arg_required_else_help = true)]
pub struct AppArguments {
    /// Snyk API token
    #[arg(long = "snyk.api-token", required = true)]
    snyk_api_token: String,
    /// Snyk API URL
    #[arg(long = "snyk.api-url", default_value = "https://api.snyk.io")]
    snyk_api_url: String,
    /// Polling interval for requesting data from Snyk API in seconds
    #[arg(long = "snyk.interval", short = 'i', default_value = "600")]
    snyk_interval: i32,
    /// Snyk organization ID to scrape projects from (can be repeated for multiple organizations)
    #[arg(long = "snyk.organization")]
    snyk_organizations: Option<String>,
    /// Timeout for requests against Snyk API
    #[arg(long = "snyk.timeout", default_value = "10")]
    request_timeout: i32,
    /// Address on which to expose metrics
    #[arg(long = "web.listen-address", default_value = ":8080")]
    listen_address: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let arguments = AppArguments::parse();

    log::debug!("Starting snyk_exporter with arguments: {:?}", arguments);

    REGISTRY
        .register(Box::new(SNYK_VULNERABILITIES_TOTAL.clone()))
        .unwrap();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(REGISTRY.clone())
        .build()?;

    // set up your meter provider with your exporter(s)
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .with_resource(Resource::new([KeyValue::new(
            "service.name",
            "tenable_exporter",
        )]))
        .build();
    global::set_meter_provider(provider);

    task::spawn(async move {
        loop {
            poll_api(&arguments).await;
            task::sleep(Duration::from_secs(300)).await;
        }
    });

    let _ = HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .route(
                "/metrics",
                web::get().to(PrometheusMetricsHandler::new(REGISTRY.clone())),
            )
    })
    .bind("localhost:8080")?
    .run()
    .await;

    Ok(())
}

pub async fn poll_api(arguments: &AppArguments) {
    let organizations = get_organizations(arguments).await;
    loop {
        for organization_id in organizations.iter() {
            let projects = get_projects(arguments, organization_id.to_string()).await;

            for project_id in projects.iter() {
                let issues = get_issues(
                    arguments,
                    organization_id.to_string(),
                    project_id.to_string(),
                )
                .await;
                let results = aggregate_issues(issues);
                SNYK_VULNERABILITIES_TOTAL
                    .with_label_values(&[
                        &results[0].severity,
                        &results[0].issue_type,
                        &results[0].title,
                        &results[0].ignored.to_string(),
                        &results[0].upgradeable.to_string(),
                        &results[0].patchable.to_string(),
                        &results[0].count.to_string(),
                    ])
                    .set(results[0].count.into());
            }
        }
    }
}

#[derive(Clone, Debug)]
struct AggregateResult {
    issue_type: String,
    title: String,
    severity: String,
    ignored: bool,
    upgradeable: bool,
    patchable: bool,
    count: i32,
}

fn aggregation_key(issue: &Issue) -> String {
    let issue_coordinates = issue.attributes.coordinates.as_ref().unwrap();
    format!(
        "{}_{}_{}_{}_{}_{}",
        serde_json::to_string(&issue.attributes.effective_severity_level).unwrap(),
        serde_json::to_string(&issue.attributes.r#type).unwrap(),
        issue.attributes.title,
        issue.attributes.ignored,
        issue_coordinates.iter().any(|c| c.is_upgradeable.unwrap()),
        issue_coordinates.iter().any(|c| c.is_patchable.unwrap()),
    )
}

fn aggregate_issues(issues: Vec<Issue>) -> Vec<AggregateResult> {
    let mut aggregate_results: HashMap<String, AggregateResult> = HashMap::new();
    log::debug!("Input issues to aggregate was: {:?}", issues);
    for issue in &issues {
        let aggregation_key = aggregation_key(&issue);
        let issue_coordinates = issue.attributes.coordinates.as_ref().unwrap();
        let aggregate = aggregate_results
            .entry(aggregation_key.clone())
            .or_insert_with(|| AggregateResult {
                issue_type: serde_json::to_string(&issue.attributes.r#type).unwrap(),
                title: issue.attributes.title.clone(),
                severity: serde_json::to_string(&issue.attributes.effective_severity_level)
                    .unwrap(),
                count: 0,
                ignored: issue.attributes.ignored,
                upgradeable: issue_coordinates.iter().any(|c| c.is_upgradeable.unwrap()),
                patchable: issue_coordinates.iter().any(|c| c.is_patchable.unwrap()),
            });
        aggregate.count += 1;
        log::debug!(
            "Added aggregation for issue {:?} with key {}",
            issue,
            aggregation_key.clone()
        );
    }
    let output: Vec<AggregateResult> = aggregate_results.values().cloned().collect();
    log::debug!("Output of aggregation was: {:?}", output);
    output
}

async fn get_organizations(arguments: &AppArguments) -> Vec<String> {
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: Some("Token".to_string()),
        key: arguments.snyk_api_token.clone(),
    });

    let mut all_organizations = Vec::new();
    let mut starting_after = None;

    loop {
        let organizations = openapi::apis::orgs_api::list_orgs(
            &configuration,
            "2024-04-22",
            starting_after.as_deref(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        all_organizations.extend(organizations.data.iter().map(|org| org.id.to_string()));

        let next = organizations.links.next;
        if next.is_none() {
            break;
        }
        let next = next.clone().unwrap();
        if let openapi::models::LinkProperty::String(next_url) = *next {
            let queries = querystring::querify(next_url.as_str());
            if let Some(entry) = queries.iter().find(|&&(key, _)| key == "starting_after") {
                starting_after = Some(entry.1.to_string());
            } else {
                break;
            }
        }
    }

    return all_organizations;
}

async fn get_projects(arguments: &AppArguments, organization_id: String) -> Vec<String> {
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: Some("Token".to_string()),
        key: arguments.snyk_api_token.clone(),
    });

    let mut all_projects = Vec::new();
    let mut starting_after = None;

    loop {
        let projects = openapi::apis::projects_api::list_org_projects(
            &configuration,
            organization_id.as_str(),
            "2024-04-22",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            starting_after.as_deref(),
            None,
            None,
        )
        .await
        .unwrap();

        all_projects.extend(
            projects
                .data
                .iter()
                .map(|projects| {
                    projects
                        .iter()
                        .map(|project| project.id.to_string())
                        .collect::<Vec<String>>()
                })
                .flatten()
                .collect::<Vec<String>>(),
        );

        let next = projects.links.next;
        if next.is_none() {
            break;
        }
        let next = next.clone().unwrap();
        if let openapi::models::LinkProperty::String(next_url) = *next {
            let queries = querystring::querify(next_url.as_str());
            if let Some(entry) = queries.iter().find(|&&(key, _)| key == "starting_after") {
                starting_after = Some(entry.1.to_string());
            } else {
                break;
            }
        }
    }

    return all_projects;
}

async fn get_issues(
    arguments: &AppArguments,
    organization_id: String,
    project_id: String,
) -> Vec<Issue> {
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: Some("Token".to_string()),
        key: arguments.snyk_api_token.clone(),
    });

    let mut all_issues = Vec::new();
    let mut starting_after = None;

    loop {
        let issues = openapi::apis::issues_api::list_org_issues(
            &configuration,
            "2024-04-22",
            organization_id.as_str(),
            starting_after.as_deref(),
            None,
            None,
            Some(project_id.as_str()),
            Some(ScanItemType::Project),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        all_issues.extend(issues.data.iter().map(|issue| issue.clone()));

        let next = issues.links.unwrap().next;
        if next.is_none() {
            break;
        }
        let next = next.clone().unwrap();
        if let openapi::models::LinkProperty::String(next_url) = *next {
            let queries = querystring::querify(next_url.as_str());
            if let Some(entry) = queries.iter().find(|&&(key, _)| key == "starting_after") {
                starting_after = Some(entry.1.to_string());
            } else {
                break;
            }
        }
    }

    return all_issues;
}
