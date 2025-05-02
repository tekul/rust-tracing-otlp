use actix_web::{web, App, HttpResponse, HttpServer};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithTonicConfig;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let telemetry_layer = match create_otlp_tracer_provider() {
        Some(tracer_provider) => {
            // The name provided here ends up as the attribute 'library.name' in traces
            let tracer = tracer_provider.tracer("rust-otel-otlp");
            Some(tracing_opentelemetry::OpenTelemetryLayer::new(tracer))
        }
        None => None,
    };

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

fn create_otlp_tracer_provider() -> Option<opentelemetry_sdk::trace::SdkTracerProvider> {
    if !std::env::vars().any(|(name, _)| name.starts_with("OTEL_")) {
        return None;
    }
    let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or("grpc".to_string());

    let exporter = match protocol.as_str() {
        "grpc" => {
            let mut exporter = opentelemetry_otlp::SpanExporter::builder().with_tonic();

            // Check if we need TLS
            if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
                if endpoint.starts_with("https") {
                    exporter = exporter.with_tls_config(tonic::transport::ClientTlsConfig::default().with_enabled_roots());
                }
            }
            exporter.build().expect("Failed to create tonic exporter")
        }
        "http/protobuf" => opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()
            .expect("Failed to create http/protobuf exporter"),
        p => panic!("Unsupported protocol {}", p),
    };

    Some(opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .build())
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
