up:
	cargo run
build: 
	cargo build --release --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ../server/game \
  --target web target/wasm32-unknown-unknown/release/game.wasm	
