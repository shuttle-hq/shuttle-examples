use axum::extract::FromRef;
use axum::Router;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use tower_http::services::{ServeDir, ServeFile};

mod auth;
mod customers;
mod dashboard;
mod deals;
mod mail;
mod payments;
mod router;

use router::create_api_router;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
    pub stripe_key: String,
    pub stripe_sub_price: String,
    pub mailgun_key: String,
    pub mailgun_url: String,
    pub domain: String,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Failed to run migrations");

    let (stripe_key, stripe_sub_price, mailgun_key, mailgun_url, domain) = grab_secrets(secrets);

    let state = AppState {
        postgres,
        stripe_key,
        stripe_sub_price,
        mailgun_key,
        mailgun_url,
        domain,
        key: Key::generate(),
    };

    let api_router = create_api_router(state);

    let router = Router::new().nest("/api", api_router).nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    );

    Ok(router.into())
}

fn grab_secrets(secrets: shuttle_runtime::SecretStore) -> (String, String, String, String, String) {
    let stripe_key = secrets
        .get("STRIPE_KEY")
        .unwrap_or_else(|| "None".to_string());

    let stripe_sub_price = secrets
        .get("STRIPE_SUB_PRICE")
        .unwrap_or_else(|| "None".to_string());

    let mailgun_key = secrets
        .get("MAILGUN_KEY")
        .unwrap_or_else(|| "None".to_string());

    let mailgun_url = secrets
        .get("MAILGUN_URL")
        .unwrap_or_else(|| "None".to_string());

    let domain = secrets
        .get("DOMAIN_URL")
        .unwrap_or_else(|| "None".to_string());

    (
        stripe_key,
        stripe_sub_price,
        mailgun_key,
        mailgun_url,
        domain,
    )
}
