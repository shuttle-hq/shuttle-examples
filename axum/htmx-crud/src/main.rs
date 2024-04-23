use sqlx::PgPool;
mod errors;
mod models;
mod router;
mod routes;
mod templates;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = router::init_router(pool);

    Ok(router.into())
}
