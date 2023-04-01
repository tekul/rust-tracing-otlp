use actix_web::{App, HttpServer, web, HttpResponse};
use std::io;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_simple()
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

#[tracing::instrument()]
async fn index() -> HttpResponse {
    info!("Handling index request");
    HttpResponse::Ok().body("Hello. You probably want to try the /rand endpoint.")
}

#[tracing::instrument()]
async fn get_random() -> HttpResponse {
    info!("Generating random number");
    let random_number = rand::random::<u64>();
    info!("Value is {random_number}");
    HttpResponse::Ok().body(format!("Hello. Your random number is {random_number}."))
}
