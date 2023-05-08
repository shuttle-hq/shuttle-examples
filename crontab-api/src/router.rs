use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{extract::Form, Router};
use serde::Deserialize;
use shuttle_runtime::tracing::info;

use crate::AppState;

pub fn build_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/set-schedule", post(set_schedule))
        .with_state(app_state)
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    schedule: String,
    url: String,
}

pub async fn set_schedule(
    State(app_state): State<Arc<AppState>>,
    Form(schedule): Form<Schedule>,
) -> impl IntoResponse {
    info!("Setting schedule: {:?}", schedule);

    app_state.persist.save("some", "key");

    let val: String = app_state.persist.load("some").unwrap();
    info!("Got state: {:?}", val);

    StatusCode::OK
}
