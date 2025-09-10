use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

impl CreateTodoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.title.len() > 255 {
            return Err("Title cannot exceed 255 characters".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl UpdateTodoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("Title cannot be empty".to_string());
            }
            if title.len() > 255 {
                return Err("Title cannot exceed 255 characters".to_string());
            }
        }
        Ok(())
    }
}