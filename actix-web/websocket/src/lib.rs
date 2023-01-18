use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use serde::Serialize;
use shuttle_service::ShuttleActixWeb;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};
use tokio::sync::{mpsc, watch};

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.rs";

type AppState = (
    mpsc::UnboundedSender<WsState>,
    watch::Receiver<ApiStateMessage>,
);

#[derive(Debug, Clone)]
enum WsState {
    Connected,
    Disconnected,
}

#[derive(Serialize, actix::Message, Default, Clone, Debug)]
#[rtype(result = "()")]
struct ApiStateMessage {
    client_count: usize,
    origin: String,
    date_time: DateTime<Utc>,
    is_up: bool,
}

struct WsActor {
    app_state: Arc<AppState>,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let tx_ws_state = self.app_state.0.clone();
        let msg = WsState::Connected;
        if let Err(e) = tx_ws_state.send(msg.clone()) {
            tracing::error!("Failed to send connected state: {e:?}");
        }

        let addr = ctx.address();
        let mut rx_api_state = self.app_state.1.clone();
        let fut = async move {
            // adding some delay to avoid getting the first message too soon.
            actix::clock::sleep(Duration::from_millis(500)).await;
            while rx_api_state.changed().await.is_ok() {
                let msg = rx_api_state.borrow().clone();
                addr.do_send(msg);
            }
        };

        fut.into_actor(self).spawn(ctx);
    }
}

impl Handler<ApiStateMessage> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: ApiStateMessage, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg).unwrap();
        tracing::info!("Handling ApiStateMessage: {msg:?}");
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        tracing::info!("Handling ws::Message: {msg:?}");
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Close(reason)) => {
                let tx = self.app_state.0.clone();
                if let Err(e) = tx.send(WsState::Disconnected) {
                    tracing::error!("Failed to send connected state: {e:?}");
                }
                ctx.close(reason)
            }
            _ => (),
        }
    }
}

async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let app_state = app_state.into_inner();
    ws::start(WsActor { app_state }, &req, stream)
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    // As we cannot use the `actix_web::main` macro, we cannot use a pure actor based implementation.
    // In order for a System to be created we need to have access to the instance of the Tokio runtime, and we can't.
    // For the moment, we're going to use channels to communicate between threads.
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
            actix::clock::sleep(duration).await;
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
                origin: format!("ws_update"),
                date_time: Utc::now(),
                is_up,
            }) {
                tracing::error!("Failed to send api state: {e:?}");
            }
        }
    });

    let app_state = web::Data::new((tx_ws_state, rx_api_state));

    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/ws")
                    .app_data(app_state)
                    .route(web::get().to(websocket)),
            );
    })
}

async fn get_api_status(client: &reqwest::Client) -> bool {
    let response = client.get(STATUS_URI).send().await;
    response.is_ok()
}
