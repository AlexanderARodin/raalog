
help:
	@cat Makefile

test:
	@cargo test -- --show-output

run: release
	@cargo run --release

release:
	@cargo rustc --release -- -C prefer-dynamic
release.wasm:
	@cargo build --release --target wasm32-unknown-unknown


git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

clean: clean.dist
	@cargo clean
