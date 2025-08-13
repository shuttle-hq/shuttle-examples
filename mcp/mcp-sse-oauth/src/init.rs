use crate::{auth::get_auth_router, middleware::validate_token_middleware, todo_mcp::TodoService};
use axum::{middleware, Router};
use rmcp::transport::{sse_server::SseServerConfig, SseServer};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub secrets: shuttle_runtime::SecretStore,
}

impl AppState {
    pub fn new(pool: PgPool, secrets: shuttle_runtime::SecretStore) -> Self {
        Self { pool, secrets }
    }
}

pub async fn init(
    addr: SocketAddr,
    pool: PgPool,
    secrets: shuttle_runtime::SecretStore,
) -> Result<(), shuttle_runtime::Error> {
    // Run database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|e| shuttle_runtime::Error::Custom(anyhow::anyhow!("Migration failed: {}", e)))?;

    // Create the OAuth store
    let app_state = Arc::new(AppState::new(pool, secrets));

    // Create SSE server configuration for MCP
    let sse_config = SseServerConfig {
        bind: addr,
        sse_path: "/mcp/sse".to_string(),
        post_path: "/mcp/message".to_string(),
        ct: CancellationToken::new(),
        sse_keep_alive: Some(Duration::from_secs(15)),
    };

    // Create SSE server
    let (sse_server, sse_router) = SseServer::new(sse_config);

    // Create protected SSE routes (require authorization)
    let protected_sse_router = sse_router.layer(middleware::from_fn_with_state(
        app_state.clone(),
        validate_token_middleware,
    ));

    // Create CORS layer for the oauth authorization server endpoint
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let auth_router = get_auth_router();

    // Create HTTP router with auth routes (non-protected) and protected SSE router
    let app = Router::new()
        .merge(auth_router)
        .with_state(app_state.clone())
        .merge(protected_sse_router)
        .layer(cors_layer);
    // Register token validation middleware for SSE
    let cancel_token = sse_server.config.ct.clone();
    // Handle Ctrl+C
    let cancel_token2 = sse_server.config.ct.clone();
    // Start SSE server with TodoService
    sse_server.with_service(move || TodoService::new(Arc::new(app_state.pool.clone())));

    // Start HTTP server
    info!("MCP OAuth Server started on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        cancel_token.cancelled().await;
        info!("Server is shutting down");
    });

    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C, shutting down");
                cancel_token2.cancel();
            }
            Err(e) => error!("Failed to listen for Ctrl+C: {}", e),
        }
    });

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    Ok(())
}
