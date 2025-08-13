use crate::init::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rmcp::transport::auth::AuthorizationMetadata;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub async fn oauth_authorization_server(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let base_url = state
        .secrets
        .get("BASE_URL")
        .expect("BASE_URL secret not found");

    let mut additional_fields = HashMap::new();
    additional_fields.insert(
        "response_types_supported".into(),
        Value::Array(vec![Value::String("code".into())]),
    );
    additional_fields.insert(
        "code_challenge_methods_supported".into(),
        Value::Array(vec![Value::String("S256".into())]),
    );

    let metadata = AuthorizationMetadata {
        issuer: Some(base_url.clone()),
        registration_endpoint: format!("{base_url}/oauth/register"),
        authorization_endpoint: format!("{base_url}/oauth/authorize"),
        token_endpoint: format!("{base_url}/oauth/token"),

        scopes_supported: Some(vec!["profile".to_string(), "email".to_string()]),
        jwks_uri: None,
        additional_fields,
    };

    (StatusCode::OK, Json(metadata)).into_response()
}
