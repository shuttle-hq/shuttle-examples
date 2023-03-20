use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"));

    Ok(router.into())
}
