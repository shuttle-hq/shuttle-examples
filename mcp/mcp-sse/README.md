# MCP SSE Server Template

A Model Context Protocol (MCP) server with Server-Sent Events transport, deployed on **Shuttle**.

## Features

- MCP server with SSE transport
- Counter service with `increment`, `decrement`, `get_value`, and `reset` operations
- Ready-to-deploy on Shuttle

## Quick Start

```bash
# Run locally
cargo shuttle run

# Deploy to Shuttle
cargo shuttle deploy
```

## Endpoints

- `http://localhost:8000/sse` - SSE endpoint for MCP client connections
- `http://localhost:8000/message` - Message endpoint

## Tools

- `increment` - Increment counter by 1
- `decrement` - Decrement counter by 1
- `get_value` - Get current counter value
- `reset` - Reset counter to zero
