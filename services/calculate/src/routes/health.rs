use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    status: String,
    message: String,
}

pub async fn health() -> Json<Health> {
    Json(Health {
        status: "up".to_owned(),
        message: "Calculate Service".to_owned(),
    })
}
