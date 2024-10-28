# Axum with HTMx

## Introduction

This is an example of how you can use Shuttle with Axum, Askama and htmx to create a frontend that's easily extendable and requires zero framework knowledge, while being able to easily inject variables from the backend into the frontend.

The app is a TODO list with a main page and an event stream page.

## Features

- A templating-based HTML frontend
- Routes for creating/viewing todos
- A route for viewing a real-time stream of todo list changes

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Run the app and go to <http://localhost:8000> to add and remove TODOs from the list.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
