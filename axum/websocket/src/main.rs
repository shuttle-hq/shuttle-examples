use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use shuttle_axum::ShuttleAxum;
use tokio::{
    sync::{watch, Mutex},
    time::sleep,
};
use tower_http::services::ServeDir;

struct State {
    clients_count: usize,
    rx: watch::Receiver<Message>,
}

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.dev/.healthz";

#[derive(Serialize)]
struct Response {
    clients_count: usize,
    #[serde(rename = "dateTime")]
    date_time: DateTime<Utc>,
    is_up: bool,
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let (tx, rx) = watch::channel(Message::Text("{}".into()));

    let state = Arc::new(Mutex::new(State {
        clients_count: 0,
        rx,
    }));

    // Spawn a thread to continually check the status of the api
    let state_send = state.clone();
    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);

        loop {
            let is_up = reqwest::get(STATUS_URI)
                .await
                .is_ok_and(|r| r.status().is_success());

            let response = Response {
                clients_count: state_send.lock().await.clients_count,
                date_time: Utc::now(),
                is_up,
            };
            let msg = serde_json::to_string(&response).unwrap();

            if tx.send(Message::Text(msg.into())).is_err() {
                break;
            }

            sleep(duration).await;
        }
    });

    let router = Router::new()
        .route("/websocket", get(websocket_handler))
        .fallback_service(ServeDir::new("static"))
        .layer(Extension(state));

    Ok(router.into())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<State>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<Mutex<State>>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let mut rx = {
        let mut state = state.lock().await;
        state.clients_count += 1;
        state.rx.clone()
    };

    // This task will receive watch messages and forward it to this connected client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            let msg = rx.borrow().clone();

            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // This task will receive messages from this client.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            println!("this example does not read any messages, but got: {text}");
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // This client disconnected
    state.lock().await.clients_count -= 1;
}
