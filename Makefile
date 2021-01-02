build:
	wasm-pack build --release
	sed -i 's#rs-templating#@tonyhb/rs-templating#' pkg/package.json

test:
	cargo test
