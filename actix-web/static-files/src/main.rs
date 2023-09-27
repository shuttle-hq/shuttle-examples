use actix_files::NamedFile;
use actix_web::{get, web::ServiceConfig, Responder};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("assets/index.html").await
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index);
    };

    Ok(config.into())
}
