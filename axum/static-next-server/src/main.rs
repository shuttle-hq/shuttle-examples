use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", PathBuf::from("static")).index_file("index.html"));

    Ok(router.into())
}
