use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_axum::axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct Snippet {
    id: String,
    content: String,
    language: String,
    title: Option<String>,
    description: Option<String>,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    view_count: i32,
    is_public: bool,
}

#[derive(Deserialize)]
struct CreateSnippet {
    content: String,
    language: String,
    title: Option<String>,
    description: Option<String>,
    expires_in_hours: Option<i64>,
    is_public: Option<bool>,
}

#[derive(Deserialize)]
struct ListQuery {
    language: Option<String>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct SnippetFull {
    id: String,
    title: Option<String>,
    description: Option<String>,
    language: String,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    view_count: i32,
    char_count: usize,
    content: String,
}

#[derive(Serialize)]
struct SnippetInfo {
    id: String,
    title: Option<String>,
    description: Option<String>,
    language: String,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    view_count: i32,
    char_count: i32,
}

#[derive(Serialize)]
struct CreateSnippetResponse {
    id: String,
    url: String,
}

// Generate a short, URL-friendly ID
fn generate_snippet_id() -> String {
    nanoid::nanoid!(8) // Generates 8-character ID like "V1StGXR8"
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "code-snippet-share",
        "timestamp": Utc::now()
    }))
}

async fn create_snippet(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSnippet>,
) -> Result<(StatusCode, Json<CreateSnippetResponse>), StatusCode> {
    let id = generate_snippet_id();
    let now = Utc::now();

    // Calculate expiration time if specified
    let expires_at = payload
        .expires_in_hours
        .map(|hours| now + Duration::hours(hours));

    let result = sqlx::query!(
        r#"
        INSERT INTO snippets (id, content, language, title, description, created_at, expires_at, view_count, is_public)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        id,
        payload.content,
        payload.language,
        payload.title,
        payload.description,
        now,
        expires_at,
        0i32,
        payload.is_public.unwrap_or(true)
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let response = CreateSnippetResponse {
                id: id.clone(),
                url: format!("/snippets/{}", id),
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_snippet_full(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<SnippetFull>, StatusCode> {
    let now = Utc::now();

    // First, check if snippet exists and is not expired
    let snippet = sqlx::query_as!(
        Snippet,
        r#"
        SELECT id, content, language, title, description, created_at, expires_at, view_count, is_public
        FROM snippets
        WHERE id = $1 AND (expires_at IS NULL OR expires_at > $2)
        "#,
        id,
        now
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match snippet {
        Some(mut snippet) => {
            // Increment view count
            let result = sqlx::query!(
                "UPDATE snippets SET view_count = view_count + 1 WHERE id = $1",
                id
            )
            .execute(&pool)
            .await;

            if result.is_ok() {
                snippet.view_count += 1;
            }

            let full = SnippetFull {
                id: snippet.id,
                title: snippet.title,
                description: snippet.description,
                language: snippet.language,
                created_at: snippet.created_at,
                expires_at: snippet.expires_at,
                view_count: snippet.view_count,
                char_count: snippet.content.len(),
                content: snippet.content,
            };

            Ok(Json(full))
        }
        None => {
            // Clean up expired snippets
            let _ = sqlx::query!(
                "DELETE FROM snippets WHERE expires_at IS NOT NULL AND expires_at <= $1",
                now
            )
            .execute(&pool)
            .await;

            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn list_snippets(
    State(pool): State<PgPool>,
    Query(params): Query<ListQuery>,
) -> Json<Vec<SnippetInfo>> {
    let now = Utc::now();
    let limit = params.limit.unwrap_or(20).min(100) as i64;

    let query = match params.language {
        Some(language) => {
            sqlx::query_as!(
                SnippetInfo,
                r#"
                SELECT
                    id,
                    title,
                    description,
                    language,
                    created_at,
                    expires_at,
                    view_count,
                    LENGTH(content) as "char_count!"
                FROM snippets
                WHERE is_public = true
                    AND (expires_at IS NULL OR expires_at > $1)
                    AND language = $2
                ORDER BY created_at DESC
                LIMIT $3
                "#,
                now,
                language,
                limit
            )
            .fetch_all(&pool)
            .await
        }
        None => {
            sqlx::query_as!(
                SnippetInfo,
                r#"
                SELECT
                    id,
                    title,
                    description,
                    language,
                    created_at,
                    expires_at,
                    view_count,
                    LENGTH(content) as "char_count!"
                FROM snippets
                WHERE is_public = true
                    AND (expires_at IS NULL OR expires_at > $1)
                ORDER BY created_at DESC
                LIMIT $2
                "#,
                now,
                limit
            )
            .fetch_all(&pool)
            .await
        }
    };

    let snippet_infos = query.unwrap_or_else(|_| vec![]);
    Json(snippet_infos)
}

async fn delete_snippet(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM snippets WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // Run database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Build the router
    let router = Router::new()
        .route("/health", get(health))
        .route("/snippets", post(create_snippet).get(list_snippets))
        .route(
            "/snippets/{id}",
            get(get_snippet_full).delete(delete_snippet),
        )
        .with_state(pool);

    Ok(router.into())
}
