
help:
	@cat Makefile

all: test release.native release.wasm

test:
	@cargo test

release.native:
	@cargo build --release
release.wasm:
	@cargo build --release --target wasm32-unknown-unknown


dev.test:
	@cargo test -- --show-output

clean:
	@cargo clean
