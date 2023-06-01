use axum::{response::IntoResponse, routing::get, Router};
use shuttle_persist::{Persist, PersistInstance};

use request_scheduler::{CrontabService, ShuttleCrontab};

async fn hello_crontab() -> impl IntoResponse {
    "Hello there, try making a POST request to '/crontab/set' to create a new job.".to_string()
}

async fn trigger_me() -> impl IntoResponse {
    "Triggered by the crontab service".to_string()
}

#[shuttle_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> ShuttleCrontab {
    // A userland router, so to speak. Will be the primary router mounted at "/",
    // while the `CrontabService` has its own router mounted at "/crontab", as
    // defined in `router.rs`.
    // See lib.rs for more documentation.
    let user_router = Router::new()
        .route("/", get(hello_crontab))
        .route("/trigger-me", get(trigger_me));

    CrontabService::new(persist, user_router)
}
