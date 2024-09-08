use std::sync::{mpsc::Sender, Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use libs::{common::time::date_formatted, domain::operation::Operation};
use serde::{Deserialize, Serialize};

pub struct AppState {
    sender: Sender<Operation>,
}

impl AppState {
    pub fn new(sender: Sender<Operation>) -> Self {
        Self { sender }
    }
}

#[derive(Deserialize)]
pub struct CalculateRequest {
    operation: String,
    value_1: u32,
    value_2: u32,
}

#[derive(Serialize)]
pub struct CalculateResponse {
    operation: String,
    value_1: u32,
    value_2: u32,
    result: u32,
    create_at: String,
}

impl TryInto<Operation> for CalculateRequest {
    type Error = &'static str;

    fn try_into(self) -> Result<Operation, Self::Error> {
        Operation::new(self.operation, self.value_1, self.value_2)
    }
}

impl From<Operation> for CalculateResponse {
    fn from(operation: Operation) -> Self {
        CalculateResponse {
            operation: operation.name(),
            value_1: operation.value_1(),
            value_2: operation.value_2(),
            result: operation.result(),
            create_at: date_formatted(operation.create_at() as i64),
        }
    }
}

pub async fn calculate(
    State(state): State<Arc<AppState>>,
    params: Query<CalculateRequest>,
) -> Result<Json<CalculateResponse>, Json<&'static str>> {
    let request = CalculateRequest {
        operation: params.0.operation,
        value_1: params.0.value_1,
        value_2: params.0.value_2,
    };

    match request.try_into() as Result<Operation, _> {
        Ok(operation) => {
            let _ = state.sender.send(operation.clone());

            Ok(Json(CalculateResponse::from(operation)))
        }
        Err(err) => Err(Json(err)),
    }
}
