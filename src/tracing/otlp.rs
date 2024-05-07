use std::env;

use opentelemetry::KeyValue;
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

pub fn init_oltp() -> Result<Tracer, TraceError> {
    let endpoint = env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());

    let export_config = ExportConfig {
        endpoint,
        protocol: Protocol::Grpc,
        timeout: Default::default(),
    };

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_export_config(export_config);

    let trace_config = trace::config().with_resource(Resource::new(vec![
        KeyValue::new(SERVICE_NAME, "SALT")
    ]));

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace_config)
        .install_simple()
}