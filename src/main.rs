use axum::{routing::get, serve, Router};
use routes::{calculate::calculate, health::health};
use tokio::net::TcpListener;

pub mod routes;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/calculate", get(calculate));

    let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();

    serve(listener, router).await.unwrap();
}
