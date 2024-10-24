# Axum with Turso

## Introduction

An example that showcases using Turso with Axum and Shuttle.

## Features

- Use Turso to store and retrieve database records.

## Pre-requisites

- Rust
- Turso user account
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Set your `TURSO_DB_TOKEN` in Secrets.toml and Turso database in `src/main.rs` in the annotation.

Run the project with `shuttle run`.

Then try it out with the following `curl` command:

```sh
curl http://localhost:8000 -H 'content-type: application/json' --data '{"uid":"1","email":"foo@bar.xyz"}'
```

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
