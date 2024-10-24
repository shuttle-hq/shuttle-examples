# Actix Web Static Files

## Introduction

This template uses `actix_files` to serve the files in the `assets` directory.

## Features

- Plain HTML-based frontend

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Then run the following `curl` command below (or visit <http://localhost:8000/index.html>):

```bash
curl localhost:8000/index.html
# <h1>Hello world!</h1>
```

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
