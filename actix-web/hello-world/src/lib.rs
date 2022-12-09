use actix_web::web::{resource, ServiceConfig};
use shuttle_service::ShuttleActixWeb;

async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Copy + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(resource("/hello").to(hello_world));
    })
}
