# Rocket with Cargo Workspaces

## Introduction

This example shows how to use a Cargo workspace with Shuttle using the `rocket` web service frameowrk.

## Features

- A route that returns `Hello, world!`.
- Cargo workspace

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
- The Shuttle package needs to be the default
