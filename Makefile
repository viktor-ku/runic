all: build

test: pc/test wasm/test
t: test

build: pc/build wasm/build

pc/build:
	cargo build --release

pc/test:
	cargo t -- --nocapture

wasm/build:
	wasm-pack build
	mkdir -p js/wasm
	mv pkg/runic* js/wasm
	rm -rf pkg
	npm run build

wasm/test:
	wasm-pack test --firefox --headless
