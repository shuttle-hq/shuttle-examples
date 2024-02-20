# Shuttle-Bevy Example
This repository serves as an example of how you can compile Bevy to WASM and run it on Shuttle.

## Instructions
Make sure you have `wasm-bindgen-cli` and the rustup `wasm32-unknown-unknown` compilation target added. If you don't, you can get them by running the following commands below:
```bash
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

Then clone this repository and go into the `game` folder.

You can try the example out by simply running `cargo run`, or go straight to compilation if you want to get your game straight to the browser.

In the Makefile, there will be an already setup command for compiling to WASM. If you don't have `make` installed or are running a build system that  cannot use `make`, you can run the command manually.
```bash
make build
```
Then if you go to the project workspace root or the `server` folder and run `cargo shuttle run`, then visit `http://localhost:8000` you should see the following message:
```
Hello world! Visit /game to see your Bevy build.
```
If you then go to `http://localhost:8000/game`, you should see your Bevy program in action!
