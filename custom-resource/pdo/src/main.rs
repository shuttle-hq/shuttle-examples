use axum::{extract::State, routing::get, Router};
use pdo::{Builder, Pdo};
use std::sync::Arc;

async fn hello_world(State(pdo): State<Arc<Pdo>>) -> String {
    pdo.name.clone()
}

#[shuttle_runtime::main]
async fn main(#[Builder(field = "value")] pdo: Pdo) -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(pdo);
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
