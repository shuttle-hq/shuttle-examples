use std::sync::Arc;

use axum::{extract, extract::State, response::IntoResponse, routing::get, Json, Router};
use libsql_client::{args, Client, Row, Statement, Value};
use serde::{Deserialize, Serialize};

fn row_string_field(r: &Row, index: usize) -> String {
    match r.values.get(index).unwrap() {
        Value::Text { value } => value.clone(),
        _ => unreachable!(),
    }
}

async fn get_posts(State(client): State<Arc<Client>>) -> Json<Vec<User>> {
    let rows = client.execute("select * from example_users").await.unwrap();
    let users: Vec<_> = rows
        .rows
        .iter()
        .map(|r| User {
            uid: row_string_field(r, 0),
            email: row_string_field(r, 1),
        })
        .collect();
    Json(users)
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    uid: String,
    email: String,
}

async fn create_users(
    State(client): State<Arc<Client>>,
    extract::Json(user): extract::Json<User>,
) -> impl IntoResponse {
    client
        .execute(Statement::with_args(
            "insert into example_users values (?, ?)",
            args!(user.uid, user.email),
        ))
        .await
        .unwrap();

    Json(serde_json::json!({ "ok": true }))
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_turso::Turso(addr = "libsql://your-db.turso.io", token = "{secrets.TURSO_DB_TOKEN}")]
    client: Client,
) -> shuttle_axum::ShuttleAxum {
    let client = Arc::new(client);

    client
        .execute("create table if not exists example_users ( uid text primary key, email text );")
        .await
        .unwrap();

    let router = Router::new()
        .route("/", get(get_posts).post(create_users))
        .with_state(client);

    Ok(router.into())
}
