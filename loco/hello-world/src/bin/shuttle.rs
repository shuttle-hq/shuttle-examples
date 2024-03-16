use hello_world::app::App;
use loco_rs::boot::{create_app, StartMode};
use loco_rs::environment::Environment;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Metadata] meta: shuttle_runtime::DeploymentMetadata,
) -> shuttle_axum::ShuttleAxum {
    let environment = match meta.env {
        shuttle_runtime::Environment::Local => Environment::Development,
        shuttle_runtime::Environment::Deployment => Environment::Production,
    };
    let boot_result = create_app::<App>(StartMode::ServerOnly, &environment)
        .await
        .unwrap();

    let router = boot_result.router.unwrap();
    Ok(router.into())
}
