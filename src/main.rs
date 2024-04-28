use std::time::Duration;
use async_std::task;
use clap::Parser;
use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics, RequestTracing};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};
use lazy_static::lazy_static;
// use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry};
use prometheus::{IntCounterVec, Opts, Registry};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref SNYK_VULNERABILITY_INFO: IntCounterVec = IntCounterVec::new(
        Opts::new("snyk_vulnerability_info", "Snyk vulnerability info"),
        &["severity"],
    ).unwrap();
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
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
    snyk_organizations: String,
    /// Timeout for requests against Snyk API
    #[arg(long = "snyk.timeout", default_value = "10")]
	request_timeout: i32,
    /// Address on which to expose metrics
    #[arg(long = "web.listen-address", default_value = ":8080")]
	listen_address: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_arguments_file = "AppArguments.json";

    let arguments: AppArguments = if std::path::Path::new(app_arguments_file).exists() {
        let file_content = std::fs::read_to_string(app_arguments_file)
            .expect("Failed to read the file");
        serde_json::from_str(&file_content)
            .expect("JSON was not well-formatted")
    } else {
        AppArguments::parse()
    };
    
    REGISTRY.register(Box::new(SNYK_VULNERABILITY_INFO.clone())).unwrap();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(REGISTRY.clone())
        .build()?;

    // set up your meter provider with your exporter(s)
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .with_resource(Resource::new([KeyValue::new("service.name", "tenable_exporter")]))
        .build();
    global::set_meter_provider(provider);

    task::spawn(async move {
        loop {
            fetch_data(arguments.clone()).await;
            task::sleep(Duration::from_secs(300)).await;
        }
    });

    let _ = HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .route("/metrics", web::get().to(PrometheusMetricsHandler::new(REGISTRY.clone())))
        })
        .bind("localhost:8080")?
        .run()
        .await;

    Ok(())
}

pub async fn fetch_data(arguments: AppArguments) {
    let mut configuration = openapi::apis::configuration::Configuration::default();
    configuration.api_key = Some(openapi::apis::configuration::ApiKey {
        prefix: None,
        // Configuration should be like: --header 'Token xxx'
        key: format!("authorization={}", format!("Token {}", arguments.snyk_api_token)),
    });

}
