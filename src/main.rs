use actix_web::{web, App, HttpResponse, HttpServer};
use std::{io, str::FromStr};
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let telemetry_layer =
        create_otlp_tracer().map(|t| tracing_opentelemetry::layer().with_tracer(t));

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();

    info!("Starting server");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(web::resource("/").to(index))
            .route("/rand", web::get().to(get_random))
    })
    .bind("127.0.0.1:8080")
    .unwrap();

    server.workers(2).run().await
}

fn create_otlp_tracer() -> Option<opentelemetry_sdk::trace::Tracer> {
    if !std::env::vars().any(|(name, _)| name.starts_with("OTEL_")) {
        return None;
    }
    let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or("grpc".to_string());

    let tracer = opentelemetry_otlp::new_pipeline().tracing();
    let headers = parse_otlp_headers_from_env();

    let tracer = match protocol.as_str() {
        "grpc" => {
            let mut exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata_from_headers(headers));

            // Check if we need TLS
            if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
                if endpoint.starts_with("https") {
                    exporter = exporter.with_tls_config(Default::default());
                }
            }
            tracer.with_exporter(exporter)
        }
        "http/protobuf" => {
            let exporter = opentelemetry_otlp::new_exporter()
                .http()
                .with_headers(headers.into_iter().collect());
            tracer.with_exporter(exporter)
        }
        p => panic!("Unsupported protocol {}", p),
    };

    Some(
        tracer
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap(),
    )
}

fn metadata_from_headers(headers: Vec<(String, String)>) -> tonic::metadata::MetadataMap {
    use tonic::metadata;

    let mut metadata = metadata::MetadataMap::new();
    headers.into_iter().for_each(|(name, value)| {
        let value = value
            .parse::<metadata::MetadataValue<metadata::Ascii>>()
            .expect("Header value invalid");
        metadata.insert(metadata::MetadataKey::from_str(&name).unwrap(), value);
    });
    metadata
}

// Support for this has now been merged into opentelemetry-otlp so check next release after 0.14
fn parse_otlp_headers_from_env() -> Vec<(String, String)> {
    let mut headers = Vec::new();

    if let Ok(hdrs) = std::env::var("OTEL_EXPORTER_OTLP_HEADERS") {
        hdrs.split(',')
            .map(|header| {
                header
                    .split_once('=')
                    .expect("Header should contain '=' character")
            })
            .for_each(|(name, value)| headers.push((name.to_owned(), value.to_owned())));
    }
    headers
}

#[tracing::instrument()]
async fn index() -> HttpResponse {
    debug!("Handling index request");
    HttpResponse::Ok().body("Hello. You probably want to try the /rand endpoint.")
}

#[tracing::instrument()]
async fn get_random() -> HttpResponse {
    debug!("Generating random number");
    let random_number = rand::random::<u64>();
    debug!("Value is {random_number}");
    HttpResponse::Ok().body(format!("Hello. Your random number is {random_number}."))
}
