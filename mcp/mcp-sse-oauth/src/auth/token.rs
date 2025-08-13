use crate::init::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    Form,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub code_verifier: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,   // client_id
    iat: i64,      // issued at
    exp: i64,      // expires at
    scope: String, // granted scopes
}

fn verify_pkce_challenge(code_verifier: &str, code_challenge: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let encoded = URL_SAFE_NO_PAD.encode(hash);
    encoded == code_challenge
}

fn generate_refresh_token() -> String {
    use std::fmt::Write;
    let mut token = String::new();
    let mut rng = rand::rng();
    for _ in 0..64 {
        let byte: u8 = rng.random();
        write!(&mut token, "{byte:02x}").unwrap();
    }
    token
}

fn generate_access_token(
    client_id: &str,
    scope: &str,
    jwt_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: client_id.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::minutes(15)).timestamp(), // 15 minute expiration
        scope: scope.to_string(),
    };

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &key)
}

async fn validate_client_credentials(
    state: &AppState,
    client_id: &str,
    client_secret: Option<&String>,
) -> Result<(), ErrorResponse> {
    let client_secret = match client_secret {
        Some(secret) => secret,
        None => {
            return Err(ErrorResponse {
                error: "invalid_client".to_string(),
                error_description: Some("client_secret parameter is required".to_string()),
            });
        }
    };

    let client_result = sqlx::query!(
        "SELECT client_secret FROM mcp_clients WHERE client_id = $1",
        client_id
    )
    .fetch_optional(&state.pool)
    .await;

    let client = match client_result {
        Ok(Some(client)) => client,
        Ok(None) => {
            return Err(ErrorResponse {
                error: "invalid_client".to_string(),
                error_description: Some("Client not found".to_string()),
            });
        }
        Err(e) => {
            error!("Database error during client validation: {}", e);
            return Err(ErrorResponse {
                error: "server_error".to_string(),
                error_description: Some("Internal server error".to_string()),
            });
        }
    };

    if client.client_secret != *client_secret {
        return Err(ErrorResponse {
            error: "invalid_client".to_string(),
            error_description: Some("Invalid client credentials".to_string()),
        });
    }

    Ok(())
}

pub async fn token_post(
    State(state): State<Arc<AppState>>,
    Form(request): Form<TokenRequest>,
) -> Response {
    match request.grant_type.as_str() {
        "authorization_code" => handle_authorization_code_grant(state, request)
            .await
            .into_response(),
        "refresh_token" => handle_refresh_token_grant(state, request)
            .await
            .into_response(),
        _ => {
            let error = ErrorResponse {
                error: "unsupported_grant_type".to_string(),
                error_description: Some(
                    "Only authorization_code and refresh_token grant types are supported"
                        .to_string(),
                ),
            };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        }
    }
}

