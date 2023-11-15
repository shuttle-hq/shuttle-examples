#[macro_use]
extern crate rocket;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{Executor, FromRow, PgPool};

#[get("/<id>")]
async fn retrieve(id: i32, state: &State<MyState>) -> Result<Json<Todo>, BadRequest<String>> {
    let todo = sqlx::query_as("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[post("/", data = "<data>")]
async fn add(
    data: Json<TodoNew>,
    state: &State<MyState>,
) -> Result<Json<Todo>, BadRequest<String>> {
    let todo = sqlx::query_as("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

    Ok(Json(todo))
}

struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };
    let rocket = rocket::build()
        .mount("/todo", routes![retrieve, add])
        .manage(state);

    Ok(rocket.into())
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
