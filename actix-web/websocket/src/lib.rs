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
use std::{sync::Arc, time::Duration};
use tokio::sync::{watch, Mutex};

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.rs";

struct AppState {
    clients_count: usize,
    rx: watch::Receiver<Response>,
}

#[derive(Serialize, actix::Message, Default, Clone, Debug)]
#[rtype(result = "()")]
struct Response {
    clients_count: usize,
    date_time: DateTime<Utc>,
    is_up: bool,
}

struct WsActor {
    app_state: Arc<Mutex<AppState>>,
}

impl WsActor {
    fn new(app_state: Arc<Mutex<AppState>>) -> Self {
        Self { app_state }
    }

    fn start(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let recipient = ctx.address().recipient();
        let state = self.app_state.clone();

        let fut = async move {
            let mut rx = {
                let mut state = state.lock().await;
                state.clients_count += 1;
                state.rx.clone()
            };

            while let Ok(()) = rx.changed().await {
                let msg = rx.borrow().clone();
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

impl Handler<Response> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) {
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
                let state = self.app_state.clone();
                tokio::spawn(async move {
                    let mut state = state.lock().await;
                    state.clients_count -= 1;
                });
                ctx.close(reason)
            }
            _ => (),
        }
    }
}

async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<Mutex<AppState>>,
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
    let (tx, rx) = watch::channel(Response::default());

    let state = Arc::new(Mutex::new(AppState {
        clients_count: 0,
        rx,
    }));

    // Spawn a thread to continually check the status of the api
    let state_send = state.clone();

    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);
        let client = reqwest::Client::default();

        loop {
            let is_up = client.get(STATUS_URI).send().await;
            let is_up = is_up.is_ok();

            let response = Response {
                clients_count: state_send.lock().await.clients_count,
                date_time: Utc::now(),
                is_up,
            };

            if tx.send(response).is_err() {
                break;
            }

            actix::clock::sleep(duration).await;
        }
    });

    let app_state = web::Data::from(state);

    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/ws")
                    .app_data(app_state)
                    .route(web::get().to(websocket)),
            );
    })
}
