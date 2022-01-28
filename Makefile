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
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --lib --release --target x86_64-unknown-linux-gnu
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --bin rs-templating --release --target x86_64-unknown-linux-gnu
	# build lib for musl
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --lib --release --target x86_64-unknown-linux-musl
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --bin rs-templating --release --target x86_64-unknown-linux-musl
	# copy
	cp ./target/x86_64-unknown-linux-musl/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_x86_linux_musl.a
	cp ./target/x86_64-unknown-linux-gnu/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_x86_linux_gcc.a

build-aarch64:
	rustup target add aarch64-unknown-linux-musl
	cargo build --lib --release --target aarch64-unknown-linux-musl
	cp ./target/aarch64-unknown-linux-musl/release/librs_templating.a ./bindings/golang/ffitemplating/librs_templating_aarch64_linux_musl.a


go:
	cp ./src/lib_ffi.h ./bindings/golang/

test:
	cargo test
