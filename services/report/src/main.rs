use axum::{routing::get, serve, Router};
use routes::{health::health, report::report};
use tokio::net::TcpListener;

pub mod routes;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/report", get(report));

    let listener = TcpListener::bind("0.0.0.0:1235").await.unwrap();

    serve(listener, router).await.unwrap();
}
