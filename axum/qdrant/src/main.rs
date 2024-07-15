use axum::{extract::State, routing::get, Router};
use qdrant_client::Qdrant;
use std::sync::Arc;

struct AppState {
    qdrant: Qdrant,
}

async fn list_collections(State(state): State<Arc<AppState>>) -> String {
    format!("{:?}\n", state.qdrant.list_collections().await)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_qdrant::Qdrant(cloud_url = "{secrets.CLOUD_URL}", api_key = "{secrets.API_KEY}")]
    qdrant: Qdrant,
) -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(AppState { qdrant });

    let router = Router::new()
        .route("/", get(list_collections))
        .with_state(state);

    Ok(router.into())
}
