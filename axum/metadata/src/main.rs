use axum::{routing::get, Router};
use shuttle_runtime::DeploymentMetadata;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Metadata] metadata: DeploymentMetadata,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(format!("{:?}", metadata)));

    Ok(router.into())
}
