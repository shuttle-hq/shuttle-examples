# Actix Web with Shuttle Shared DB (Postgres)

## Introduction

This template shows how to connect a Postgres database and use it for a simple TODO list app.

## Features
- Public routes for creating/fetching notes using a database.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Then run the following `curl` commands below:

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
- If you're running locally, don't forget to have Docker installed and running!
