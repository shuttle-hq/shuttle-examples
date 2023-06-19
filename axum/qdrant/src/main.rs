use axum::{extract::State, routing::get, Router};
use qdrant_client::prelude::*;
use std::sync::Arc;

struct AppState {
    qdrant: QdrantClient,
}

async fn list_collections(State(state): State<Arc<AppState>>) -> String {
    format!("{:?}\n", state.qdrant.list_collections().await)
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_qdrant::Qdrant] info: shuttle_qdrant::QdrantReadyInfo
) -> shuttle_axum::ShuttleAxum {
    let mut config = QdrantClientConfig::from_url(&info.url);

    if let Some(api_key) = info.api_key {
        config.set_api_key(&api_key);
    }

    let qdrant = QdrantClient::new(Some(config))?;
    let state = Arc::new(AppState { qdrant });

    let router = Router::new().route("/", get(list_collections)).with_state(state);

    Ok(router.into())
}
