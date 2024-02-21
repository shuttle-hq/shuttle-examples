use axum::http::StatusCode;

pub enum ApiError {
    SqlxError,
    ReqwestError,
}

impl ApiError {
    pub fn to_error(&self, err: String) -> (StatusCode, String) {
        match self {
            ApiError::SqlxError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("SQL query error: {err}"),
            ),
            ApiError::ReqwestError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Fetch error: {err}"),
            ),
        }
    }
}
