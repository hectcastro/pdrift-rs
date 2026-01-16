.PHONY: build test lint format ci

build:
	mise exec -- cargo build --release

test:
	mise exec -- cargo test

lint:
	mise exec -- cargo clippy -- -D warnings

format:
	mise exec -- cargo fmt

ci: lint test
