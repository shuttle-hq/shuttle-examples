# Cargo Feature Flags with Shuttle

## Introduction

An example that showcases how to use feature flags with Shuttle (see `Cargo.toml`).

## Features

- Feature flags for conditionally enabling or disabling Shuttle (see `Cargo.toml`).

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000> to try it out.

When you're ready to deploy, use `npm run deploy` which will build the static assets then deploy the service.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
