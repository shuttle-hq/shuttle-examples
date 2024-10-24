# OpenDAL Memory Storage

## Introduction

This example shows that how to connect to an in-memory storage using OpenDAL.

The project consists of the following files

- `Shuttle.toml` contains the name of the app
- `src/main.rs` is where all the magic happens - it creates a Shuttle service with two endpoints: one for adding new data and one for retrieving it back.

## Features

- Public routes for adding/retrieving data

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Then try the following `curl` command:
T
```bash
curl -X POST -H 'content-type: application/json' localhost:8000/todo --data '{"note":"My todo"}'
# {"id":1,"note":"My todo"}
```

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
