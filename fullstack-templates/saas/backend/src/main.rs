use axum::body::{boxed, Body};
use axum::extract::FromRef;
use axum::http::{Response, StatusCode};
use axum::routing::get;
use axum::Router;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use std::path::PathBuf;
use tower::ServiceExt;
use tower_http::services::ServeDir;

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
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
    #[shuttle_static_folder::StaticFolder(folder = "public")] public: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Had some errors running migrations :(");

    let (stripe_key, mailgun_key, mailgun_url, domain) = grab_secrets(secrets);

    let state = AppState {
        postgres,
        stripe_key,
        mailgun_key,
        mailgun_url,
        domain,
        key: Key::generate(),
    };

    let api_router = create_api_router(state);

    let router = Router::new()
        .nest("/api", api_router)
        .fallback_service(get(|req| async move {
            match ServeDir::new(public).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }));

    Ok(router.into())
}

fn grab_secrets(secrets: shuttle_secrets::SecretStore) -> (String, String, String, String) {
    let stripe_key = secrets
        .get("STRIPE_KEY")
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

    (stripe_key, mailgun_key, mailgun_url, domain)
}
