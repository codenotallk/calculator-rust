use axum::{extract::Query, Json};
use libs::{common::time::{date_formatted, get_epoch_from_formatted}, domain::operation::Operation, repository::{repository, repository::Interval}};
use serde::{Deserialize, Serialize};


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

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    offset: Option<u32>,
    from: Option<String>,
    to: Option<String>,
}

impl TryInto <Interval> for ReportRequest {
    type Error = &'static str;

    fn try_into(self) -> Result<Interval, Self::Error> {
        
        let from = if let Some (from) = &self.from {
            match get_epoch_from_formatted (from) {
                Ok(date) => Some (date),
                Err(err) => return Err(err),
            }
        } else {
            None
        };

        let to = if let Some (to) = &self.to {
            match get_epoch_from_formatted (to) {
                Ok(date) => Some (date),
                Err(err) => return Err(err),
            }
        } else {
            None
        };

        Ok(Interval::new(self.offset, from, to))
    }
}

pub async fn report(filter: Query<ReportRequest>) -> Result<Json<Vec<ReportResponse>>, Json<&'static str>> {

    let interval: Result<Interval, _> = filter.0.try_into();

    match interval {
        Ok(interval) => {
            let operations = repository::get(interval).await;

            let reponses: Vec<ReportResponse> = operations
                .into_iter()
                .map(|operation| ReportResponse::from(operation))
                .collect();

            Ok(Json(reponses))
        },
        Err(err) => Err(Json(err)),
    }
}
