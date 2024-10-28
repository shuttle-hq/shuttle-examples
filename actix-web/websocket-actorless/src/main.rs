use actix_files::NamedFile;
use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use actix_ws::Message;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::Serialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};
use tokio::sync::{mpsc, watch};

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.dev";

type AppState = (
    mpsc::UnboundedSender<WsState>,
    watch::Receiver<ApiStateMessage>,
);

#[derive(Debug, Clone)]
enum WsState {
    Connected,
    Disconnected,
}

#[derive(Serialize, Default, Clone, Debug)]
struct ApiStateMessage {
    client_count: usize,
    origin: String,
    date_time: DateTime<Utc>,
    is_up: bool,
}

async fn echo_handler(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    tx: mpsc::UnboundedSender<WsState>,
) {
    while let Some(Ok(msg)) = msg_stream.next().await {
        match msg {
            Message::Ping(bytes) => {
                if session.pong(&bytes).await.is_err() {
                    return;
                }
            }
            Message::Text(s) => {
                session.text(s.clone()).await.unwrap();
                tracing::info!("Got text, {}", s);
            }
            _ => break,
        }
    }

    if let Err(e) = tx.send(WsState::Disconnected) {
        tracing::error!("Failed to send disconnected state: {e:?}");
    }

    let _ = session.close(None).await;
}

async fn websocket(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let app_state = app_state.into_inner();
    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;

    let tx_ws_state = app_state.0.clone();
    let tx_ws_state2 = tx_ws_state.clone();

    // send connected state
    if let Err(e) = tx_ws_state.send(WsState::Connected) {
        tracing::error!("Failed to send connected state: {e:?}");
    }

    // listen for api state changes
    let mut session_clone = session.clone();
    let mut rx_api_state = app_state.1.clone();
    actix_web::rt::spawn(async move {
        // adding some delay to avoid getting the first message too soon.
        tokio::time::sleep(Duration::from_millis(500)).await;
        while rx_api_state.changed().await.is_ok() {
            let msg = rx_api_state.borrow().clone();
            tracing::info!("Handling ApiStateMessage: {msg:?}");
            let msg = serde_json::to_string(&msg).unwrap();
            session_clone.text(msg).await.unwrap();
        }
    });

    // echo handler
    actix_web::rt::spawn(echo_handler(session, msg_stream, tx_ws_state2));
    Ok(response)
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // We're going to use channels to communicate between threads.
    // api state channel
    let (tx_api_state, rx_api_state) = watch::channel(ApiStateMessage::default());
    // websocket state channel
    let (tx_ws_state, mut rx_ws_state) = mpsc::unbounded_channel::<WsState>();

    // create a shared state for the client counter
    let client_count = Arc::new(AtomicUsize::new(0));
    let client_count2 = client_count.clone();

    // share tx_api_state
    let shared_tx_api_state = Arc::new(tx_api_state);
    let shared_tx_api_state2 = shared_tx_api_state.clone();

    // share reqwest client
    let client = reqwest::Client::default();
    let client2 = client.clone();

    // Spawn a thread to continually check the status of the api
    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);

        loop {
            tokio::time::sleep(duration).await;
            let is_up = get_api_status(&client).await;

            let response = ApiStateMessage {
                client_count: client_count.load(std::sync::atomic::Ordering::SeqCst),
                origin: "api_update loop".to_string(),
                date_time: Utc::now(),
                is_up,
            };

            if shared_tx_api_state.send(response).is_err() {
                tracing::error!("Failed to send api state from checker thread");
                break;
            }
        }
    });

    // spawn a thread to continuously check the status of the websocket connections
    tokio::spawn(async move {
        while let Some(state) = rx_ws_state.recv().await {
            match state {
                WsState::Connected => {
                    tracing::info!("Client connected");
                    client_count2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                WsState::Disconnected => {
                    tracing::info!("Client disconnected");
                    client_count2.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                }
            }

            let client_count = client_count2.load(std::sync::atomic::Ordering::SeqCst);
            tracing::info!("Client count: {client_count}");

            let is_up = get_api_status(&client2).await;

            if let Err(e) = shared_tx_api_state2.send(ApiStateMessage {
                client_count,
                origin: "ws_update".to_string(),
                date_time: Utc::now(),
                is_up,
            }) {
                tracing::error!("Failed to send api state: {e:?}");
            }
        }
    });

    let app_state = web::Data::new((tx_ws_state, rx_api_state));

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/ws")
                    .app_data(app_state)
                    .route(web::get().to(websocket)),
            );
    };
    Ok(config.into())
}

async fn get_api_status(client: &reqwest::Client) -> bool {
    let response = client.get(STATUS_URI).send().await;
    response.is_ok()
}
