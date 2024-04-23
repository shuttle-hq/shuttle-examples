use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shuttle_persist::PersistError;

#[derive(Debug)]
pub(crate) struct CrontabServiceError;

impl From<PersistError> for CrontabServiceError {
    fn from(_err: PersistError) -> Self {
        Self
    }
}

impl IntoResponse for CrontabServiceError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
