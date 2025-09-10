use sqlx::{Pool, Postgres};
use anyhow::Result;

use crate::models::{Todo, UpdateTodoRequest};

#[derive(Clone)]
pub struct TodoRepository {
    pool: Pool<Postgres>,
}

impl TodoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_todo(&self, title: &str) -> Result<Todo> {
        let row = sqlx::query!(
            "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed, created_at",
            title
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Todo {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at,
        })
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>> {
        let rows = sqlx::query!("SELECT id, title, completed, created_at FROM todos ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.id,
                title: row.title,
                completed: row.completed,
                created_at: row.created_at,
            })
            .collect();

        Ok(todos)
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Result<Option<Todo>> {
        let row = sqlx::query!("SELECT id, title, completed, created_at FROM todos WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(Todo {
                id: row.id,
                title: row.title,
                completed: row.completed,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_todo(&self, id: i32, update: UpdateTodoRequest) -> Result<Option<Todo>> {
        let existing = self.get_todo_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();
        
        let new_title = update.title.map(|t| t.trim().to_string()).unwrap_or(existing.title);
        let new_completed = update.completed.unwrap_or(existing.completed);

        let row = sqlx::query!(
            "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed, created_at",
            new_title,
            new_completed,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(Todo {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at,
        }))
    }

    pub async fn delete_todo(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}