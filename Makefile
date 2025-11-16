# Eidos Makefile
# Common build and release tasks

.PHONY: help build build-release test clean install docker docker-run format lint check-all

# Default target
help:
	@echo "Eidos Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build          - Build debug binary"
	@echo "  build-release  - Build optimized release binary"
	@echo "  test           - Run all tests"
	@echo "  bench          - Run benchmarks"
	@echo "  clean          - Clean build artifacts"
	@echo "  install        - Install to ~/.local/bin"
	@echo "  docker         - Build Docker image"
	@echo "  docker-run     - Run in Docker container"
	@echo "  format         - Format code with rustfmt"
	@echo "  lint           - Run clippy linter"
	@echo "  check-all      - Run all checks (format, lint, test)"
	@echo "  release        - Build release artifacts for distribution"

# Build targets
build:
	@echo "Building debug binary..."
	cargo build

build-release:
	@echo "Building release binary..."
	cargo build --release --locked
	@echo "Binary: target/release/eidos"

# Testing
test:
	@echo "Running tests..."
	cargo test --all

bench:
	@echo "Running benchmarks..."
	cargo bench

# Cleaning
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf dist/

# Installation
install: build-release
	@echo "Installing to ~/.local/bin..."
	mkdir -p ~/.local/bin
	cp target/release/eidos ~/.local/bin/
	chmod +x ~/.local/bin/eidos
	@echo "Installed to ~/.local/bin/eidos"
	@echo "Make sure ~/.local/bin is in your PATH"

# Docker
docker:
	@echo "Building Docker image..."
	docker build -t eidos:latest .

docker-run:
	@echo "Running Eidos in Docker..."
	docker-compose run --rm eidos

# Code quality
format:
	@echo "Formatting code..."
	cargo fmt --all

lint:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

check-all: format lint test
	@echo "All checks passed!"

# Release builds
release: clean
	@echo "Building release artifacts..."
	mkdir -p dist

	# Linux x86_64
	@echo "Building for Linux x86_64..."
	cargo build --release --locked
	cp target/release/eidos dist/eidos-linux-x86_64
	strip dist/eidos-linux-x86_64

	# Create tarball
	@echo "Creating tarball..."
	tar -czf dist/eidos-linux-x86_64.tar.gz \
		-C dist eidos-linux-x86_64 \
		-C .. README.md LICENSE eidos.toml.example

	@echo "Release artifacts created in dist/"
	@ls -lh dist/

# Package installation files
package:
	@echo "Creating installation package..."
	mkdir -p dist/eidos-$(VERSION)
	cp target/release/eidos dist/eidos-$(VERSION)/
	cp README.md LICENSE eidos.toml.example dist/eidos-$(VERSION)/
	cp install.sh dist/eidos-$(VERSION)/
	tar -czf dist/eidos-$(VERSION)-linux-x86_64.tar.gz -C dist eidos-$(VERSION)
	rm -rf dist/eidos-$(VERSION)
	@echo "Package created: dist/eidos-$(VERSION)-linux-x86_64.tar.gz"

# Development helpers
dev-setup:
	@echo "Setting up development environment..."
	rustup component add rustfmt clippy
	@echo "Installing cargo-watch for auto-rebuild..."
	cargo install cargo-watch || true
	@echo "Development environment ready!"

watch:
	@echo "Watching for changes..."
	cargo watch -x check -x test

# Quick run targets
run-chat:
	cargo run -- chat "Hello, world!"

run-translate:
	cargo run -- translate "Bonjour le monde"

run-core:
	cargo run -- core "list files"

# CI targets
ci-test:
	cargo test --all --locked

ci-lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

ci-build:
	cargo build --release --locked

.DEFAULT_GOAL := help
