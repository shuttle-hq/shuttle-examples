use rama::service::service_fn;
use std::convert::Infallible;

async fn hello_world() -> Result<&'static str, Infallible> {
    Ok("Hello, world!")
}

#[shuttle_runtime::main]
async fn main() -> Result<impl shuttle_rama::ShuttleService, shuttle_rama::ShuttleError> {
    Ok(shuttle_rama::RamaService::application(service_fn(
        hello_world,
    )))
}
