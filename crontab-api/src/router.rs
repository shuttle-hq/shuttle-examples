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
pub struct ScheduleRaw {
    schedule: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    schedule: String,
    url: String,
}

impl From<ScheduleRaw> for Schedule {
    fn from(ScheduleRaw { schedule, url }: ScheduleRaw) -> Self {
        // TODO: Parse & verify
        Self { schedule, url }
    }
}

pub async fn set_schedule(Form(schedule_raw): Form<ScheduleRaw>) -> impl IntoResponse {
    let schedule: Schedule = schedule_raw.into();
    info!("Setting schedule: {:?}", schedule);

    StatusCode::OK
}
