use anyhow::Result;
use opentelemetry_otlp::{HasExportConfig, Protocol, WithExportConfig};
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn configure_telemetry(enable_otlp: bool) -> Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(EnvFilter::from_default_env());

    if enable_otlp {
        let mut exporter = opentelemetry_otlp::new_exporter().tonic().with_env();
        let config = {
            let cfg = exporter.export_config();
            ExporterConfig {
                endpoint: cfg.endpoint.clone(),
                protocol: cfg.protocol,
            }
        };
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .install_batch(opentelemetry::runtime::Tokio)?;
        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(telemetry_layer)
            .init();
        tracing::info!(endpoint = config.endpoint, protocol = ?config.protocol, "OpenTelemetry is enabled.");
    } else {
        tracing_subscriber::registry().with(fmt_layer).init();
    };
    Ok(())
}

struct ExporterConfig {
    endpoint: String,
    protocol: Protocol,
}
