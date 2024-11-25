pub mod auth;
pub mod openai;

pub async fn health_check() -> &'static str {
    "Hello, world!"
}
