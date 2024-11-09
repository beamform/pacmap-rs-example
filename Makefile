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
	@echo "  make docker-build  - Build Docker image"
	@echo "  make docker-run    - Run Docker container"
	@echo "  make docker-clean  - Remove Docker image"

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

# Docker image name
DOCKER_IMAGE := pacmap-rs-example
DOCKER_TAG := latest

# Build Docker image
.PHONY: docker-build
docker-build:
	@echo "Building Docker image..."
	docker build -t $(DOCKER_IMAGE):$(DOCKER_TAG) .

# Run Docker container
.PHONY: docker-run
docker-run:
	@echo "Running Docker container..."
	docker run --rm -v "$(PWD):/home/pacmap" $(DOCKER_IMAGE):$(DOCKER_TAG)

# Clean Docker image
.PHONY: docker-clean
docker-clean:
	@echo "Removing Docker image..."
	docker rmi $(DOCKER_IMAGE):$(DOCKER_TAG)

# All Docker operations
.PHONY: docker-all
docker-all: docker-build docker-run
