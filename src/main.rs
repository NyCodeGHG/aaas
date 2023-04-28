use std::io::Cursor;

use agg::Config;
use anyhow::Result;
use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, Server,
};
use tokio::signal::unix::SignalKind;
use tracing::info;

macro_rules! signal {
    ($signal:expr) => {
        async {
            let mut signal = ::tokio::signal::unix::signal($signal).unwrap();
            signal.recv().await;
        }
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let app = Router::new()
        .route("/render", post(render))
        .route("/", get(|| async { ":3" }));
    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap());
    info!("Listening on http://localhost:8080");
    server
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
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
        })
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn render(body: Bytes) -> Result<Response, StatusCode> {
    let gif = tokio::task::spawn_blocking(|| {
        let mut output = Vec::new();
        let input = Cursor::new(body);
        agg::run(input, &mut output, Config::default())?;
        Result::<Vec<u8>>::Ok(output)
    })
    .await
    .unwrap()
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, gif).into_response())
}
