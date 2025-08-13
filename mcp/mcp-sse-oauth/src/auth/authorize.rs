use crate::init::AppState;
use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use chrono::{Duration, Utc};
use rand::Rng;
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

#[derive(Template)]
#[template(path = "authorize.html")]
struct AuthorizeTemplate {
    client_id: String,
    client_name: String,
    redirect_uri: String,
    scope: String,
    scopes: Vec<String>,
    code_challenge: String,
    code_challenge_method: String,
    state: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeForm {
    pub client_id: String,
    pub redirect_uri: String,
    pub code_challenge: String,
    pub state: Option<String>,
    pub action: String,
}

fn generate_authorization_code() -> String {
    use std::fmt::Write;
    let mut code = String::new();
    let mut rng = rand::rng();
    for _ in 0..43 {
        let byte: u8 = rng.random();
        write!(&mut code, "{byte:02x}").unwrap();
    }
    code
}

pub async fn authorize_get(
    Query(params): Query<AuthorizeRequest>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Validate required parameters
    if params.response_type != "code" {
        return (
            StatusCode::BAD_REQUEST,
            Html("Unsupported response type".to_string()),
        )
            .into_response();
    }

    // Look up client in database
    let client_result = sqlx::query!(
        "SELECT client_name FROM mcp_clients WHERE client_id = $1",
        params.client_id
    )
    .fetch_optional(&state.pool)
    .await;

    let client = match client_result {
        Ok(Some(client)) => client,
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, Html("Invalid client".to_string())).into_response()
        }
        Err(e) => {
            error!("Database error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response();
        }
    };

    let scope = params.scope.unwrap_or_else(|| "profile email".to_string());
    let scopes: Vec<String> = scope.split_whitespace().map(|s| s.to_string()).collect();

    let template = AuthorizeTemplate {
        client_id: params.client_id,
        client_name: client.client_name,
        redirect_uri: params.redirect_uri,
        scope: scope.clone(),
        scopes,
        code_challenge: params.code_challenge,
        code_challenge_method: params.code_challenge_method,
        state: params.state.unwrap_or_default(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Template render error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Template error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn authorize_post(
    State(state): State<Arc<AppState>>,
    Form(form): Form<AuthorizeForm>,
) -> impl IntoResponse {
    if form.action == "deny" {
        let mut redirect_url = format!("{}?error=access_denied", form.redirect_uri);
        if let Some(state) = form.state {
            redirect_url.push_str(&format!("&state={state}"));
        }
        return Redirect::to(&redirect_url).into_response();
    }

    // Generate authorization code
    let auth_code = generate_authorization_code();
    let expires_at = Utc::now() + Duration::minutes(10); // 10 minute expiration

    // Store authorization code in database
    let store_result = sqlx::query!(
        r#"
        INSERT INTO authorization_codes (code, client_id, redirect_uri, code_challenge, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        auth_code,
        form.client_id,
        form.redirect_uri,
        form.code_challenge,
        expires_at
    )
    .execute(&state.pool)
    .await;

    match store_result {
        Ok(_) => {
            let mut redirect_url = format!("{}?code={}", form.redirect_uri, auth_code);
            if let Some(state) = form.state {
                redirect_url.push_str(&format!("&state={state}"));
            }
            Redirect::to(&redirect_url).into_response()
        }
        Err(e) => {
            error!("Failed to store authorization code: {}", e);
            let mut redirect_url = format!("{}?error=server_error", form.redirect_uri);
            if let Some(state) = form.state {
                redirect_url.push_str(&format!("&state={state}"));
            }
            Redirect::to(&redirect_url).into_response()
        }
    }
}
