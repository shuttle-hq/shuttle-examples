# Actix Web Hello World

## Introduction

A `Hello world!` example for using Actix Web with Shuttle.

## Features

- A route that returns `Hello, world!`.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visiting <http://localhost:8000> should output `Hello, world!`.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
