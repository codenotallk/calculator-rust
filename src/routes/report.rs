use axum::Json;
use serde::Serialize;

use crate::{common::time::date_formatted, domain::operation::Operation, repository::repository};

#[derive(Serialize)]
pub struct ReportResponse {
    operation: String,
    value_1: u32,
    value_2: u32,
    result: u32,
    create_at: String,
}

impl From<Operation> for ReportResponse {
    fn from(operation: Operation) -> Self {
        Self {
            operation: operation.name(),
            value_1: operation.value_1(),
            value_2: operation.value_2(),
            result: operation.result(),
            create_at: date_formatted(operation.create_at() as i64),
        }
    }
}

pub async fn report() -> Json<Vec<ReportResponse>> {
    let operations = repository::get();

    let reponses: Vec<ReportResponse> = operations
        .into_iter()
        .map(|operation| ReportResponse::from(operation))
        .collect();

    Json(reponses)
}
