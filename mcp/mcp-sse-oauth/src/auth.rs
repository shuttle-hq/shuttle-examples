pub mod authorize;
pub mod metadata;
pub mod registration;
pub mod token;

pub use authorize::{authorize_get, authorize_post};
pub use metadata::oauth_authorization_server;
pub use registration::client_registration;
pub use token::token_post;

use crate::init::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn get_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/.well-known/oauth-authorization-server",
            get(oauth_authorization_server),
        )
        .route("/oauth/register", post(client_registration))
        .route("/oauth/authorize", get(authorize_get).post(authorize_post))
        .route("/oauth/token", post(token_post))
}
