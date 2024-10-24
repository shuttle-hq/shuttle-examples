# Actix Web Cookie authentication with actix-session

## Introduction

This template uses the [actix_identity](https://docs.rs/actix-identity) and [actix_session](https://docs.rs/actix-session) crates to manage user sessions.

## Features

- (basic) User-based session management
- Private/public routing

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visiting <http://localhost:8000> will show the currently logged in user.
Visiting <http://localhost:8000/login> will log you in as `user1` (no authentication logic is in place).
Visiting <http://localhost:8000/logout> will log you out again.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
