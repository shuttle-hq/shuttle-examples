use rama::http::service::fs::ServeDir;

#[shuttle_runtime::main]
async fn main() -> Result<impl shuttle_rama::ShuttleService, shuttle_rama::ShuttleError> {
    Ok(shuttle_rama::RamaService::application(ServeDir::new(
        "assets",
    )))
}
