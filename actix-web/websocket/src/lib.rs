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

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.rs";

#[derive(Serialize, actix::Message)]
#[rtype(result = "()")]
struct Response {
    clients_count: usize,
    date_time: DateTime<Utc>,
    is_up: bool,
}

#[derive(Default)]
struct WsActor {
    client_count: Arc<AtomicUsize>,
}

impl WsActor {
    fn new(client_count: Arc<AtomicUsize>) -> Self {
        Self { client_count }
    }

    fn start(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let recipient = ctx.address().recipient();
        let client_count = self.client_count.clone();
        client_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let fut = async move {
            let duration = Duration::from_secs(PAUSE_SECS);
            let client = reqwest::Client::default();

            loop {
                let is_up = client.get(STATUS_URI).send().await;
                let is_up = is_up.is_ok();

                let response = Response {
                    clients_count: client_count.load(std::sync::atomic::Ordering::SeqCst),
                    date_time: Utc::now(),
                    is_up,
                };
                recipient.do_send(response);
                actix::clock::sleep(duration).await;
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
                self.client_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                ctx.close(reason)
            }
            _ => (),
        }
    }
}

async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    client_count: web::Data<AtomicUsize>,
) -> actix_web::Result<HttpResponse> {
    let response = ws::start(
        WsActor::new(client_count.into_inner().clone()),
        &req,
        stream,
    );
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
    let client_count = Arc::new(AtomicUsize::new(0));
    let client_count = web::Data::from(client_count);

    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/ws")
                    .app_data(client_count)
                    .route(web::get().to(websocket)),
            );
    })
}
