use axum::{routing::get, Router};
use shuttle_metadata::Metadata;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_metadata::ShuttleMetadata] metadata: Metadata,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(format!("{:?}", metadata)));

    Ok(router.into())
}
