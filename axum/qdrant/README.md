# Axum with Qdrant on Shuttle

## Introduction

This example shows how to use `shuttle-qdrant` to connect to a Qdrant vector database from Shuttle.

## Features

- View your Qdrant collections at the base route

## Pre-requisites

- Rust
- Qdrant user account
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Go to [Qdrant](https://qdrant.tech/) and start a database and collection.
Paste the URL and API key in the Secrets.toml file.

Run the project with `shuttle run`.

Visit <http://localhost:8000> to view your collections.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If connecting to Qdrant doesn't work, try checking your Secrets.toml file.
