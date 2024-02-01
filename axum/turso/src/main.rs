use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use libsql::Connection;
use serde::{Deserialize, Serialize};

async fn get_posts(State(client): State<Arc<Connection>>) -> Json<Vec<User>> {
    let mut rows = client
        .query("select * from example_users", ())
        .await
        .unwrap();
    let mut users = vec![];
    while let Some(row) = rows.next().unwrap() {
        users.push(User {
            uid: row.get::<String>(0).unwrap(),
            email: row.get::<String>(1).unwrap(),
        });
    }
    Json(users)
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    uid: String,
    email: String,
}

async fn create_users(
    State(client): State<Arc<Connection>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    client
        .execute(
            "insert into example_users (uid, email) values (?1, ?2)",
            [user.uid, user.email],
        )
        .await
        .unwrap();

    Json(serde_json::json!({ "ok": true }))
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_turso::Turso(addr = "libsql://your-db.turso.io", token = "{secrets.TURSO_DB_TOKEN}")]
    client: Connection,
) -> shuttle_axum::ShuttleAxum {
    let client = Arc::new(client);

    client
        .execute(
            "create table if not exists example_users ( uid text primary key, email text );",
            (),
        )
        .await
        .unwrap();

    let router = Router::new()
        .route("/", get(get_posts).post(create_users))
        .with_state(client);

    Ok(router.into())
}
