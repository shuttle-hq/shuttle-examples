## Using Shuttle Secrets with Rocket

## Introduction

This example shows how to use secrets using [rocket](https://github.com/rwf2/rocket) and Shuttle.

The secrets resource requires a `Secrets.toml` file to be present in your crate. Each like in this file
should be a key-value pair that you can access using `SecretStore::get(&self, key)`.

## Features

- A route that displays the secret from `Secrets.toml`

## Pre-requisites

- Rust
- Docker if running locally
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Rename `Secrets.toml.example` to `Secrets.toml`, then run this project with `shuttle run`.

Visit <http://localhost:8000> to try it out.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're getting errors, check your `Secrets.toml` file.
