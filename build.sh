#! /bin/bash
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
	rustup run nightly \
	wasm-pack build --target web --scope wasml \
	-- -Z build-std=panic_abort,std
