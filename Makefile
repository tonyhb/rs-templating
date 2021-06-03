build:
	cargo build --bin rs-templating --release
	cargo build --lib --release --target=wasm32-unknown-unknown
	cargo build --bin rs-templating --release --target x86_64-unknown-linux-musl
	cargo wasi build --lib --release
	wasm-pack build --release
	sed -i 's#rs-templating#@tonyhb/rs-templating#' pkg/package.json

test:
	cargo test
