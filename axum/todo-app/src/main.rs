mod database;
mod handlers;
mod models;

use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use database::TodoRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: TodoRepository,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let repo = TodoRepository::new(pool);
    let state = AppState { repo };

    let app = Router::new()
        .route(
            "/api/todos",
            get(handlers::get_todos).post(handlers::create_todo),
        )
        .route(
            "/api/todos/{id}",
            get(handlers::get_todo)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app.into())
}
