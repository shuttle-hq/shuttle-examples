use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use libsql::Database;
use serde::{Deserialize, Serialize};

async fn get_posts(State(client): State<Arc<Database>>) -> Json<Vec<User>> {
    let conn = client.connect().unwrap();

    let mut rows = conn.query("select * from example_users", ()).await.unwrap();
    let mut users = vec![];
    while let Some(row) = rows.next().await.unwrap() {
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
    State(client): State<Arc<Database>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let conn = client.connect().unwrap();
    conn.execute(
        "insert into example_users (uid, email) values (?1, ?2)",
        [user.uid, user.email],
    )
    .await
    .unwrap();

    Json(serde_json::json!({ "ok": true }))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_turso::Turso(addr = "libsql://your-db.turso.io", token = "{secrets.TURSO_DB_TOKEN}")]
    client: Database,
) -> shuttle_axum::ShuttleAxum {
    let client = Arc::new(client);
    let conn = client.connect().unwrap();

    conn.execute(
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
