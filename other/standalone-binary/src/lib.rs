use axum::{extract::State, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn some_service(State(state): State<MyAxumState>) -> &'static str {
    println!("Secret from state: {}", state.api_key);
    "This function can use a secret to generate the response"
}

#[derive(Clone)]
struct MyAxumState {
    pub api_key: String,
    // add other stuff here
}

/// This function builds the Router (the struct that shuttle_runtime::main wants to return).
/// It uses any resource that Shuttle's main function gets hold of.
/// The standalone binary has to provide those by itself.
pub fn build_router(api_key_for_some_service: String) -> Router {
    let state = MyAxumState {
        api_key: api_key_for_some_service,
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/service", get(some_service))
        .with_state(state)
}
