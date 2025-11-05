use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

// Application state
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// Database model for S3 upload records
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct S3Upload {
    id: Uuid,
    filename: String,
    s3_key: String,
    s3_bucket: String,
    s3_url: String,
    file_size: i64,
    content_type: Option<String>,
    uploaded_at: DateTime<Utc>,
}

// Request model for creating upload record
#[derive(Debug, Deserialize)]
struct CreateUploadRequest {
    filename: String,
    s3_key: String,
    s3_bucket: String,
    s3_url: String,
    file_size: i64,
    content_type: Option<String>,
}

// Response models
#[derive(Debug, Serialize)]
struct CreateUploadResponse {
    id: Uuid,
    filename: String,
    s3_url: String,
    uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct UploadListResponse {
    uploads: Vec<S3Upload>,
    count: usize,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

// Health check endpoint
async fn health_check() -> &'static str {
    "S3 Upload Manager API is running!"
}

// Create upload record - stores S3 upload details in database
async fn create_upload(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUploadRequest>,
) -> Result<(StatusCode, Json<CreateUploadResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if payload.filename.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Filename cannot be empty".to_string(),
            }),
        ));
    }

    if payload.s3_key.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "S3 key cannot be empty".to_string(),
            }),
        ));
    }

    if payload.file_size <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "File size must be greater than 0".to_string(),
            }),
        ));
    }

    let upload_id = Uuid::new_v4();
    let uploaded_at = Utc::now();

    // Insert upload record into database
    sqlx::query(
        r#"
        INSERT INTO s3_uploads (id, filename, s3_key, s3_bucket, s3_url, file_size, content_type, uploaded_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(upload_id)
    .bind(&payload.filename)
    .bind(&payload.s3_key)
    .bind(&payload.s3_bucket)
    .bind(&payload.s3_url)
    .bind(payload.file_size)
    .bind(&payload.content_type)
    .bind(uploaded_at)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to save upload record: {}", e),
            }),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateUploadResponse {
            id: upload_id,
            filename: payload.filename,
            s3_url: payload.s3_url,
            uploaded_at,
        }),
    ))
}

// Get all upload records
async fn get_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<Json<UploadListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uploads = sqlx::query_as::<_, S3Upload>(
        r#"
        SELECT id, filename, s3_key, s3_bucket, s3_url, file_size, content_type, uploaded_at
        FROM s3_uploads
        ORDER BY uploaded_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch uploads: {}", e),
            }),
        )
    })?;

    let count = uploads.len();

    Ok(Json(UploadListResponse { uploads, count }))
}

// Get a specific upload record by ID
async fn get_upload_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<S3Upload>, (StatusCode, Json<ErrorResponse>)> {
    let upload = sqlx::query_as::<_, S3Upload>(
        r#"
        SELECT id, filename, s3_key, s3_bucket, s3_url, file_size, content_type, uploaded_at
        FROM s3_uploads
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        )
    })?;

    match upload {
        Some(u) => Ok(Json(u)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Upload record not found".to_string(),
            }),
        )),
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Run database migrations - create table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS s3_uploads (
            id UUID PRIMARY KEY,
            filename VARCHAR(255) NOT NULL,
            s3_key VARCHAR(512) NOT NULL,
            s3_bucket VARCHAR(255) NOT NULL,
            s3_url VARCHAR(512) NOT NULL,
            file_size BIGINT NOT NULL,
            content_type VARCHAR(100),
            uploaded_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(&db)
    .await
    .expect("Failed to create s3_uploads table");

    // Create application state
    let state = Arc::new(AppState { db });

    // Build router with API endpoints
    let router = Router::new()
        .route("/", get(health_check))
        .route("/uploads", post(create_upload))
        .route("/uploads", get(get_uploads))
        .route("/uploads/:id", get(get_upload_by_id))
        .with_state(state);

    Ok(router.into())
}
