use axum::{routing::get, Router, extract::State, Json, response::IntoResponse};
use serde_json::json;

use shuttle_firecrawl::firecrawl::FirecrawlApp;

#[derive(Clone)]
struct AppState {
    firecrawl: FirecrawlApp,
}

impl AppState {
    fn new(firecrawl: FirecrawlApp) -> Self {
        Self { firecrawl }
    }
}

async fn scrape_page(State(state): State<AppState>) -> impl IntoResponse {
        let scrape_response = state.firecrawl.scrape_url("https://firecrawl.dev", Some(json!({
            "formats": ["markdown", "html"]
        }))).await.unwrap();

        Json(scrape_response)
}

#[shuttle_runtime::main]
async fn main(#[shuttle_firecrawl::Firecrawl(
    api_key = "{secrets.FIRECRAWL_API_KEY}"
)] firecrawl: FirecrawlApp) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(firecrawl);

    let router = Router::new().route("/", get(scrape_page)).with_state(state);

    Ok(router.into())
}
