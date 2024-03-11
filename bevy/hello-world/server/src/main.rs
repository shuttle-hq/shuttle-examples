use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    );
    Ok(router.into())
}
