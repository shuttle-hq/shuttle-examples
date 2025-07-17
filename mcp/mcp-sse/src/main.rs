use std::net::SocketAddr;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use tokio_util::sync::CancellationToken;

mod mcp_service;
use mcp_service::Counter;

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<McpSseService, shuttle_runtime::Error> {
    Ok(McpSseService {})
}

struct McpSseService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for McpSseService {
    async fn bind(self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        tracing::info!("Starting SSE MCP Server on {}", addr);

        // Create SSE server configuration
        let config = SseServerConfig {
            bind: addr,
            sse_path: "/sse".to_string(),
            post_path: "/message".to_string(),
            ct: CancellationToken::new(),
            sse_keep_alive: Some(std::time::Duration::from_secs(30)),
        };

        // Create SSE server
        let (sse_server, router) = SseServer::new(config);

        // Create TCP listener
        let listener = tokio::net::TcpListener::bind(addr).await
            .map_err(|e| shuttle_runtime::Error::Custom(anyhow::anyhow!("Failed to bind: {}", e).into()))?;

        // Setup graceful shutdown
        let ct = sse_server.config.ct.child_token();
        let server = axum::serve(listener, router).with_graceful_shutdown(async move {
            ct.cancelled().await;
            tracing::info!("SSE server shutting down gracefully");
        });

        // Start the HTTP server in a background task
        tokio::spawn(async move {
            if let Err(e) = server.await {
                tracing::error!("SSE server error: {}", e);
            }
        });

        // Register the Counter service with the SSE server
        let ct = sse_server.with_service(Counter::new);

        tracing::info!("SSE MCP Server started successfully");
        tracing::info!("SSE endpoint: http://{}/sse", addr);
        tracing::info!("Message endpoint: http://{}/message", addr);

        // Wait for shutdown signal
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received shutdown signal");
            }
            _ = ct.cancelled() => {
                tracing::info!("Service cancelled");
            }
        }

        ct.cancel();
        tracing::info!("SSE MCP Server shut down");
        Ok(())
    }
}
