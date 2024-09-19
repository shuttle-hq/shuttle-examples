use axum::{routing::get, Router};

use shuttle_firecrawl::firecrawl::App;

#[derive(Clone)]
struct AppState {
    firecrawl: FirecrawlApp,
}

impl AppState {
    fn new(firecrawl: FirecrawlApp) -> Self {
        Self { firecrawl }
    }
}

async fn hello_world() -> String {
        let scrapeResponse = app.scrape_url("https://firecrawl.dev", Some(json!({
            "formats": ["markdown", "html"]
        })));

        scrapeResponse
}

#[shuttle_runtime::main]
async fn main(#[shuttle_firecrawl::Firecrawl] firecrawl: FirecrawlApp) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(firecrawl);

    let router = Router::new().route("/", get(scrape_page)).with_state(state);

    Ok(router.into())
}
