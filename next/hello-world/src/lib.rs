use tracing::debug;

shuttle_next::app! {
    #[shuttle_next::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    // We can also use tracing/log macros directly:
    #[shuttle_next::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        debug!("goodbye endpoint called")
        "Goodbye, World!"
    }
}
