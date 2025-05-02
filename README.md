Sample application to accompany the article [Flexible Tracing with Rust and OpenTelemetry](https://broch.tech/posts/rust-tracing-opentelemetry).

## TLDR

### Jaeger

```
$ OTEL_SERVICE_NAME=randy OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317" RUST_LOG="debug,h2=warn" cargo run
```

### Honeycomb

The endpoints here are for EU registered accounts. If you are in the US, use `api.honeycomb.io` instead.

With grpc (the default):

```
$ OTEL_EXPORTER_OTLP_ENDPOINT=https://api.eu1.honeycomb.io OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=your-api-key" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```

With http/protobuf:

```
OTEL_EXPORTER_OTLP_ENDPOINT=https://api.eu1.honeycomb.io OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=your-api-key" OTEL_EXPORTER_OTLP="http/protobuf" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```

### Aspecto

```
OTEL_EXPORTER_OTLP_ENDPOINT=https://otelcol.aspecto.io:4317 OTEL_EXPORTER_OTLP_HEADERS="Authorization=your-api-key" OTEL_SERVICE_NAME=randy RUST_LOG="debug,h2=warn" cargo run
```
