use opentelemetry::KeyValue;
use opentelemetry_sdk::{trace as sdktrace, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing(service_name: &str) {
    // JSON logs with RFC3339 timestamps
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339());

    // Base registry (logs only)
    let mut registry = tracing_subscriber::registry().with(env_filter).with(fmt_layer);

    // If you compile with `--features otel-otlp`, add an OTLP span layer too.
    #[cfg(feature = "otel-otlp")]
    {
        if let Some(layer) = build_otel_layer(service_name) {
            registry = registry.with(layer);
        }
    }

    registry.init();
}

#[cfg(feature = "otel-otlp")]
fn build_otel_layer<S>(service_name: &str) -> Option<tracing_opentelemetry::OpenTelemetryLayer<S, sdktrace::Tracer>>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    use opentelemetry_otlp::Protocol;

    // Read endpoint/headers from env (works for BetterStack or any OTLP endpoint)
    let endpoint =
        std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4318".into());

    // Headers in the standard comma-separated form: k1=v1,k2=v2
    let headers = std::env::var("OTEL_EXPORTER_OTLP_HEADERS").ok();
    let headers_map = parse_headers(headers.as_deref());

    // Resource (service name etc.)
    let resource = Resource::builder()
        .with_service_name(service_name.to_string())
        .with_attributes(vec![
            KeyValue::new("telemetry.sdk.language", "rust"),
        ])
        .build();

    // Build an OTLP/HTTP span exporter (0.30 API)
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .with_protocol(Protocol::HttpBinary) // or HttpProtobuf; works with most collectors
        .with_headers(headers_map)
        .build()
        .ok()?;

    // Tracer provider with batch exporter on Tokio
    let provider = sdktrace::TracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build();

    let tracer = provider.tracer("polars-otel-shuttle");

    // Install as global provider
    opentelemetry::global::set_tracer_provider(provider);

    // Bridge tracing -> OpenTelemetry
    Some(tracing_opentelemetry::layer().with_tracer(tracer))
}

#[cfg(feature = "otel-otlp")]
fn parse_headers(input: Option<&str>) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Some(s) = input {
        for kv in s.split(',') {
            if let Some((k, v)) = kv.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    map
}

/// Call this on shutdown if you want to block until spans are exported.
pub fn shutdown_tracing() {
    // No-op for opentelemetry 0.30.
   // The tracer provider will flush on drop when the process exits.
}

