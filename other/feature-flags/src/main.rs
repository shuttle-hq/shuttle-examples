use axum::{routing::get, Router};

#[cfg(not(feature = "shuttle"))]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(feature = "shuttle")]
async fn hello() -> &'static str {
    "Hello, Shuttle!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello));

    Ok(router.into())
}
