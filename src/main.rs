use agg::Config;
use anyhow::Result;
use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    routing::{get, post},
    Router, Server,
};
use std::io::Cursor;
use tokio::signal::unix::SignalKind;
use tracing::info;

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
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn render(body: Bytes) -> Result<Response<Full<Bytes>>, StatusCode> {
    let gif = tokio::task::spawn_blocking(|| {
        let mut output = Vec::new();
        let input = Cursor::new(body);
        agg::run(input, &mut output, Config::default())?;
        Result::<Vec<u8>>::Ok(output)
    })
    .await
    .unwrap()
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "image/gif")
        .body(Full::new(Bytes::from(gif)))
        .unwrap())
}

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
