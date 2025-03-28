use axum::{routing::get, Router, extract::State};

async fn hello_world(State(state): State<AppState>,) -> String {
    let body = format!("Hello from {}", state.id);
    body
}

#[derive(Clone)]
struct AppState {
    pub id: ulid::Ulid
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = AppState {id: ulid::Ulid::new()};
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
