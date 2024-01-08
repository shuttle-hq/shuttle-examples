use multi_binary::build_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all resources in some way
    dotenvy::from_filename("Secrets.toml")?;
    let my_secret = std::env::var("SOME_API_KEY").unwrap();

    // Use the shared build function
    let router = build_router(my_secret);

    // Do the serving on its own
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
