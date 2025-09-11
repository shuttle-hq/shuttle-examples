use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};

use super::AppError;
use crate::{
    AppState,
    models::{CreateTodoRequest, UpdateTodoRequest},
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    let todo = state.repo.create_todo(payload.title.trim()).await?;
    Ok((StatusCode::CREATED, Json(json!(todo))))
}

pub async fn get_todos(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let todos = state.repo.get_all_todos().await?;
    Ok(Json(json!(todos)))
}

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let todo = state
        .repo
        .get_todo_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;
    Ok(Json(json!(todo)))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Value>, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    let todo = state
        .repo
        .update_todo(id, payload)
        .await?
        .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;
    Ok(Json(json!(todo)))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let deleted = state.repo.delete_todo(id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Todo not found".to_string()))
    }
}
