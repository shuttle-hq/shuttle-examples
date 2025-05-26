// src/bin/main.rs

use shuttle_runtime::CustomError;
use telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use startup::Application;

mod routes;
mod startup;
mod telemetry;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("url-shortener-v1".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // run the database migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|err| {
            let msg = format!("Unable to run the database migrations: {}", err);
            CustomError::new(err).context(msg)
        })?;

    tracing::info!("Building the application...");
    let Application(router) = Application::build(pool);

    Ok(router.into())
}
