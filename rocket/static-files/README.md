## Serving Static Assets with Rocket

## Introduction

This example shows how to serve static assets using [rocket](https://github.com/rwf2/rocket) and Shuttle.

## Features

- Plain HTML-based frontend

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run this project with `shuttle run`.

Visit <http://localhost:8000/index.html> to try it out.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
