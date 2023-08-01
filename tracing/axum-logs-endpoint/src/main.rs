use std::{sync::Arc, time::SystemTime};

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use axum_error::Result;
use serde_json::Value as JsonValue;
use shuttle_axum::ShuttleAxum;
use tracing::info;

mod logger;
use logger::Logger;

mod state;
use state::AppState;

#[shuttle_runtime::main(tracing_layer = Logger::make_layer)]
async fn axum_logs() -> ShuttleAxum {
    let state = AppState::new();
    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/message/:message", post(send_message))
        .route("/logs/:amount", get(get_logs))
        .with_state(state);

    Ok(router.into())
}

async fn send_message(Path(message): Path<String>) -> String {
    info!(?message, now = ?SystemTime::now());
    message
}

async fn get_logs(
    Path(amount): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<JsonValue>>> {
    let mut rx = state.sub();
    let mut logs = Vec::new();

    while let Ok(log) = rx.recv().await {
        logs.push(log);

        if logs.len() == amount {
            break;
        }
    }

    Ok(Json(logs))
}
