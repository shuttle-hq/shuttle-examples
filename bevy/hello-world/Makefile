build:
	cargo build --release --target wasm32-unknown-unknown --manifest-path ./game/Cargo.toml && \
		wasm-bindgen --out-dir server/dist --target web game/target/wasm32-unknown-unknown/release/game.wasm	
up: 
	make build && cargo shuttle run
