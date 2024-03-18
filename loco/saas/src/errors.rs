use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Stripe error: {0}")]
    Stripe(#[from] stripe::StripeError),
    #[error("User already has this subscription tier!")]
    UserTierAlreadyExists,
    #[error("SQL error: {0}")]
    SQL(#[from] sea_orm::DbErr),
    #[error("Model error: {0}")]
    Model(#[from] loco_rs::model::ModelError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let res = match self {
            Self::Stripe(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::SQL(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::UserTierAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "User already has this tier!".to_string(),
            ),
            Self::Model(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        res.into_response()
    }
}
