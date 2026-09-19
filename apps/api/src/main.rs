mod app;
mod domain;
mod http;
mod providers;
#[cfg(test)] mod domain_tests;

use std::{net::SocketAddr, sync::Arc};
use axum::Router;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let state = Arc::new(app::AppState::new());
    let router: Router = http::router(state).layer((TraceLayer::new_for_http(), CorsLayer::permissive()));
    let addr: SocketAddr = std::env::var("API_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()).parse().expect("valid API_ADDR");
    let listener = TcpListener::bind(addr).await.expect("bind API_ADDR");
    tracing::info!(%addr, "KloudLab API listening");
    axum::serve(listener, router).await.expect("serve API");
}
