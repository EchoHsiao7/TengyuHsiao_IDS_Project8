all: check build format lint test

check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
	
test_main:
	cargo test --test test_main

release:
	cargo build --release