# Rust, Qdrant, Shuttle: A Rust-based semantic document search

This example shows how to create a semantic document search with chat capability in Rust, using Shuttle and Qdrant - a vector search engine.

## Prerequisites

1. Install Shuttle CLI tools:

```bash
cargo install cargo-shuttle
```

2. Get a Qdrant Account at [qdrant.tech](https://qdrant.tech)

3. Get an OpenAI API key at [openai.com](https://platform.openai.com)

4. Create a `Secrets.toml` file (or `Secrets.dev.toml` for local development) with the following contents:

```toml
OPENAI_API_KEY = "<ENTER YOUR KEY HERE>"
QDRANT_TOKEN = "<ENTER YOUR KEY HERE>"
QDRANT_URL = "<ENTER YOUR KEY HERE>"
```

## Running the example

```bash
cargo shuttle run
```

## Troubleshooting
- If you run the service but it only lists the files loaded and doesn't try to embed them, check you have credits/funding in your OpenAI account.
- Make sure your Qdrant URL ends with port `:6334`.
