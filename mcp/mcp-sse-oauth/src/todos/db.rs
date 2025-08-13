use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTodosFilter {
    pub completed: Option<bool>,
}

pub async fn create_todo(
    pool: &PgPool,
    request: CreateTodoRequest,
    client_id: &str,
) -> Result<Todo, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO todos (title, completed, client_id)
        VALUES ($1, COALESCE($2, FALSE), $3)
        RETURNING id, title, completed
        "#,
        request.title,
        request.completed,
        client_id
    )
    .fetch_one(pool)
    .await?;

    Ok(Todo {
        id: row.id,
        title: row.title,
        completed: row.completed,
    })
}

pub async fn list_todos(
    pool: &PgPool,
    client_id: &str,
    filter: Option<ListTodosFilter>,
) -> Result<Vec<Todo>, sqlx::Error> {
    let mut query = "SELECT id, title, completed FROM todos WHERE client_id = $1".to_string();
    let param_count = 2;

    if let Some(f) = &filter {
        if f.completed.is_some() {
            query.push_str(&format!(" AND completed = ${param_count}"));
        }
    }

    query.push_str(" ORDER BY id DESC");

    let mut query_builder = sqlx::query(&query).bind(client_id);

    if let Some(f) = &filter {
        if let Some(completed) = f.completed {
            query_builder = query_builder.bind(completed);
        }
    }

    let rows = query_builder.fetch_all(pool).await?;

    let todos = rows
        .into_iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get("completed"),
        })
        .collect();

    Ok(todos)
}

pub async fn update_todo(
    pool: &PgPool,
    id: Uuid,
    client_id: &str,
    request: UpdateTodoRequest,
) -> Result<Option<Todo>, sqlx::Error> {
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if request.title.is_some() {
        set_clauses.push(format!("title = ${param_count}"));
        param_count += 1;
    }
    if request.completed.is_some() {
        set_clauses.push(format!("completed = ${param_count}"));
        param_count += 1;
    }

    if set_clauses.is_empty() {
        let row = sqlx::query!(
            r#"
            SELECT id, title, completed
            FROM todos
            WHERE id = $1 AND client_id = $2
            "#,
            id,
            client_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(row.map(|r| Todo {
            id: r.id,
            title: r.title,
            completed: r.completed,
        }));
    }

    let query = format!(
        "UPDATE todos SET {} WHERE id = ${} AND client_id = ${} RETURNING id, title, completed",
        set_clauses.join(", "),
        param_count,
        param_count + 1
    );

    let mut query_builder = sqlx::query(&query);

    if let Some(title) = request.title {
        query_builder = query_builder.bind(title);
    }
    if let Some(completed) = request.completed {
        query_builder = query_builder.bind(completed);
    }
    query_builder = query_builder.bind(id).bind(client_id);

    let row = query_builder.fetch_optional(pool).await?;

    Ok(row.map(|r| Todo {
        id: r.get("id"),
        title: r.get("title"),
        completed: r.get("completed"),
    }))
}

pub async fn delete_todo(pool: &PgPool, id: Uuid, client_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE id = $1 AND client_id = $2",
        id,
        client_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_todo_by_id(
    pool: &PgPool,
    id: Uuid,
    client_id: &str,
) -> Result<Option<Todo>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, title, completed
        FROM todos
        WHERE id = $1 AND client_id = $2
        "#,
        id,
        client_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Todo {
        id: r.id,
        title: r.title,
        completed: r.completed,
    }))
}
