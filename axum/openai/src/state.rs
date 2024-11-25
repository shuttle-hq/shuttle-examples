use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use shuttle_openai::async_openai::{config::OpenAIConfig, Client};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub openai_client: Client<OpenAIConfig>,
    key: Key,
}

impl AppState {
    pub async fn new(
        conn_string: String,
        openai_client: Client<OpenAIConfig>,
    ) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(&conn_string).await?;

        Ok(Self {
            db,
            openai_client,
            key: Key::generate(),
        })
    }

    pub async fn seed(&self) {
        sqlx::migrate!().run(&self.db).await.unwrap();
    }
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
