Example of how you can compile Bevy to WASM and run it on Shuttle.

## Instructions

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

Go to the project workspace root or the `server` folder, run `cargo shuttle run`, then visit `http://localhost:8000/game`, you should see your Bevy program in action!
