use lazy_static::lazy_static;
use serde_json::Value as JsonValue;
use std::{io, sync::Mutex};
use tokio::sync::broadcast;
use tracing::metadata::LevelFilter;
use tracing_subscriber::Layer;

lazy_static! {
    pub static ref LOG_CHANNEL: broadcast::Sender<JsonValue> = broadcast::channel(64).0;
}

pub struct Logger {
    tx: broadcast::Sender<JsonValue>,
}

#[allow(dead_code)]
impl Logger {
    pub fn new() -> Self {
        Self {
            tx: LOG_CHANNEL.clone(),
        }
    }

    pub fn make_layer() -> impl Layer<tracing_subscriber::Registry> {
        tracing_subscriber::fmt::layer()
            .pretty()
            .json()
            .with_writer(Mutex::new(Self::new()))
            .with_filter(LevelFilter::INFO)
    }
}

impl io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Ok(log) = serde_json::from_slice(buf) else {
            return Ok(0);
        };

        if let Ok(n) = self.tx.send(log) {
            return Ok(n);
        }

        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
