use actix_web::{web, App, HttpResponse, HttpServer};
use opentelemetry_otlp::WithExportConfig;
use std::{io, str::FromStr};
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let mut exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_metadata(load_metadata_from_env())
        .with_env();

    // Check if we need TLS
    if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        if endpoint.starts_with("https") {
            exporter = exporter.with_tls_config(Default::default());
        }
    }

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

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

fn load_metadata_from_env() -> tonic::metadata::MetadataMap {
    use tonic::metadata;

    let mut metadata = metadata::MetadataMap::new();
    if let Ok(headers) = std::env::var("OTEL_EXPORTER_OTLP_HEADERS") {
        headers
            .split(',')
            .map(|header| {
                header
                    .split_once('=')
                    .expect("Header should contain '=' character")
            })
            .map(|(name, value)| {
                (
                    name,
                    value
                        .parse::<metadata::MetadataValue<metadata::Ascii>>()
                        .expect("Header value invalid"),
                )
            })
            .for_each(|(name, value)| {
                metadata.insert(metadata::MetadataKey::from_str(name).unwrap(), value);
            });
    }
    metadata
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
