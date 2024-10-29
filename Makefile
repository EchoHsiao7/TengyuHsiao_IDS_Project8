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

p_setup:
	pip install -r requirements.txt


p_lint:
	pylint --disable=R,C,locally-disabled --ignore-patterns=test_.*?py *.py

p_format:	
	black *.py 


p_test:
	pytest 


p_dev:
	docker-compose up --build


p_clean:
	find . -type f -name "*.pyc" -delete