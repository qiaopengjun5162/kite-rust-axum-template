#!/usr/bin/env bash
# Kite x402 Rust/Axum template — quick start commands.

default:
	cargo build --release

# Run the server (requires .env with PAY_TO and UPSTREAM_URL)
.PHONY: run
run:
	cargo run

# Run in background with example env
.PHONY: run-demo
run-demo:
	PAY_TO=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045 \
	KITE_NETWORK=mainnet \
	UPSTREAM_URL=https://api.open-meteo.com \
	PRICE_USD=0.001 \
	SERVICE_DESCRIPTION="Open-Meteo forecast behind x402 on Kite" \
	PORT=8080 \
	cargo run

# Check compilation
.PHONY: check
check:
	cargo check

# Run all tests
.PHONY: test
test:
	cargo test

# Run linter
.PHONY: lint
lint:
	cargo clippy -- -D warnings

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
