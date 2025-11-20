# Makefile for affiliate-config-parser

# Default run target (example usage: make run FILE=example.offer)
run:
	cargo run -- parse $(FILE)

# Format code using cargo fmt
fmt:
	cargo fmt

# Lint using Clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run unit tests
test:
	cargo test

# Build project
build:
	cargo build --release

# Full validation before commit: format, clippy, tests
check-all: fmt clippy test

# Show help
help:
	@echo "Available commands:"
	@echo "  make run FILE=example.offer   - Run parser on a file"
	@echo "  make fmt                      - Format code (cargo fmt)"
	@echo "  make clippy                   - Run lints"
	@echo "  make test                     - Run unit tests"
	@echo "  make build                    - Build release binary"
	@echo "  make check-all                - Run fmt + clippy + tests"
