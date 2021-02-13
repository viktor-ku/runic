all: test-all

test:
	cargo t -- --nocapture
	wasm-pack test --firefox --headless

test/web:
	wasm-pack test --firefox --headless

build:
	cargo build
	wasm-pack build

test-all: build test
