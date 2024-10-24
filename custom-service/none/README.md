# Shuttle Custom Service

## Introduction

This template shows how to create your own custom Shuttle service that can run anything HTTP-based.

## Features

- A struct that implements `shuttle_runtime::Service`.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Set up whatever you want to do in the `bind()` function, then run the project using `shuttle run`.

Visit <http://localhost:8000> (or your relevant routes) to try it out.

Variables from resource annotations can be added to `MyService` struct to use them in the `bind()` function.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- Note that by default the service doesn't do anything. You need to run a server or do something else that runs in an infinite loop otherwise the program will exit.
