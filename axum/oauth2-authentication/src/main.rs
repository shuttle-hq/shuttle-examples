use axum::{
    extract::FromRef,
    routing::{get},
    Router,
    Extension,
    response::Html
};
use axum_extra::extract::cookie::Key;
use reqwest::Client;
use routes::oauth;
use axum::middleware;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use oauth2::{AuthUrl, TokenUrl, ClientId, RedirectUrl, basic::BasicClient, ClientSecret};



pub mod routes;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    ctx: Client,
    key: Key
}


// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.expect("Failed migrations :(");

    let oauth_id = secrets.get("GOOGLE_OAUTH_CLIENT_ID").unwrap();
    let oauth_secret = secrets.get("GOOGLE_OAUTH_CLIENT_SECRET").unwrap();

    let ctx = Client::new();

    let state = AppState {
        db,
        ctx,
        key: Key::generate()
    };

    let oauth_client = build_oauth_client(oauth_id.clone(), oauth_secret);

    let router = init_router(state, oauth_client, oauth_id);

    Ok(router.into())
}

fn init_router(state: AppState, oauth_client: BasicClient, oauth_id: String) -> Router {
let auth_router = Router::new()
    .route("/auth/google_callback", get(oauth::google_callback));

let protected_router = Router::new()
    .route("/", get(oauth::protected))
    .route_layer(middleware::from_fn_with_state(state.clone(), oauth::check_authenticated));

let homepage_router = Router::new()
    .route("/", get(homepage))
    .layer(Extension(oauth_id));

Router::new()
    .nest("/api", auth_router)
    .nest("/protected", protected_router)
    .nest("/", homepage_router)
    .layer(Extension(oauth_client))
    .with_state(state)


}

fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let redirect_url = "http://localhost:8000/api/auth/google_callback".to_string();

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[axum::debug_handler]
async fn homepage(
    Extension(oauth_id): Extension<String>
) -> Html<String> {
    Html(format!("<p>Welcome!</p>
    
    <a href=\"https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={oauth_id}&response_type=code&redirect_uri=http://localhost:8000/api/auth/google_callback\">
    Click here to sign into Google!
     </a>"))

    
}
