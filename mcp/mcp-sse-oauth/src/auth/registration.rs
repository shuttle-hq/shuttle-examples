use crate::init::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono;
use rand;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ClientRegistrationRequest {
    pub client_name: Option<String>,
    pub redirect_uris: Vec<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ClientRegistrationResponse {
    pub client_id: String,
    pub client_secret: String,
    pub client_name: String,
    pub redirect_uris: Vec<String>,
    pub scope: String,
    pub client_id_issued_at: i64,
    pub client_secret_expires_at: i64,
}

fn generate_client_secret() -> String {
    use std::fmt::Write;
    let mut secret = String::new();
    for _ in 0..32 {
        let byte: u8 = rand::random();
        write!(&mut secret, "{byte:02x}").unwrap();
    }
    secret
}

pub async fn client_registration(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ClientRegistrationRequest>,
) -> impl IntoResponse {
    // Validate redirect URIs are provided
    if request.redirect_uris.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid_request",
                "error_description": "redirect_uris is required and must not be empty"
            })),
        )
            .into_response();
    }

    // Generate client credentials
    let client_id = Uuid::new_v4().to_string();
    let client_secret = generate_client_secret();
    let client_name = request
        .client_name
        .unwrap_or_else(|| "MCP Client".to_string());
    let scopes = request.scope.unwrap_or_else(|| "mcp".to_string());
    let issued_at = chrono::Utc::now().timestamp();
    let expires_at = chrono::Utc::now() + chrono::Duration::days(90);

    // Store client in database
    let query_result = sqlx::query!(
        r#"
        INSERT INTO mcp_clients (client_id, client_secret, client_name, redirect_uris, client_secret_expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        client_id,
        client_secret,
        client_name,
        &request.redirect_uris,
        expires_at
    )
    .execute(&state.pool)
    .await;

    match query_result {
        Ok(_) => {
            let response = ClientRegistrationResponse {
                client_id: client_id.clone(),
                client_secret,
                client_name,
                redirect_uris: request.redirect_uris,
                scope: scopes,
                client_id_issued_at: issued_at,
                client_secret_expires_at: expires_at.timestamp(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to register client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "server_error",
                    "error_description": "Failed to register client"
                })),
            )
                .into_response()
        }
    }
}
