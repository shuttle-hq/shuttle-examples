use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

pub fn build_router() -> Router {
    Router::new().route("/", get(hello_world))
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}
