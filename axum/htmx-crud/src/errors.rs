use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiError {
    SQLError(sqlx::Error),
    HTTPError(axum::http::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::SQLError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("SQL error: {e}")).into_response()
            }
            Self::HTTPError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("HTTP error: {e}"),
            )
                .into_response(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        Self::SQLError(e)
    }
}

impl From<axum::http::Error> for ApiError {
    fn from(e: axum::http::Error) -> Self {
        Self::HTTPError(e)
    }
}
