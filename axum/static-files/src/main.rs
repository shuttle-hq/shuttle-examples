use std::path::PathBuf;

use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(
    // Name your static assets folder by passing `folder = <name>` to `StaticFolder`
    // If you don't pass a name, it will default to `static`.
    #[shuttle_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .merge(SpaRouter::new("/assets", static_folder).index_file("index.html"));

    Ok(router)
}
