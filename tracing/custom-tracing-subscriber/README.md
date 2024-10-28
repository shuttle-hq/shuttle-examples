# Custom Tracing Subscriber with Shuttle

## Introduction

An example that showcases using a custom `tracing-subscriber` with Shuttle.

Notes:
- `default-features` is disabled on `shuttle-runtime` crate to allow this to be possible

## Features

- A Shuttle serivce with a custom tracing subscriber.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000> to try it out - you should get a message in your logs upon receiving the HTTP request.

You can also additionally adjust the custom tracing subscriber as you want.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
