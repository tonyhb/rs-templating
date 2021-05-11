build:
	cargo build --bin rs-templating --release
	cargo build --lib --release --target=wasm32-unknown-unknown
	cargo wasi build --lib --release
	wasm-pack build --release
	sed -i 's#rs-templating#@tonyhb/rs-templating#' pkg/package.json

test:
	cargo test
