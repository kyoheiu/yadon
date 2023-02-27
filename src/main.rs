use std::net::TcpListener;

use axum::{debug_handler, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let router = Router::new().route("/health", get(health)).layer(layer);

    axum::Server::from_tcp(TcpListener::bind("0.0.0.0:8080").expect("Failed to listen."))
        .expect("Failed to listen.")
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn health() -> String {
    "Hello, yadon.".to_string()
}
