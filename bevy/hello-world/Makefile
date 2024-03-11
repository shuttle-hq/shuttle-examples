build:
	cargo build --package game --release --target wasm32-unknown-unknown && \
		wasm-bindgen --out-dir dist --target web target/wasm32-unknown-unknown/release/game.wasm	
up: build
	cargo shuttle run
