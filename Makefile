.PHONY: help build test clean docker-build docker-run

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in debug mode
	cargo build

build-dev: ## Build the project with dynamic linking (faster)
	cargo build --features dev

build-release: ## Build the project in release mode
	cargo build --release

run: ## Run the game in development mode
	cargo run --features dev

run-release: ## Run the game in release mode
	cargo run --release

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

clean: ## Clean build artifacts
	cargo clean

docker-build: ## Build Docker image
	docker build -t template-bevy:latest .

docker-run: ## Run Docker container
	docker run --rm template-bevy:latest

all: fmt clippy test build ## Run format, clippy, test, and build
