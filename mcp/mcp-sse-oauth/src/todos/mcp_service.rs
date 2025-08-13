use super::db::{self, CreateTodoRequest, ListTodosFilter, UpdateTodoRequest};
use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoService {
    db_pool: Arc<PgPool>,
    tool_router: ToolRouter<TodoService>,
    client_id: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateTodoInput {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetTodoInput {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListTodosInput {
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpdateTodoInput {
    pub id: String,
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DeleteTodoInput {
    pub id: String,
}

#[tool_router]
impl TodoService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            db_pool,
            tool_router: Self::tool_router(),
            client_id: Arc::new(Mutex::new(None)),
        }
    }

    #[tool(description = "Create a new todo item")]
    async fn create_todo(
        &self,
        Parameters(input): Parameters<CreateTodoInput>,
    ) -> Result<CallToolResult, McpError> {
        let client_id = {
            let reader = self.client_id.lock().await;
            reader.clone().ok_or_else(|| {
                McpError::internal_error("Client not authenticated".to_string(), None)
            })?
        };

        let request = CreateTodoRequest {
            title: input.title,
            completed: input.completed,
        };

        match db::create_todo(&self.db_pool, request, &client_id).await {
            Ok(todo) => {
                let todo_json = serde_json::to_string_pretty(&todo).map_err(|e| {
                    McpError::internal_error(format!("Serialization error: {e}"), None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Todo created successfully:\n{todo_json}"
                ))]))
            }
            Err(e) => Err(McpError::internal_error(
                format!("Failed to create todo: {e}"),
                None,
            )),
        }
    }

    #[tool(description = "Get a specific todo by ID")]
    async fn get_todo(
        &self,
        Parameters(input): Parameters<GetTodoInput>,
    ) -> Result<CallToolResult, McpError> {
        let id = input
            .id
            .parse::<Uuid>()
            .map_err(|e| McpError::invalid_params(format!("Invalid UUID format: {e}"), None))?;

        let client_id = {
            let reader = self.client_id.lock().await;
            reader.clone().ok_or_else(|| {
                McpError::internal_error("Client not authenticated".to_string(), None)
            })?
        };

        match db::get_todo_by_id(&self.db_pool, id, &client_id).await {
            Ok(Some(todo)) => {
                let todo_json = serde_json::to_string_pretty(&todo).map_err(|e| {
                    McpError::internal_error(format!("Serialization error: {e}"), None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(todo_json)]))
            }
            Ok(None) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Todo with ID {id} not found or not accessible"
            ))])),
            Err(e) => Err(McpError::internal_error(
                format!("Failed to get todo: {e}"),
                None,
            )),
        }
    }

    #[tool(description = "List todos with optional filtering")]
    async fn list_todos(
        &self,
        Parameters(input): Parameters<ListTodosInput>,
    ) -> Result<CallToolResult, McpError> {
        let client_id = {
            let reader = self.client_id.lock().await;
            reader.clone().ok_or_else(|| {
                McpError::internal_error("Client not authenticated".to_string(), None)
            })?
        };

        let filter = if input.completed.is_some() {
            Some(ListTodosFilter {
                completed: input.completed,
            })
        } else {
            None
        };

        match db::list_todos(&self.db_pool, &client_id, filter).await {
            Ok(todos) => {
                let todos_json = serde_json::to_string_pretty(&todos).map_err(|e| {
                    McpError::internal_error(format!("Serialization error: {e}"), None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Found {} todos:\n{}",
                    todos.len(),
                    todos_json
                ))]))
            }
            Err(e) => Err(McpError::internal_error(
                format!("Failed to list todos: {e}"),
                None,
            )),
        }
    }

    #[tool(description = "Update an existing todo")]
    async fn update_todo(
        &self,
        Parameters(input): Parameters<UpdateTodoInput>,
    ) -> Result<CallToolResult, McpError> {
        let id = input
            .id
            .parse::<Uuid>()
            .map_err(|e| McpError::invalid_params(format!("Invalid UUID format: {e}"), None))?;

        let client_id = {
            let reader = self.client_id.lock().await;
            reader.clone().ok_or_else(|| {
                McpError::internal_error("Client not authenticated".to_string(), None)
            })?
        };

        let request = UpdateTodoRequest {
            title: input.title,
            completed: input.completed,
        };

        match db::update_todo(&self.db_pool, id, &client_id, request).await {
            Ok(Some(todo)) => {
                let todo_json = serde_json::to_string_pretty(&todo).map_err(|e| {
                    McpError::internal_error(format!("Serialization error: {e}"), None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Todo updated successfully:\n{todo_json}"
                ))]))
            }
            Ok(None) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Todo with ID {id} not found or not accessible"
            ))])),
            Err(e) => Err(McpError::internal_error(
                format!("Failed to update todo: {e}"),
                None,
            )),
        }
    }

    #[tool(description = "Delete a todo by ID")]
    async fn delete_todo(
        &self,
        Parameters(input): Parameters<DeleteTodoInput>,
    ) -> Result<CallToolResult, McpError> {
        let id = input
            .id
            .parse::<Uuid>()
            .map_err(|e| McpError::invalid_params(format!("Invalid UUID format: {e}"), None))?;

        let client_id = {
            let reader = self.client_id.lock().await;
            reader.clone().ok_or_else(|| {
                McpError::internal_error("Client not authenticated".to_string(), None)
            })?
        };

        match db::delete_todo(&self.db_pool, id, &client_id).await {
            Ok(true) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Todo with ID {id} deleted successfully"
            ))])),
            Ok(false) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Todo with ID {id} not found or not accessible"
            ))])),
            Err(e) => Err(McpError::internal_error(
                format!("Failed to delete todo: {e}"),
                None,
            )),
        }
    }
}

#[tool_handler]
impl ServerHandler for TodoService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides todo management tools. You can create, read, update, and delete todos. Each todo has an id, title, and completion status.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            if let Some(client_id) = http_request_part.extensions.get::<String>() {
                let mut writer = self.client_id.lock().await;
                *writer = Some(client_id.clone());
            } else {
                tracing::warn!("No client_id found in HTTP request extensions");
            }
        }
        Ok(self.get_info())
    }
}
