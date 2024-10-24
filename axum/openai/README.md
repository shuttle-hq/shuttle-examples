# Axum with OpenAI

## Introduction

A simple endpoint that sends a chat message to ChatGPT and returns the response.

## Features

- Query an OpenAI model at the base route

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Set your OpenAI API key in `Secrets.toml`.

Run the project with `shuttle run`.

Try the following `curl` command to query OpenAI:

```sh
curl http://localhost:8000 -H 'content-type: application/json' --data '{"message":"What is shuttle.dev?"}'
```

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If connecting to OpenAI doesn't work, try checking your Secrets.toml file.
