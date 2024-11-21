use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shuttle_openai::async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use sqlx::Row;
use sqlx::{postgres::PgRow, types::JsonValue};

use crate::state::AppState;

use super::auth::Claims;

#[derive(Deserialize)]
pub struct Request {
    prompt: String,
    model: String,
}

#[derive(Serialize)]
struct Response {
    conversation_id: i32,
    response: String,
}

impl Response {
    fn new(conversation_id: i32, response: String) -> Self {
        Self {
            conversation_id,
            response,
        }
    }
}

#[derive(sqlx::FromRow, Serialize)]
struct ConversationId(i32);

#[derive(Serialize)]
struct Conversation {
    messages: Vec<ChatMessage>,
}

impl Conversation {
    fn new(messages: Vec<ChatMessage>) -> Self {
        Self { messages }
    }
}

impl<'r> sqlx::FromRow<'r, PgRow> for Conversation {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let val: sqlx::types::JsonValue = row.try_get("conversation")?;

        let messages: Vec<ChatMessage> = serde_json::from_value(val).unwrap();

        Ok(Self::new(messages))
    }
}

#[derive(Deserialize, Serialize)]
struct ChatMessage {
    role: String,
    message: String,
}

pub async fn create_chat(State(state): State<AppState>, claims: Claims) -> impl IntoResponse {
    let ConversationId(convo_id) = sqlx::query_as(
        r#"INSERT INTO conversations
        (user_id)
        VALUES
        ($1)
        RETURNING id"#,
    )
    .bind(claims.user_id())
    .fetch_one(&state.db)
    .await
    .unwrap();

    Json(Response::new(convo_id, String::new()))
}

pub async fn get_conversation_list(
    State(state): State<AppState>,
    claims: Claims,
) -> impl IntoResponse {
    let conversation_ids: Vec<ConversationId> =
        sqlx::query_as("SELECT ID from conversations WHERE user_id = $1 order by id desc")
            .bind(claims.user_id())
            .fetch_all(&state.db)
            .await
            .unwrap();

    Json(conversation_ids)
}

pub async fn fetch_conversation_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<i32>,
    claims: Claims,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let conversation: Conversation = match sqlx::query_as(
        "SELECT CONVERSATION FROM conversations where id = $1 AND user_id = $2
        ",
    )
    .bind(conversation_id)
    .bind(claims.user_id())
    .fetch_one(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok((StatusCode::OK, Json(conversation)))
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<i32>,
    claims: Claims,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let conversation: Conversation = sqlx::query_as(
        "SELECT CONVERSATION FROM conversations where id = $1 AND user_id = $2
        ",
    )
    .bind(conversation_id)
    .bind(claims.user_id())
    .fetch_one(&state.db)
    .await
    .unwrap_or_else(|_| Conversation {
        messages: Vec::new(),
    });

    let mut chat_messages = Vec::new();

    for message in conversation.messages {
        let chat_message = match message.role.trim() {
            "system" => ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(message.message)
                    .build()
                    .map_err(|_| {
                        Err::<ChatCompletionRequestMessage, (StatusCode, String)>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Invalid chat role".to_string(),
                        ))
                    })
                    .unwrap(),
            ),

            "user" => ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(message.message)
                    .build()
                    .map_err(|_| {
                        Err::<ChatCompletionRequestMessage, (StatusCode, String)>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Invalid chat role".to_string(),
                        ))
                    })
                    .unwrap(),
            ),

            err => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid chat role".to_string(),
                ))
            }
        };

        chat_messages.push(chat_message)
    }

    chat_messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(req.prompt.to_owned())
            .build()
            .unwrap()
            .into(),
    );

    let chat_req = CreateChatCompletionRequestArgs::default()
        .model(&req.model)
        .messages(chat_messages)
        .n(1)
        .build()
        .unwrap();

    let res = state.openai_client.chat().create(chat_req).await.unwrap();
    let reply: String = res
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap()
        .to_owned();

    sqlx::query(
        r#"UPDATE conversations
        SET conversation = conversation || $1
        WHERE id = $2"#,
    )
    .bind(json!([{
        "role": "user",
        "message": &req.prompt,
    },
    {
        "role":"system",
        "message": &reply
    }]))
    .bind(conversation_id)
    .execute(&state.db)
    .await
    .unwrap();

    Ok(Json(Response::new(conversation_id, reply)))
}
