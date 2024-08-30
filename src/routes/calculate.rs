use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

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
}

pub async fn calculate(params: Query<CalculateRequest>) -> Json<CalculateResponse> {
    let request = CalculateRequest {
        operation: params.0.operation,
        value_1: params.0.value_1,
        value_2: params.0.value_2,
    };

    Json(CalculateResponse {
        operation: request.operation,
        value_1: request.value_1,
        value_2: request.value_2,
        result: request.value_1 + request.value_2,
    })
}
