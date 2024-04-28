mod snyk;
use openapi::models::{Coordinate, Issue};

use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics, RequestTracing};
use async_std::task;
use clap::Parser;
use lazy_static::lazy_static;
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
            "snyk-exporter",
        )]))
        .build();
    global::set_meter_provider(provider);

    task::spawn(async move {
        loop {
            log::debug!("Polling Snyk API");
            poll_api(&arguments).await;
            task::sleep(Duration::from_secs(arguments.snyk_interval.try_into().unwrap())).await;
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
    let organizations = snyk::get_organizations(arguments).await;
    for organization_id in organizations.iter() {
		log::info!("Collecting for organization: {}", organization_id);
        let projects = snyk::get_projects(arguments, organization_id.to_string()).await;

        for project_id in projects.iter() {
            log::info!("Collecting for project: {}", project_id);
        
            let issues = snyk::get_issues(
                arguments,
                organization_id.to_string(),
                project_id.to_string(),
            )
            .await;
        
            log::info!("Recorded {} results for organization '{}'", issues.len(), organization_id);
        
            let results = aggregate_issues(issues);
            for result in results.iter() {
                SNYK_VULNERABILITIES_TOTAL
                .with_label_values(&[
                    &result.severity,
                    &result.issue_type,
                    &result.title,
                    &result.ignored.to_string(),
                    &result.upgradeable.to_string(),
                    &result.patchable.to_string(),
                    &result.count.to_string(),
                ])
                .set(result.count.into());
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
    let default_coordinates: Vec<Coordinate> = vec![];
    let issue_coordinates = issue.attributes.coordinates.as_ref().unwrap_or(&default_coordinates);
    log::debug!("Issue coordinates: {:?}", issue_coordinates);
    format!(
        "{}_{}_{}_{}_{}_{}",
        serde_json::to_string(&issue.attributes.effective_severity_level).unwrap(),
        serde_json::to_string(&issue.attributes.r#type).unwrap(),
        issue.attributes.title,
        issue.attributes.ignored,
        issue_coordinates.iter().any(|c| c.is_upgradeable.unwrap_or(false)),
        issue_coordinates.iter().any(|c| c.is_patchable.unwrap_or(false)),
    )
}

fn aggregate_issues(issues: Vec<Issue>) -> Vec<AggregateResult> {
    let mut aggregate_results: HashMap<String, AggregateResult> = HashMap::new();
    log::debug!("Input issues to aggregate was: {:?}", issues);
    for issue in &issues {
        let aggregation_key = aggregation_key(&issue);
        let default_coordinates: Vec<Coordinate> = vec![];
        let issue_coordinates = issue.attributes.coordinates.as_ref().unwrap_or(&default_coordinates);
        let aggregate = aggregate_results
            .entry(aggregation_key.clone())
            .or_insert_with(|| AggregateResult {
                issue_type: serde_json::to_string(&issue.attributes.r#type).unwrap(),
                title: issue.attributes.title.clone(),
                severity: serde_json::to_string(&issue.attributes.effective_severity_level)
                    .unwrap(),
                count: 0,
                ignored: issue.attributes.ignored,
                upgradeable: issue_coordinates.iter().any(|c| c.is_upgradeable.unwrap_or(false)),
                patchable: issue_coordinates.iter().any(|c| c.is_patchable.unwrap_or(false)),
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
