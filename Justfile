alias b := build
build:
  cargo build --release --target wasm32-unknown-unknown --target-dir target/
