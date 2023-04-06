Sample application to accompany the article [Flexible Tracing with Rust and OpenTelemetry](https://broch.tech/posts/rust-tracing-opentelemetry).

## TLDR

### Jaeger

```
$ OTEL_SERVICE_NAME=randy OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317" RUST_LOG="debug,h2=warn" cargo run
```

### Honeycomb

With grpc (the default):

```
$ OTEL_EXPORTER_OTLP_ENDPOINT=https://api.honeycomb.io:443 OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=your-api-key" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```

With http/protobuf:

```
OTEL_EXPORTER_OTLP_ENDPOINT=https://api.honeycomb.io/v1/traces OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=your-api-key" OTEL_EXPORTER_OTLP="http/protobuf" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```

### Aspecto

```
OTEL_EXPORTER_OTLP_ENDPOINT=https://otelcol.aspecto.io:4317 OTEL_EXPORTER_OTLP_HEADERS="Authorization=your-api-key" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```
