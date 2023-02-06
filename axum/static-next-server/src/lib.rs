use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;
use sync_wrapper::SyncWrapper;

#[shuttle_service::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
