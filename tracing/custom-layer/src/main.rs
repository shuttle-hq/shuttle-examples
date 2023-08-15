use lazy_static::lazy_static;
use std::{net::SocketAddr, time::Duration};
use tokio::sync::broadcast;
use tracing::info;
use tracing_subscriber::fmt::MakeWriter;

lazy_static! {
    /// Global channel for sending logs
    pub static ref LOG_CHANNEL: broadcast::Sender<String> = broadcast::channel(64).0;
}

#[shuttle_runtime::main(tracing_layer = Logger::init)]
async fn init() -> Result<MyService, shuttle_runtime::Error> {
    Ok(MyService::new())
}

#[derive(Debug)]
struct MyService {
    pub logs: broadcast::Receiver<String>,
}

impl MyService {
    pub fn new() -> Self {
        Self {
            logs: LOG_CHANNEL.subscribe(),
        }
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(mut self, _addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // send some messages...
        for i in 0..10 {
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(i)).await;
                info!("Hello from thread #{i}!");
            });
        }

        while let Ok(message) = self.logs.recv().await {
            // do something with your logs!
            println!("Got a new log!");
            tokio::fs::write(Logger::LOG_FILE, message).await?;
        }

        Ok(())
    }
}

/// Logger struct passed to our custom tracing layer
#[derive(Debug)]
struct Logger {
    sender: broadcast::Sender<String>,
}

// Necessary for [`MakeWriter`] impl
impl std::io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let message = String::from_utf8_lossy(buf);

        if let Err(err) = self.sender.send(message.to_string()) {
            eprintln!("Failed to send message: {err:?}");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// Necessary for passing [`Logger`] as a custom writer to [`tracing_subscriber`]
impl MakeWriter<'_> for Logger {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        Self {
            sender: LOG_CHANNEL.clone(),
        }
    }
}

impl Logger {
    const LOG_FILE: &str = "/var/log/my-service.log";

    pub fn init() -> impl tracing_subscriber::Layer<shuttle_runtime::Registry> {
        let logger = Self {
            sender: LOG_CHANNEL.clone(),
        };

        tracing_subscriber::fmt::layer()
            .without_time()
            .with_writer(logger)
    }
}
