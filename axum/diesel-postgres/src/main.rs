use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::get, Router};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use serde::{Deserialize, Serialize};
use shuttle_shared_db::PostgresDiesel;

mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Todo {
    id: i32,
    note: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = schema::todos)]
pub struct NewTodo {
    note: String,
}

#[shuttle_runtime::main]
async fn axum(#[PostgresDiesel] pool: Pool) -> shuttle_axum::ShuttleAxum {
    {
        let conn = pool.get().await.expect("Couldn't establish connection");
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(drop))
            .await
            .expect("Migration failed")
            .expect("Migration failed");
    }

    let router = Router::new()
        .route("/todos", get(receive).post(create))
        .with_state(pool);

    Ok(router.into())
}

async fn receive(State(pool): State<Pool>) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    tracing::info!("get recv");
    let todo = pool
        .get()
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Couldn't establish connection\n\n{e:#?}"),
            )
        })?
        .interact(|conn| {
            schema::todos::table
                .select(Todo::as_select())
                .load(conn)
                .map_err(|e| {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        format!("Couldn't query todos table\n\n{e:#?}"),
                    )
                })
        })
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Interaction with database failed\n\n{e:#?}"),
            )
        })??;

    Ok(Json(todo))
}

async fn create(
    State(pool): State<Pool>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    tracing::info!("add recv");
    let todo = pool
        .get()
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Couldn't establish connection\n\n{e:#?}"),
            )
        })?
        .interact(|conn| {
            diesel::insert_into(schema::todos::table)
                .values(new_todo)
                .returning(Todo::as_returning())
                .get_result(conn)
                .map_err(|e| {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        format!("Insertion of new todo failed\n\n{e:#?}"),
                    )
                })
        })
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Interaction with database failed\n\n{e:#?}"),
            )
        })??;

    Ok(Json(todo))
}
