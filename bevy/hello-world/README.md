# Bevy on Shuttle (with Axum)

## Introduction

Example of how you can compile Bevy to WASM and run it on Shuttle.

## Features

- A Bevy program that prints `Hello, world!`.
- A server that runs the WASM-compiled Bevy program.

## Pre-requisites

- Rust
- [Bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Make sure you have `wasm-bindgen-cli` and the rustup `wasm32-unknown-unknown` compilation target added. If you don't, you can get them by running the following:
```bash
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

You can try the example out by simply running `cargo run`, or go straight to compilation if you want to get your game straight to the browser.

In the Makefile, there is a command for compiling the game to WASM.
If you don't have `make` installed, you can run the command manually.

```bash
make build
```

Go to the project workspace root or the `server` folder, run `shuttle run`, then visit `http://localhost:8000/game`, you should see your Bevy program in action!

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
- Don't forget to install Bevy dependencies (as above) if you get compilation errors.
