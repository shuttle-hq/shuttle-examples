# Actix Web Actorless Websockets

## Introduction

This template shows how to use actorless websockets to build an API monitoring service and a simple chat app.

## Features

- A plain HTML-based frontend
- Websocket connection

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Then visit <http://localhost:8000/index.html> to try it out!

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
