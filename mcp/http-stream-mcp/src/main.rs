use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Clone)]
struct TaskManager {
    tasks: Arc<Mutex<Vec<Task>>>,
    next_id: Arc<Mutex<usize>>,
    tool_router: ToolRouter<TaskManager>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddTaskRequest {
    #[schemars(description = "The title of the task")]
    title: String,
    #[schemars(description = "A detailed description of the task")]
    description: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CompleteTaskRequest {
    #[schemars(description = "The ID of the task to mark as completed")]
    id: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetTaskRequest {
    #[schemars(description = "The ID of the task to retrieve")]
    id: usize,
}

#[tool_router]
impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Add a new task to the task manager")]
    async fn add_task(
        &self,
        Parameters(AddTaskRequest { title, description }): Parameters<AddTaskRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut tasks = self.tasks.lock().await;
        let mut next_id = self.next_id.lock().await;

        let task = Task {
            id: *next_id,
            title,
            description,
            completed: false,
        };

        *next_id += 1;
        tasks.push(task.clone());

        let response = serde_json::json!({
            "success": true,
            "task": task,
            "message": format!("Task '{}' added successfully with ID {}", task.title, task.id)
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Mark a task as completed")]
    async fn complete_task(
        &self,
        Parameters(CompleteTaskRequest { id }): Parameters<CompleteTaskRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut tasks = self.tasks.lock().await;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            let response = serde_json::json!({
                "success": true,
                "task": task,
                "message": format!("Task '{}' marked as completed", task.title)
            });

            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&response).unwrap(),
            )]))
        } else {
            Err(McpError::invalid_params(
                format!("Task with ID {} not found", id),
                None,
            ))
        }
    }

    #[tool(description = "List all tasks in the task manager")]
    async fn list_tasks(&self) -> Result<CallToolResult, McpError> {
        let tasks = self.tasks.lock().await;

        let response = serde_json::json!({
            "total": tasks.len(),
            "tasks": tasks.clone()
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a specific task by ID")]
    async fn get_task(
        &self,
        Parameters(GetTaskRequest { id }): Parameters<GetTaskRequest>,
    ) -> Result<CallToolResult, McpError> {
        let tasks = self.tasks.lock().await;

        if let Some(task) = tasks.iter().find(|t| t.id == id) {
            let response = serde_json::json!({
                "success": true,
                "task": task
            });

            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&response).unwrap(),
            )]))
        } else {
            Err(McpError::invalid_params(
                format!("Task with ID {} not found", id),
                None,
            ))
        }
    }
}

#[tool_handler]
impl ServerHandler for TaskManager {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "task-manager".to_string(),
                version: "0.1.0".to_string(),
                title: None,
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "A task manager MCP server that allows you to add, complete, list, and retrieve tasks with real-time updates."
                    .to_string(),
            ),
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting Task Manager MCP Server");

    let service = rmcp::transport::streamable_http_server::StreamableHttpService::new(
        || Ok(TaskManager::new()),
        rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default()
            .into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);

    Ok(router.into())
}
