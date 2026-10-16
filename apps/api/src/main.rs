mod app;
mod domain;
mod http;

use std::{net::SocketAddr, sync::Arc};
use axum::{http::{header, HeaderValue, Method}, Router};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let state = Arc::new(app::AppState::default());
    let web_origin = std::env::var("WEB_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into()).parse::<HeaderValue>().expect("valid WEB_ORIGIN");
    let cors = CorsLayer::new().allow_origin(web_origin).allow_methods([Method::GET, Method::POST]).allow_headers([header::CONTENT_TYPE]);
    let router: Router = http::router(state).layer((TraceLayer::new_for_http(), cors));
    let addr: SocketAddr = std::env::var("API_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()).parse().expect("valid API_ADDR");
    let listener = TcpListener::bind(addr).await.expect("bind API_ADDR");
    tracing::info!(%addr, "KloudLab API listening");
    axum::serve(listener, router).await.expect("serve API");
}
