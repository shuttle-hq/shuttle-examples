# Standalone binary - run an app with Shuttle or standalone

## Introduction

This example shows how to separate a project's Shuttle logic from its core functionality so that two binaries can be made: one for running with `shuttle run` and deploying to Shuttle, and one that can be run with `cargo run --bin ...`.

All startup logic is placed in the binary source files, while the implementation (endpoints etc) is moved to the library of the crate.

- `src/bin/shuttle.dev` is the main binary with Shuttle, run with `shuttle run`. Note that the `[[bin]]` entry in `Cargo.toml` needs to have the same name as the crate. The file can have any name you want.
- `src/bin/standalone.rs` is without Shuttle, run with `cargo run --bin standalone` (you can change the name)

This example shows how to use separate logic for getting secrets (Shuttle secrets vs homemade solution), but the same approach can be applied to other resources that are initiated by Shuttle's main function.

## Features

- Separate binaries to allow non-Shuttle deployments.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

Visit <http://localhost:8000> to try it out.

You can use `cargo run --bin standalone` to run the non-Shuttle binary.

When you're ready to deploy, use `npm run deploy` which will build the static assets then deploy the service.

## Troubleshooting

- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- If you're getting errors about not being able to find the right binary, make sure the binary name is the same as your project.
