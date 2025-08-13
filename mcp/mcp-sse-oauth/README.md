# MCP SSE OAuth Server (Shuttle)

A Model Context Protocol (MCP) server that provides secure todo management functionality over Server-Sent Events (SSE), protected by OAuth2 with PKCE (Proof Key for Code Exchange). The server includes a complete authentication system and todo management tools backed by PostgreSQL. Designed to run both locally and on Shuttle.

## Features

- **OAuth2 PKCE Authentication**: Secure authentication flow with PKCE support
- **Server-Sent Events (SSE)**: Real-time MCP communication over SSE transport
- **PostgreSQL Integration**: Persistent storage with automatic migrations
- **Production Ready**: Deployable on Shuttle with proper secrets management

## Architecture

The server consists of several key components:

- **Authentication Layer**: OAuth2 server with PKCE support (`src/auth/`)
- **MCP Service**: Todo management tools over MCP protocol (`src/todos/mcp_service.rs`)
- **SSE Transport**: Real-time communication layer (`src/init.rs`)
- **Database Layer**: PostgreSQL with SQLx for data persistence (`src/todos/db.rs`)
- **Middleware**: Token validation and request authentication (`src/middleware.rs`)

## MCP Tools

The server exposes the following MCP tools for authenticated clients:

- `create_todo` - Create a new todo item
- `get_todo` - Retrieve a specific todo by ID
- `list_todos` - List todos with optional filtering by completion status
- `update_todo` - Update an existing todo's title or completion status
- `delete_todo` - Delete a todo by ID

All tools are scoped to the authenticated client's session, ensuring data isolation between different OAuth clients.

## Setup and Development

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (for local development)
- Shuttle CLI (for deployment)

### Local Development

Run the following commands to get started:

```bash
# Initialize the project
shuttle init --from shuttle-hq/shuttle-examples --subfolder mcp/mcp-sse-oauth

# Navigate to the project directory
cd mcp-sse-oauth

# Run the development server
shuttle run

# Deploy the server
shuttle deploy
```

## API Endpoints

### OAuth Endpoints

| Endpoint                                        | Method | Description                         |
| ----------------------------------------------- | ------ | ----------------------------------- |
| `/oauth/register`                               | POST   | Register a new OAuth client         |
| `/oauth/authorize`                              | GET    | OAuth authorization endpoint        |
| `/oauth/token`                                  | POST   | Token exchange and refresh endpoint |
| `/oauth/.well-known/oauth-authorization-server` | GET    | OAuth server metadata               |

### MCP Endpoints

| Endpoint       | Method | Description                                       |
| -------------- | ------ | ------------------------------------------------- |
| `/mcp/sse`     | GET    | Server-Sent Events endpoint for MCP communication |
| `/mcp/message` | POST   | HTTP endpoint for MCP message posting             |
