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

#[derive(Debug)]
enum WsState {
    Connected,
    Disconnected,
}

#[derive(Serialize, actix::Message, Default, Clone, Debug)]
#[rtype(result = "()")]
struct ApiStateMessage {
    clients_count: usize,
    date_time: DateTime<Utc>,
    is_up: bool,
}

struct WsActor {
    app_state: Arc<AppState>,
}

impl WsActor {
    fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    fn start(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let recipient = ctx.address().recipient();
        let tx_ws_state = self.app_state.0.clone();
        let mut rx_api_state = self.app_state.1.clone();

        if let Err(e) = tx_ws_state.send(WsState::Connected) {
            tracing::error!("Failed to send connected state: {e:?}");
        }

        let fut = async move {
            // listen to the state channel
            while let Ok(()) = rx_api_state.changed().await {
                let msg = rx_api_state.borrow().clone();
                tracing::info!("WS: Sending {msg:?}");
                recipient.do_send(msg);
            }
        };

        fut.into_actor(self).spawn(ctx);
    }
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start(ctx);
    }
}

impl Handler<ApiStateMessage> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: ApiStateMessage, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg).unwrap();
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        tracing::info!("WS: {msg:?}");
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
    let response = ws::start(WsActor::new(app_state.into_inner().clone()), &req, stream);
    tracing::info!("New WS: {response:?}");
    response
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    // As we cannot use the `actix_web::main` macro a pure actor based implementation
    // In order for a System to be created we need to have access to the instance of the Tokio runtime.
    // For the moment, we're going to use channels to communicate between threads.
    // api state channel
    let (tx_api_state, rx_api_state) = watch::channel(ApiStateMessage::default());
    // websocket state channel
    let (tx_ws_state, mut rx_ws_state) = mpsc::unbounded_channel::<WsState>();

    let client_count = Arc::new(AtomicUsize::new(0));

    // Spawn a thread to continually check the status of the api
    let client_count_send = client_count.clone();
    let client = reqwest::Client::default();
    let client = client.clone();
    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);

        loop {
            let is_up = get_api_status(&client).await;

            let response = ApiStateMessage {
                clients_count: client_count_send.load(std::sync::atomic::Ordering::SeqCst),
                date_time: Utc::now(),
                is_up,
            };

            if tx_api_state.send(response).is_err() {
                break;
            }

            actix::clock::sleep(duration).await;
        }
    });

    // spawn a thread to continually check the status of the websocket connections
    tokio::spawn(async move {
        while let Some(state) = rx_ws_state.recv().await {
            match state {
                WsState::Connected => {
                    client_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                WsState::Disconnected => {
                    client_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                }
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
