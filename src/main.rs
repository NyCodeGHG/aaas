use agg::Config;
use anyhow::Result;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    routing::{get, post},
    Json, Router, Server,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use serde::Serialize;
use std::{env, io::Cursor};
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
use tracing::{debug_span, info, Instrument};

use crate::telemetry::configure_telemetry;

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    configure_telemetry(
        env::var("ENABLE_OTLP")
            .ok()
            .and_then(|var| var.parse().ok())
            .unwrap_or(false),
    )?;
    let app = Router::new()
        .route("/render", post(render))
        .route("/", get(index))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default());
    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap());
    info!("Listening on http://localhost:8080");
    server
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_hook())
        .await?;
    Ok(())
}

#[tracing::instrument()]
async fn index() -> &'static str {
    ":3"
}

#[axum::debug_handler]
#[tracing::instrument(skip(body))]
async fn render(body: Bytes) -> Result<Response<Full<Bytes>>, (StatusCode, Json<ErrorResponse>)> {
    let gif = tokio::task::spawn_blocking(|| {
        let mut output = Vec::new();
        let input = Cursor::new(body);
        agg::run(
            input,
            &mut output,
            Config {
                show_progress_bar: false,
                ..Default::default()
            },
        )?;
        Result::<Vec<u8>>::Ok(output)
    })
    .instrument(debug_span!("gif"))
    .await
    .unwrap()
    .map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: error.to_string(),
            }),
        )
    })?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "image/gif")
        .body(Full::new(Bytes::from(gif)))
        .unwrap())
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[allow(unused)]
macro_rules! signal {
    ($signal:expr) => {
        async {
            let mut signal = ::tokio::signal::unix::signal($signal).unwrap();
            signal.recv().await;
        }
    };
}

async fn shutdown_hook() {
    #[cfg(unix)]
    tokio::select! {
        _ = signal!(SignalKind::interrupt()) => {
            info!("Received SIGINT. Shutting down.");
        },
        _ = signal!(SignalKind::terminate()) => {
            info!("Received SIGTERM. Shutting down.");
        },
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await.unwrap()
}
