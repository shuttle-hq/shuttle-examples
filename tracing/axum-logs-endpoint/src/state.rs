use crate::logger::LOG_CHANNEL;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct AppState {
    logs: broadcast::Sender<JsonValue>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            logs: LOG_CHANNEL.clone(),
        })
    }

    pub fn sub(&self) -> broadcast::Receiver<JsonValue> {
        self.logs.subscribe()
    }
}
