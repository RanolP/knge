use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,axum=info".to_string()),
        )
        .init();

    let app = Router::new().route("/healthz", get(healthz));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(%addr, "starting api server");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("api server exited unexpectedly");
}

async fn healthz() -> &'static str {
    "ok"
}
