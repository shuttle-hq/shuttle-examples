use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use shuttle_runtime::tracing::info;

// TODO: sqlite in memory
// TODO: sqlite in StaticFile provider
// TODO: sqlite in provier

async fn hello_world(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let _pool = &state.pool;

    "Hi there".to_string()
}

pub struct AppState {
    pool: shuttle_sqlite::SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_sqlite::SQLite(db_name = "my_sqlite.db")] pool: shuttle_sqlite::SqlitePool,
) -> shuttle_axum::ShuttleAxum {
    let mut conn = pool.acquire().await.unwrap();

    let res = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users(id int, name varchar(128), email varchar(128));",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let id = 42;
    let name = "ferris";
    let email = "foo@bar.com";

    let res = sqlx::query("INSERT INTO users(id, name, email) VALUES (?, ?, ?);")
        .bind(id)
        .bind(name)
        .bind(email)
        .execute(&pool)
        .await
        .expect("Failed to insert user");

    let mut res = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    info!("Retrieved {res:?}");

    let state = Arc::new(AppState { pool: pool.clone() });
    let router = axum::Router::new();

    Ok(router.into())
}
