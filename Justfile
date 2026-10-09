#!/usr/bin/env bash
# Kite x402 Rust/Axum template — quick start commands.

default:
	cargo build --release

# Run the server (requires .env with PAY_TO and UPSTREAM_URL)
run:
	cargo run

# Run in background with example env
run-demo:
	PAY_TO=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045 KITE_NETWORK=mainnet UPSTREAM_URL=https://api.open-meteo.com PRICE_USD=0.001 SERVICE_DESCRIPTION="Open-Meteo forecast behind x402 on Kite" PORT=8080 cargo run

# Check compilation
check:
	cargo check

# Run all tests
test:
	cargo test

# Run linter
lint: fmt clippy deny typos

# Format code
fmt:
	cargo fmt

# Clippy
clippy:
	cargo clippy --all-targets --all-features --tests --benches -- -D warnings

# Format TOML
taplo-fmt:
	taplo fmt --option reorder_keys=true --check

# Dependency check
deny:
cargo deny check advisories

# Spell check
typos:
	typos

# Run pre-commit hooks
pre-commit:
	pre-commit run --all-files

# Clean build artifacts
clean:
	cargo clean
