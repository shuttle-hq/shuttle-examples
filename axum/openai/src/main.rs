use std::sync::Arc;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};

struct AppState {
    openai: Client<OpenAIConfig>,
}

async fn chat(State(state): State<Arc<AppState>>, Json(v): Json<Value>) -> Result<Json<Value>, ()> {
    let user_msg: ChatCompletionRequestMessage = ChatCompletionRequestUserMessageArgs::default()
        .content(v["message"].as_str().unwrap())
        .build()
        .unwrap()
        .into();
    let req = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages(vec![user_msg])
        .n(1)
        .build()
        .unwrap();
    let res = state.openai.chat().create(req).await.unwrap();
    let reply = res
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap()
        .to_owned();

    Ok(Json(json!({ "response": reply })))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_openai::OpenAI(api_key = "{secrets.OPENAI_API_KEY}")] openai: Client<OpenAIConfig>,
) -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(AppState { openai });

    let router = Router::new().route("/", post(chat)).with_state(state);

    Ok(router.into())
}
