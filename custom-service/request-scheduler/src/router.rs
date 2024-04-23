use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{extract::Form, Router};
use tokio::sync::oneshot;
use tracing::debug;

use crate::{CrontabServiceState, Msg, RawJob};

pub(crate) fn make_router(cron_state: Arc<CrontabServiceState>) -> Router {
    Router::new()
        .route("/set", post(set_schedule))
        .with_state(cron_state)
}

async fn set_schedule(
    State(state): State<Arc<CrontabServiceState>>,
    Form(job): Form<RawJob>,
) -> impl IntoResponse {
    debug!("Accepted new job: {:?}", job);
    let (tx, rx) = oneshot::channel();

    state.sender.send(Msg::NewJob(job, tx)).await.unwrap();

    rx.await.expect("Channel transmission failed")
}
