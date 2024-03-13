use crate::{contents::File, errors::EmbeddingError, errors::SetupError};
use anyhow::Result;
use openai::{
    chat::{ChatCompletion, ChatCompletionBuilder, ChatCompletionDelta, ChatCompletionMessage},
    embeddings::{Embedding, Embeddings},
};
use shuttle_secrets::SecretStore;
use tokio::sync::mpsc::Receiver;

type Conversation = Receiver<ChatCompletionDelta>;

pub fn setup(secrets: &SecretStore) -> Result<()> {
    // This sets OPENAI_API_KEY as API_KEY for use with all openai crate functions
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(SetupError("OPENAI Key not available"))?;
    openai::set_key(openai_key);
    Ok(())
}

pub async fn embed_file(file: &File) -> Result<Embeddings> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    Embeddings::create("text-embedding-ada-002", sentence_as_str, "stefan")
        .await
        .map_err(|_| EmbeddingError {}.into())
}

pub async fn embed_sentence(prompt: &str) -> Result<Embedding> {
    Embedding::create("text-embedding-ada-002", prompt, "stefan")
        .await
        .map_err(|_| EmbeddingError {}.into())
}

pub async fn chat_stream(prompt: &str, contents: &str) -> Result<Conversation> {
    let content = format!("{}\n Context: {}\n Be concise", prompt, contents);

    ChatCompletionBuilder::default()
        .model("gpt-3.5-turbo")
        .temperature(0.0)
        .user("stefan")
        .messages(vec![ChatCompletionMessage {
            role: openai::chat::ChatCompletionMessageRole::User,
            content,
            name: Some("stefan".to_string()),
        }])
        .create_stream()
        .await
        .map_err(|_| EmbeddingError {}.into())
}

pub async fn _chat(prompt: &str, contents: &str) -> Result<ChatCompletion> {
    let content = format!("{}\n Context: {}\n Be concise", prompt, contents);

    ChatCompletionBuilder::default()
        .model("gpt-3.5-turbo")
        .temperature(0.0)
        .user("stefan")
        .messages(vec![ChatCompletionMessage {
            role: openai::chat::ChatCompletionMessageRole::User,
            content,
            name: Some("stefan".to_string()),
        }])
        .create()
        .await
        .map_err(|_| EmbeddingError {}.into())
}
