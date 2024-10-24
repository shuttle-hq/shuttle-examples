# Rocket with dynamic templates using Handlebars

## Introduction

This example shows how to render templated content using Handlebars templates from [rocket](https://github.com/SergioBenitez/Rocket/) and shuttle.

This example is inspired by templating example from the rocket repo, to see more ways to do this checkout the [original](https://github.com/SergioBenitez/Rocket/tree/master/examples/templating).

## Features

- Plain HTML-based frontend with HTML templating

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000/hello/shuttle> to try it out.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
