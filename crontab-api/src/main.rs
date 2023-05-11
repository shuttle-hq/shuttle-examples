use axum::{response::IntoResponse, routing::get, Router};
use shuttle_crontab::{CrontabService, ShuttleCrontab};
use shuttle_persist::{Persist, PersistInstance};

async fn hello_crontab() -> impl IntoResponse {
    "Hello there, try making a POST request to '/crontab/set' to create a new job.".to_string()
}

async fn trigger_me() -> impl IntoResponse {
    "Triggered by the crontab service".to_string()
}

#[shuttle_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> ShuttleCrontab {
    let router = Router::new()
        .route("/", get(hello_crontab))
        .route("/trigger-me", get(trigger_me));
    CrontabService::new(persist, router)
}
