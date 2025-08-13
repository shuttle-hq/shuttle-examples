use sqlx::PgPool;
use std::net::SocketAddr;

mod auth;
mod init;
mod middleware;
mod todo_mcp;
mod todos;

struct McpSseService {
    pool: PgPool,
    secrets: shuttle_runtime::SecretStore,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for McpSseService {
    async fn bind(self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        init::init(addr, self.pool, self.secrets).await
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:password@localhost:5432/mcp-sse-auth"
    )]
    pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> Result<McpSseService, shuttle_runtime::Error> {
    Ok(McpSseService { pool, secrets })
}
