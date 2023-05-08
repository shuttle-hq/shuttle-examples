use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{extract::Form, Router};
use serde::Deserialize;
use shuttle_runtime::tracing::info;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/set-schedule", post(set_schedule))
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    schedule: String,
    url: String,
}

pub async fn set_schedule(Form(schedule): Form<Schedule>) -> impl IntoResponse {
    info!("Setting schedule: {:?}", schedule);

    StatusCode::OK
}
