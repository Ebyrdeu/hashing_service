use std::error::Error;

use opentelemetry::global::set_tracer_provider;
use opentelemetry::KeyValue;
use opentelemetry::runtime::AsyncStd;
use opentelemetry::sdk::export::trace::stdout::PipelineBuilder;

pub async fn init_tracer() -> Result<(), Box<dyn Error>> {
    let tracer = PipelineBuilder::new()
        .with_service_name("your_service_name")
        .with_tags(vec![
            KeyValue::new("exporter", "jaeger"),
            KeyValue::new("env", "dev"),
        ])
        .install_batch(AsyncStd)?;

    set_tracer_provider(tracer);

    Ok(())
}
