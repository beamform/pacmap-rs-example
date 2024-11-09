# Default target
.PHONY: all
all: help

# Help message
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make install       - Install Rust toolchain using rustup"
	@echo "  make build         - Build the project in development mode"
	@echo "  make build-release - Build the project in release mode"
	@echo "  make run           - Run the project in release mode"
	@echo "  make clean         - Remove build artifacts"

# Install Rust toolchain
.PHONY: install
install:
	@echo "Installing Rust toolchain..."
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@echo "Please restart your terminal or run: source $$HOME/.cargo/env"

# Build in development mode
.PHONY: build
build:
	@echo "Building in development mode..."
	cargo build

# Build in release mode
.PHONY: build-release
build-release:
	@echo "Building in release mode..."
	cargo build --release

# Run in release mode
.PHONY: run
run:
	@echo "Running in release mode..."
	cargo run --release

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