async fn handle_authorization_code_grant(
    state: Arc<AppState>,
    request: TokenRequest,
) -> impl IntoResponse {
    // Validate client credentials
    if let Err(error) =
        validate_client_credentials(&state, &request.client_id, request.client_secret.as_ref())
            .await
    {
        return (StatusCode::UNAUTHORIZED, Json(error)).into_response();
    }

    // Validate required parameters for authorization code grant
    let code = match request.code {
        Some(code) => code,
        None => {
            let error = ErrorResponse {
                error: "invalid_request".to_string(),
                error_description: Some("code parameter is required".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
    };

    let redirect_uri = match request.redirect_uri {
        Some(uri) => uri,
        None => {
            let error = ErrorResponse {
                error: "invalid_request".to_string(),
                error_description: Some("redirect_uri parameter is required".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
    };

    let code_verifier = match request.code_verifier {
        Some(verifier) => verifier,
        None => {
            let error = ErrorResponse {
                error: "invalid_request".to_string(),
                error_description: Some("code_verifier parameter is required".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
    };

    // Look up authorization code in database
    let auth_code_result = sqlx::query!(
        r#"
        SELECT client_id, redirect_uri, code_challenge, expires_at
        FROM authorization_codes 
        WHERE code = $1
        "#,
        code
    )
    .fetch_optional(&state.pool)
    .await;

    let auth_code = match auth_code_result {
        Ok(Some(code)) => code,
        Ok(None) => {
            let error = ErrorResponse {
                error: "invalid_grant".to_string(),
                error_description: Some("Authorization code not found".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
        Err(e) => {
            error!("Database error: {}", e);
            let error = ErrorResponse {
                error: "server_error".to_string(),
                error_description: Some("Internal server error".to_string()),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
        }
    };

    // Validate authorization code hasn't expired

    if Utc::now() > auth_code.expires_at {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("Authorization code expired".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    if auth_code.client_id != request.client_id {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("Client ID mismatch".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    if auth_code.redirect_uri != redirect_uri {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("Redirect URI mismatch".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    // Verify PKCE code challenge
    if !verify_pkce_challenge(&code_verifier, &auth_code.code_challenge) {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("PKCE verification failed".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    // Delete authorization code (one-time use)
    let delete_result = sqlx::query!("DELETE FROM authorization_codes WHERE code = $1", code)
        .execute(&state.pool)
        .await;

    if let Err(e) = delete_result {
        error!("Failed to delete authorization code: {}", e);
    }

    // Generate access token and refresh token
    let scope = "mcp";

    let jwt_secret = state
        .secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET secret not found");

    let access_token = match generate_access_token(&request.client_id, &scope, &jwt_secret) {
        Ok(token) => token,
        Err(e) => {
            error!("Failed to generate access token: {}", e);
            let error = ErrorResponse {
                error: "server_error".to_string(),
                error_description: Some("Failed to generate access token".to_string()),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
        }
    };

    let refresh_token = generate_refresh_token();
    let refresh_expires_at = Utc::now() + Duration::days(30); // 30 day expiration

    // Store refresh token in database
    let store_refresh_result = sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (token, client_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
        refresh_token,
        request.client_id,
        refresh_expires_at
    )
    .execute(&state.pool)
    .await;

    if let Err(e) = store_refresh_result {
        error!("Failed to store refresh token: {}", e);
        let error = ErrorResponse {
            error: "server_error".to_string(),
            error_description: Some("Failed to store refresh token".to_string()),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
    }

    let response = TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 900, // 15 minutes
        refresh_token,
        scope: scope.to_string(),
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn handle_refresh_token_grant(
    state: Arc<AppState>,
    request: TokenRequest,
) -> impl IntoResponse {
    // Validate client credentials
    if let Err(error) =
        validate_client_credentials(&state, &request.client_id, request.client_secret.as_ref())
            .await
    {
        return (StatusCode::UNAUTHORIZED, Json(error)).into_response();
    }

    // Validate required parameters for refresh token grant
    let refresh_token = match request.refresh_token {
        Some(token) => token,
        None => {
            let error = ErrorResponse {
                error: "invalid_request".to_string(),
                error_description: Some("refresh_token parameter is required".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
    };

    // Look up refresh token in database
    let refresh_token_result = sqlx::query!(
        r#"
        SELECT client_id, expires_at
        FROM refresh_tokens 
        WHERE token = $1
        "#,
        refresh_token
    )
    .fetch_optional(&state.pool)
    .await;

    let refresh_token_record = match refresh_token_result {
        Ok(Some(token)) => token,
        Ok(None) => {
            let error = ErrorResponse {
                error: "invalid_grant".to_string(),
                error_description: Some("Refresh token not found".to_string()),
            };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response();
        }
        Err(e) => {
            error!("Database error: {}", e);
            let error = ErrorResponse {
                error: "server_error".to_string(),
                error_description: Some("Internal server error".to_string()),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
        }
    };

    // Validate refresh token hasn't expired

    if Utc::now() > refresh_token_record.expires_at {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("Refresh token expired".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    if refresh_token_record.client_id != request.client_id {
        let error = ErrorResponse {
            error: "invalid_grant".to_string(),
            error_description: Some("Client ID mismatch".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    // Delete old refresh token (rotation)
    let delete_result = sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", refresh_token)
        .execute(&state.pool)
        .await;

    if let Err(e) = delete_result {
        error!("Failed to delete old refresh token: {}", e);
    }

    // Generate new access token and refresh token
    let scope = "mcp";

    let jwt_secret = state
        .secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET secret not found");

    let access_token = match generate_access_token(&request.client_id, &scope, &jwt_secret) {
        Ok(token) => token,
        Err(e) => {
            error!("Failed to generate access token: {}", e);
            let error = ErrorResponse {
                error: "server_error".to_string(),
                error_description: Some("Failed to generate access token".to_string()),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
        }
    };

    let new_refresh_token = generate_refresh_token();
    let refresh_expires_at = Utc::now() + Duration::days(30); // 30 day expiration

    // Store new refresh token in database
    let store_refresh_result = sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (token, client_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
        new_refresh_token,
        request.client_id,
        refresh_expires_at
    )
    .execute(&state.pool)
    .await;

    if let Err(e) = store_refresh_result {
        error!("Failed to store new refresh token: {}", e);
        let error = ErrorResponse {
            error: "server_error".to_string(),
            error_description: Some("Failed to store refresh token".to_string()),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
    }

    let response = TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 900, // 15 minutes
        refresh_token: new_refresh_token,
        scope: scope.to_string(),
    };

    (StatusCode::OK, Json(response)).into_response()
}
