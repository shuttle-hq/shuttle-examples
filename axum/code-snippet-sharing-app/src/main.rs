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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: String,
    content: String,
    language: String,
    title: Option<String>,
    description: Option<String>,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    view_count: u32,
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
    view_count: u32,
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
    view_count: u32,
    char_count: usize,
}

#[derive(Serialize)]
struct CreateSnippetResponse {
    id: String,
    url: String,
}

type SnippetStore = Arc<Mutex<HashMap<String, Snippet>>>;

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
    State(store): State<SnippetStore>,
    Json(payload): Json<CreateSnippet>,
) -> Result<(StatusCode, Json<CreateSnippetResponse>), StatusCode> {
    let id = generate_snippet_id();
    let now = Utc::now();

    // Calculate expiration time if specified
    let expires_at = payload
        .expires_in_hours
        .map(|hours| now + Duration::hours(hours));

    let snippet = Snippet {
        id: id.clone(),
        content: payload.content,
        language: payload.language,
        title: payload.title,
        description: payload.description,
        created_at: now,
        expires_at,
        view_count: 0,
        is_public: payload.is_public.unwrap_or(true),
    };

    let mut snippets = store.lock().unwrap();
    snippets.insert(id.clone(), snippet);

    let response = CreateSnippetResponse {
        id: id.clone(),
        url: format!("/snippets/{}", id),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_snippet_full(
    State(store): State<SnippetStore>,
    Path(id): Path<String>,
) -> Result<Json<SnippetFull>, StatusCode> {
    let mut snippets = store.lock().unwrap();

    match snippets.get_mut(&id) {
        Some(snippet) => {
            if let Some(expires_at) = snippet.expires_at {
                if Utc::now() > expires_at {
                    snippets.remove(&id);
                    return Err(StatusCode::NOT_FOUND);
                }
            }

            snippet.view_count += 1;

            let full = SnippetFull {
                id: snippet.id.clone(),
                title: snippet.title.clone(),
                description: snippet.description.clone(),
                language: snippet.language.clone(),
                created_at: snippet.created_at,
                expires_at: snippet.expires_at,
                view_count: snippet.view_count,
                char_count: snippet.content.len(),
                content: snippet.content.clone(),
            };

            Ok(Json(full))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn list_snippets(
    State(store): State<SnippetStore>,
    Query(params): Query<ListQuery>,
) -> Json<Vec<SnippetInfo>> {
    let snippets = store.lock().unwrap();
    let now = Utc::now();

    let mut snippet_infos: Vec<SnippetInfo> = snippets
        .values()
        .filter(|snippet| {
            // Only show public, non-expired snippets
            snippet.is_public
                && snippet.expires_at.is_none_or(|exp| now <= exp)
                && params
                    .language
                    .as_ref()
                    .is_none_or(|lang| &snippet.language == lang)
        })
        .map(|snippet| SnippetInfo {
            id: snippet.id.clone(),
            title: snippet.title.clone(),
            description: snippet.description.clone(),
            language: snippet.language.clone(),
            created_at: snippet.created_at,
            expires_at: snippet.expires_at,
            view_count: snippet.view_count,
            char_count: snippet.content.len(),
        })
        .collect();

    // Sort by creation date (newest first)
    snippet_infos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Limit results
    let limit = params.limit.unwrap_or(20).min(100);
    snippet_infos.truncate(limit);

    Json(snippet_infos)
}

async fn delete_snippet(
    State(store): State<SnippetStore>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut snippets = store.lock().unwrap();

    match snippets.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Initialize the in-memory store
    let store: SnippetStore = Arc::new(Mutex::new(HashMap::new()));

    // Build the router
    let router = Router::new()
        .route("/health", get(health))
        .route("/snippets", post(create_snippet).get(list_snippets))
        .route(
            "/snippets/{id}",
            get(get_snippet_full).delete(delete_snippet),
        )
        .with_state(store);

    Ok(router.into())
}
