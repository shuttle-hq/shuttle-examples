use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use firecrawl::{
    scrape::{ScrapeFormats, ScrapeOptions},
    FirecrawlApp,
};
use serde::Deserialize;
use url::Url;

#[derive(Clone)]
struct AppState {
    firecrawl: FirecrawlApp,
}

impl AppState {
    fn new() -> Self {
        let firecrawl_api_key =
            std::env::var("FIRECRAWL_API_KEY").expect("FIRECRAWL_API_KEY to be set as an env var");
        let firecrawl = FirecrawlApp::new(firecrawl_api_key).unwrap();
        Self { firecrawl }
    }
}

#[derive(Deserialize)]
struct Request {
    url: Url,
}

async fn scrape_page(
    State(state): State<AppState>,
    Json(json): Json<Request>,
) -> impl IntoResponse {
    let scrape_options = ScrapeOptions {
        formats: Some(vec![ScrapeFormats::HTML, ScrapeFormats::Markdown]),
        ..Default::default()
    };

    let scrape_response = state
        .firecrawl
        .scrape_url(&json.url, scrape_options)
        .await
        .unwrap();

    Json(scrape_response)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });

    let state = AppState::new();

    let router = Router::new()
        .route("/scrape", get(scrape_page))
        .with_state(state);

    Ok(router.into())
}
