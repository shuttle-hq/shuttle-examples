# Custom Shuttle Resource

## Introduction

This template shows how to create your own custom Shuttle resource in a Shuttle project, using Axum for the `main.rs` file to set up the route.

Note that PDO is short for Plain Data Object.

## Features

- A `lib.rs` file that contains code for a custom Shuttle resource
- A `main.rs` file that uses the

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project using `shuttle run`.

Visit <http://localhost:8000> to try it out.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
