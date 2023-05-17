use axum::{routing::get, Router};
use rusqlite::Connection;
use shuttle_runtime::tracing::info;
use std::sync::Arc;
use tokio::sync::Mutex;

// TODO: sqlite in memory
// TODO: sqlite in StaticFile provider
// TODO: sqlite in provier

async fn hello_world(
    connection: axum::extract::Extension<Arc<Mutex<Connection>>>,
) -> impl axum::response::IntoResponse {
    let c = connection.lock().await;

    let mut stmt = c.prepare("SELECT x FROM foo").unwrap();

    let items = stmt
        .query_map([], |row| {
            let x: i32 = row.get(0).unwrap();
            Ok(x)
        })
        .unwrap();

    (
        axum::http::StatusCode::OK,
        [("content-type", "text/plain")],
        items
            .into_iter()
            .map(|x| format!("value from db: {:?}\n", x.unwrap()))
            .collect::<String>(),
    )
}
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let conn = Connection::open("./db.sqlite").unwrap();
    // open_in_memory().unwrap();

    let connection = Arc::new(Mutex::new(conn));

    let c = connection.lock().await;
    let res = c.execute_batch("CREATE TABLE foo(x INTEGER)");
    match res {
        Ok(_) => info!("created table"),
        Err(_) => info!("table already exists"),
    }

    let res = c.execute("INSERT INTO foo (x) VALUES (1)", ());
    match res {
        Ok(rows) => info!("inserted {:?} rows", rows),
        Err(err) => info!("error inserting: {:?}", err),
    }

    let router = Router::new()
        .route("/", get(hello_world))
        .layer(axum::extract::Extension(connection.clone()));

    Ok(router.into())
}
