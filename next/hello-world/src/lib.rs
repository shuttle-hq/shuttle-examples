shuttle_next::app! {
    use futures::TryStreamExt;
    use tracing::debug;
    use shuttle_next::body::StreamBody;
    use shuttle_next::extract::BodyStream;
    use shuttle_next::response::{Response, IntoResponse};

    #[shuttle_next::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    // We can also use tracing/log macros directly:
    #[shuttle_next::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        debug!("goodbye endpoint called");
        "Goodbye, World!"
    }

    // We can also extract the http body in our handlers.
    // The endpoint below takes the body from the request using the axum `BodyStream`
    // extractor, lazily maps its bytes to uppercase and streams it back in our response:
    #[shuttle_next::endpoint(method = post, route = "/uppercase")]
    async fn uppercase(body: BodyStream) -> impl IntoResponse {
        let chunk_stream = body.map_ok(|chunk| {
            chunk
                .iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Vec<u8>>()
        });
        Response::new(StreamBody::new(chunk_stream))
    }
}
