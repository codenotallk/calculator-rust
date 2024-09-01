use axum::{routing::get, serve, Router};
use routes::{calculate::calculate, health::health, report::report};
use tokio::net::TcpListener;

pub mod common;
pub mod domain;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/calculate", get(calculate))
        .route("/v1/report", get(report));

    let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();

    serve(listener, router).await.unwrap();
}
