# Axum with Static Assets

## Introduction

This example shows how to serve static assets using [axum](https://github.com/tokio-rs/axum) and Shuttle.

This example is inspired by the static-file-server example from the axum repo, to see more ways to do this check out the [original](https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs).

## Features

- Plain HTML-based frontend

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000/index.html> to view the homepage.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
