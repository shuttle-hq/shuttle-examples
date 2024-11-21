use derive_more::{Display, Error, From};

#[derive(Display, Debug, From, Error)]
pub enum AppError {
    #[display("{_0}")]
    SQL(sqlx::Error),
    #[display("{_0}")]
    OpenAI(shuttle_openai::async_openai::error::OpenAIError),
}
