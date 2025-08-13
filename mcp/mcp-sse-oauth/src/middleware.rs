use crate::init::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // client_id
    pub iat: i64,      // issued at
    pub exp: i64,      // expires at
    pub scope: String, // granted scopes
}

pub async fn validate_token_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = request.headers().get("Authorization");
    let token = match auth_header {
        Some(header) => {
            let header_str = match header.to_str() {
                Ok(s) => s,
                Err(_) => {
                    error!("Invalid Authorization header encoding");
                    return StatusCode::UNAUTHORIZED.into_response();
                }
            };

            if let Some(stripped) = header_str.strip_prefix("Bearer ") {
                stripped.to_string()
            } else {
                error!("Authorization header missing Bearer prefix");
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
        None => {
            error!("Missing Authorization header");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let jwt_secret = state
        .secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET secret not found");

    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();

    match decode::<Claims>(&token, &key, &validation) {
        Ok(token_data) => {
            // Check if token is expired (JWT validation already handles this, but being explicit)
            let now = chrono::Utc::now().timestamp();
            if token_data.claims.exp < now {
                error!("Token has expired");
                return StatusCode::UNAUTHORIZED.into_response();
            }

            // Optionally, verify the client still exists in database
            let client_exists = sqlx::query!(
                "SELECT client_id FROM mcp_clients WHERE client_id = $1",
                token_data.claims.sub
            )
            .fetch_optional(&state.pool)
            .await;

            match client_exists {
                Ok(Some(_)) => {
                    // Add client_id to request extensions for downstream handlers
                    request.extensions_mut().insert(token_data.claims.sub);
                    next.run(request).await
                }
                Ok(None) => {
                    error!("Client no longer exists: {}", token_data.claims.sub);
                    StatusCode::UNAUTHORIZED.into_response()
                }
                Err(e) => {
                    error!("Database error validating client: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            error!("Token validation failed: {}", e);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
