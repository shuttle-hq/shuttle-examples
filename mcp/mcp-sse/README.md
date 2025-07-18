# MCP SSE Server Template

A [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server template with Server-Sent Events transport, deployed on **Shuttle**. MCP servers enable AI agents to securely connect to external data sources and tools, providing real-time access to databases, APIs, and live data streams.

## Features

- MCP server with SSE transport
- Counter service with `increment`, `decrement`, `get_value`, and `reset` operations
- Ready-to-deploy on [Shuttle](https://www.shuttle.dev)

## Use Cases

This MCP SSE server template is ideal for:

- **AI Agent Integration**: Connect AI assistants like Claude, GPT, or local models to real-time data streams
- **Live Data Monitoring**: Stream counter values, metrics, or status updates to AI agents for real-time decision making
- **Interactive Applications**: Build chatbots or AI assistants that can interact with stateful services
- **Prototyping MCP Servers**: Use as a starting point for custom MCP servers with Server-Sent Events transport

## Quick Start

### Local Development

```bash
shuttle run
```

### Deploy to Shuttle

```bash
shuttle deploy
```

## Endpoints

- `http://localhost:8000/sse` - SSE endpoint for MCP client connections
- `http://localhost:8000/message` - Message endpoint

## Accessing the Deployment

After deployment, your MCP server will be available at:

- `https://your-project-name.shuttle.app/sse` - SSE endpoint for MCP client connections
- `https://your-project-name.shuttle.app/message` - Message endpoint

Replace `your-project-name` with the actual name of your deployed project (e.g., `mcp-z8dz.shuttle.app`).

## Cursor Integration

To use this MCP server with Cursor, add the following to your `mcp.json` configuration file:

```json
{
  "mcpServers": {
    "mcp-sse": {
      "url": "https://your-project-name.shuttle.app/sse"
    }
  }
}
```

Or if connecting to your local development server:

```json
{
  "mcpServers": {
    "mcp-sse-local": {
      "url": "http://localhost:8000/sse"
    }
  }
}
```

Replace `your-project-name` with your actual deployed project name.

## Tools

- `increment` - Increment counter by 1
- `decrement` - Decrement counter by 1
- `get_value` - Get current counter value
- `reset` - Reset counter to zero
