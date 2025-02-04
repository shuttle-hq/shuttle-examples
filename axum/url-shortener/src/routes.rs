// src/lib/routes/routes.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::{headers::Host, TypedHeader};
use axum_macros::debug_handler;
use sqlx::{Error, PgPool};
use tracing::instrument;
use url::Url;

// health_check handler
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// redirect endpoint handler
#[debug_handler]
#[instrument(name = "redirect" skip(state))]
pub async fn get_redirect(
    State(state): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let url: (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_one(&state)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => {
                tracing::error!("shortened URL not found in the database...");
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    tracing::info!("shortened URL retrieved, redirecting...");
    Ok(Redirect::permanent(&url.0))
}

// shorten endpoint handler
#[debug_handler]
#[instrument(name = "shorten" skip(state))]
pub async fn post_shorten(
    State(state): State<PgPool>,
    TypedHeader(header): TypedHeader<Host>,
    url: String,
) -> Result<impl IntoResponse, StatusCode> {
    let id = &nanoid::nanoid!(6);
    let p_url = Url::parse(&url).map_err(|_| {
        tracing::error!("Unable to parse URL");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;
    let host = header.hostname();
    sqlx::query("INSERT INTO urls (id, url) VALUES ($1, $2)")
        .bind(id)
        .bind(p_url.as_str())
        .execute(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_body = format!("https://{}/{}\n", host, id);

    tracing::info!("URL shortened and saved successfully...");
    Ok((StatusCode::OK, response_body).into_response())
}
