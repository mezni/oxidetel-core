use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health/live", get(|| async { "ok" }))
        .route("/health/ready", get(|| async { "ok" }))
        .route("/products", get(|| async { "[]" }))
        .route("/offers", get(|| async { "[]" }))
        .route("/add-ons", get(|| async { "[]" }));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8006));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
