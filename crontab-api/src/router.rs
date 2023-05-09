use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{extract::Form, Router};
use serde::Deserialize;
use shuttle_runtime::tracing::info;

use crate::{AppState, Crontab, Job};

pub fn build_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/set-schedule", post(set_schedule))
        .with_state(app_state)
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}

pub async fn set_schedule(
    State(app_state): State<Arc<AppState>>,
    Form(job): Form<Job>,
) -> impl IntoResponse {
    info!("Setting new job: {:?}", job);

    let mut crontab = match app_state.persist.load::<Crontab>("crontab") {
        Ok(tab) => tab,
        Err(_) => Crontab { jobs: vec![] },
    };

    crontab.jobs.push(job);
    info!("Updating state: {:?}", &crontab);

    app_state.persist.save("crontab", crontab).unwrap();

    StatusCode::OK
}
