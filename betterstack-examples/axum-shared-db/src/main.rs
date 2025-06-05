use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::time::Instant;

async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Track total requests
    tracing::info!(counter.db_requests_total = 1, "New DB request received");
    
    let start = Instant::now();
    
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await;

    // Record query duration
    let duration = start.elapsed().as_millis() as f64;
    tracing::info!(
        histogram.db_query_duration_ms = duration,
        "Query execution time"
    );

    match result {
        Ok(todo) => {
            // Track successful requests
            tracing::info!(counter.db_requests_success = 1, "DB request successful");
            Ok((StatusCode::OK, Json(todo)))
        }
        Err(e) => {
            // Track failed requests
            tracing::info!(counter.db_requests_failure = 1, "DB request failed");
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

async fn add(
    State(state): State<MyState>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/todos", post(add))
        .route("/todos/:id", get(retrieve))
        .with_state(state);

    Ok(router.into())
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}
