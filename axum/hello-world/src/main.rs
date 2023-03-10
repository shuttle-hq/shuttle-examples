use axum::{routing::get, Router};
use shuttle_axum::AxumService;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world));

    Ok(AxumService(router))
}
