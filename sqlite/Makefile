# Makefile for basic Rust project

# Check for syntax errors or issues with dependencies
install:
	cargo build

# Check for compilation without building binary
check:
	cargo check

# Format the code (basic linting for style)
format:
	cargo fmt -- --check

# Run Clippy for more detailed formatting
lint:
	cargo clippy -- -D warnings

# Run tests
test:
	cargo test

# Build the release binary
release:
	cargo build --release

# Run all CI checks, including linting
all: check format lint test release
