use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get};
use shuttle_runtime::tracing::info;
use shuttle_sqlite::{SQLite, SQLiteConnOpts, SqlitePool};

async fn db_ops(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.pool;

    let id = 42;
    let name = "ferris";
    let email = "foo@bar.com";

    let _ = sqlx::query("INSERT INTO users(id, name, email) VALUES (?, ?, ?);")
        .bind(id)
        .bind(name)
        .bind(email)
        .execute(pool)
        .await
        .expect("Failed to insert user");

    let ferris = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();

    let res = format!("Created and retrieved {:?}", ferris);
    info!(res);
    res
}

pub struct AppState {
    pool: shuttle_sqlite::SqlitePool,
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

#[shuttle_runtime::main]
async fn axum(
    #[SQLite(opts = SQLiteConnOpts::new().filename("custom.sqlite"))] pool: SqlitePool,
) -> shuttle_axum::ShuttleAxum {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users(id int, name varchar(128), email varchar(128));",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    info!("Table created. Navigate to localhost:8000 to query the database.");

    let state = Arc::new(AppState { pool: pool.clone() });
    let router = axum::Router::new()
        .route("/", get(db_ops))
        .with_state(state);

    Ok(router.into())
}
