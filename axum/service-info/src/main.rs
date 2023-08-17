use axum::{routing::get, Router};
use shuttle_service_info::ServiceInfo;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_service_info::ShuttleServiceInfo] service_info: ServiceInfo,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(format!("{:?}", service_info)));

    Ok(router.into())
}
