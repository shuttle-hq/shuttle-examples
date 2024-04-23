use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub(crate) struct CrontabServiceError;

impl IntoResponse for CrontabServiceError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
