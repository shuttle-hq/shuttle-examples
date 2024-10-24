# Axum JWT authentication

## Introduction

This template uses the `jsonwebtoken` crate to encode and decode claims in a JWT, which clients use in the Authorization HTTP header to access protected resources (endpoints).

## Features

- JWT based authentication

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000> to try it out.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
