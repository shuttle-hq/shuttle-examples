use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Deserialize)]
struct TodoRequest {
    pub content: String,
}

#[derive(Serialize, FromRow)]
struct TodoResponse {
    pub id: i32,
    pub content: String,
}

async fn get_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<TodoResponse>>, (StatusCode, String)> {
    let todos = sqlx::query_as("SELECT id, content FROM todos")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(todos))
}

async fn post_todos(
    State(state): State<AppState>,
    Json(req): Json<TodoRequest>,
) -> Result<Json<TodoResponse>, (StatusCode, String)> {
    let todo = sqlx::query_as("INSERT INTO todos (content) VALUES ($1) RETURNING id, content")
        .bind(&req.content)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, dbg!(e).to_string()))?;

    Ok(Json(todo))
}

async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TodoResponse>, (StatusCode, String)> {
    let todo = sqlx::query_as("DELETE FROM todos WHERE id = $1 RETURNING id, content")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(todo))
}

async fn put_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<TodoRequest>,
) -> Result<Json<TodoResponse>, (StatusCode, String)> {
    let todo = sqlx::query_as("UPDATE todos SET content = $1 WHERE id = $2 RETURNING id, content")
        .bind(&req.content)
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(todo))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState { pool };
    let router = Router::new()
        // API routes
        .route("/api/todos", get(get_todos).post(post_todos))
        .route("/api/todos/{id}", delete(delete_todo).put(put_todo))
        // Serve files from 'assets' directory when no API route matches the request.
        // ServeDir falls back to serve index.html when requesting a directory.
        .fallback_service(ServeDir::new("assets"))
        // Emit INFO level tracing events on each request
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    Ok(router.into())
}
