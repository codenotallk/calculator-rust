use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::Duration,
};

use axum::{routing::get, serve, Router};
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};
use libs::{common::config::get_config, domain::operation::Operation};
use routes::{
    calculate::{calculate, AppState, CalculateResponse},
    health::health,
};
use tokio::net::TcpListener;

pub mod routes;

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<Operation>, Receiver<Operation>) = mpsc::channel();

    let state = Arc::new(AppState::new(tx));

    let router = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/calculate", get(calculate))
        .with_state(state);

    tokio::spawn(async move {
        produce_message(rx).await;
    });

    let listener = TcpListener::bind(format!("0.0.0.0:{}", get_config().unwrap().calculate.port))
        .await
        .unwrap();

    serve(listener, router).await.unwrap();
}

async fn produce_message(receiver: Receiver<Operation>) {
    let config = get_config().unwrap();

    let mut producer = Producer::from_hosts(config.broker.hosts)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(operation) => {
                let message = serde_json::to_string(&CalculateResponse::from(operation)).unwrap();
                producer
                    .send(&Record::from_value(&config.calculate.topic, message))
                    .unwrap();
            }
            Err(_) => todo!(),
        }
    }
}
