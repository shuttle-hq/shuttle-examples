use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

async fn hello_world() -> &'static str {
    "Hello world! Go to /game to see your Bevy build."
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world)).nest_service(
        "/game",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    );
    Ok(router.into())
}
