# Salvo image rescaler

## Introduction

This is a simple example showing how to use path parameters in Salvo and the `image` crate to load and manipulate an image.

## Features

- A route that returns a resized version of the Shuttle logo.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visiting <http://localhost:8000/400/400> returns the Shuttle logo image but rescaled to 400 x 400 pixels.

When you're ready to deploy, use `shuttle deploy`.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
