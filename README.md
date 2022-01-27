# rs-templating

A kind templating library built using [tera](http://github.com/keats/tera), allowing you to:

- Inspect templates to find all variable names
- Execute templates with missing variables, without throwing exceptions

It also contains a webassembly build via wasm, a shared library, and a static library for use
in other programming languages.

### Building

**Pre-requisites**

Install cargo-wasi && wasm pack:

1. `cargo install cargo-wasi`
2. `cargo install wasm-pack`

**Building**

Running `make build` will build the dyanmic library, static library x86 binaries, and webassembly.
