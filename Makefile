build:
	cargo build --release --target=wasm32-unknown-unknown
	cargo wasi build --release
	wasm-pack build --release
	sed -i 's#rs-templating#@tonyhb/rs-templating#' pkg/package.json

test:
	cargo test
