build-all: build go

build: build-wasm build-x86 build-aarch64

build-wasm:
	cargo wasi build --lib --release
	wasm-pack build --release
	sed -i 's#rs-templating#@tonyhb/rs-templating#' pkg/package.json

build-x86:
	# ensure target exists
	rustup target add x86_64-unknown-linux-musl
	rustup target add x86_64-unknown-linux-gnu
	# build lib for gnu
	cargo build --lib --release --target x86_64-unknown-linux-gnu
	cargo build --bin rs-templating --release --target x86_64-unknown-linux-gnu
	# build lib for musl
	cargo build --lib --release --target x86_64-unknown-linux-musl
	cargo build --bin rs-templating --release --target x86_64-unknown-linux-musl
	# copy
	cp ./target/x86_64-unknown-linux-musl/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_x86_linux_musl.a
	cp ./target/x86_64-unknown-linux-gnu/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_x86_linux_gnu.a
	strip -g ./bindings/golang/ffitemplating/*x86_linux*

build-aarch64:
	rustup target add aarch64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-gnu
	cargo build --lib --release --target aarch64-unknown-linux-musl
	cargo build --lib --release --target aarch64-unknown-linux-gnu
	cp ./target/aarch64-unknown-linux-musl/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_aarch64_linux_musl.a
	cp ./target/aarch64-unknown-linux-gnu/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_aarch64_linux_gnu.a
	strip -g ./bindings/golang/ffitemplating/*aarch64_linux*

build-darwin:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	RUSTFLAGS="-C link-arg=-s"  cargo build --lib --release --target aarch64-apple-darwin
	RUSTFLAGS="-C link-arg=-s"  cargo build --lib --release --target x86_64-apple-darwin
	cp ./target/aarch64-apple-darwin/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_aarch64_darwin.a
	cp ./target/x86_64-apple-darwin/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_x86_darwin.a
	strip ./bindings/golang/ffitemplating/*darwin*



go:
	cp ./src/lib_ffi.h ./bindings/golang/

test:
	cargo test
