all: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: style-fix
style-fix:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy
