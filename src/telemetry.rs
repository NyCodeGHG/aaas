use anyhow::Result;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_otlp::{HasExportConfig, Protocol, WithExportConfig};
use tracing::Level;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn configure_telemetry(enable_otlp: bool) -> Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    if enable_otlp {
        let mut exporter = opentelemetry_otlp::new_exporter().tonic().with_env();
        let config = {
            let cfg = exporter.export_config();
            ExporterConfig {
                endpoint: cfg.endpoint.clone(),
                protocol: cfg.protocol,
            }
        };
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .install_batch(opentelemetry::runtime::Tokio)?;
        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter)
            .with(telemetry_layer)
            .init();
        tracing::info!(endpoint = config.endpoint, protocol = ?config.protocol, "OpenTelemetry is enabled.");
    } else {
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter)
            .init();
        tracing::warn!("OpenTelemetry is not enabled.");
    };
    Ok(())
}

struct ExporterConfig {
    endpoint: String,
    protocol: Protocol,
}
