# Axum with Shuttle Shared DB (Postgres)

## Introduction

This template shows how to connect a Postgres database and use it for a simple TODO list app.

## Features

- Public routes for fetching/creating notes using a database.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Set your OpenAI API key in `Secrets.toml`.

Run the project with `shuttle run`.

Try the following `curl` commands to interact with your database through the API:

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/todos --data '{"note":"My todo"}'
# {"id":1,"note":"My todo"}

curl localhost:8000/todos/1
# {"id":1,"note":"My todo"}
```

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If connecting to OpenAI doesn't work, try checking your Secrets.toml file.
- Make sure you have Docker installed and running.
