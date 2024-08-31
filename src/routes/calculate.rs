use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::operation::Operation, repository::repository};

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

impl TryInto<Operation> for CalculateRequest {
    type Error = &'static str;

    fn try_into(self) -> Result<Operation, Self::Error> {
        Operation::new(self.operation, self.value_1, self.value_2)
    }
}

pub async fn calculate(
    params: Query<CalculateRequest>,
) -> Result<Json<CalculateResponse>, Json<&'static str>> {
    let request = CalculateRequest {
        operation: params.0.operation,
        value_1: params.0.value_1,
        value_2: params.0.value_2,
    };

    match request.try_into() as Result<Operation, _> {
        Ok(operation) => {

            repository::save (operation.clone());

            Ok(Json(CalculateResponse {
                operation: operation.name(),
                value_1: operation.value_1(),
                value_2: operation.value_2(),
                result: operation.result(),
            }))
        },
        Err(err) => Err(Json(err)),
    }
}
