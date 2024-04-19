use actix_web::{
    error, get,
    middleware::Logger,
    post,
    web::{self, Json, ServiceConfig},
    Result,
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool};

#[get("/{id}")]
async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as("SELECT * FROM todos WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[post("")]
async fn add(todo: web::Json<TodoNew>, state: web::Data<AppState>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&todo.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/todos")
                .wrap(Logger::default())
                .service(retrieve)
                .service(add)
                .app_data(state),
        );
    };

    Ok(config.into())
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}
