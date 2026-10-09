# ==============================================================================
# kite-rust-axum-template
# Description: Kite x402 service template (Rust + Axum)
# ==============================================================================

CARGO := cargo
SHELL := /bin/bash
MAIN_BRANCH := main

.PHONY: all build clean test bench check clippy format release update help
all: build

## build: Build the project
build:
	@$(CARGO) build --all-features

## build-release: Build in release mode
build-release:
	@$(CARGO) build --release --all-features

## clean: Clean build artifacts
clean:
	@$(CARGO) clean

## test: Run all tests
test:
	@$(CARGO) test --all-features

## bench: Run benchmarks
bench:
	@$(CARGO) bench --all-features

## check: Quick code check
check:
	@$(CARGO) check --all-features

## clippy: Lint code with Clippy
clippy:
	@$(CARGO) clippy --all-features -- -D warnings

## format: Check code formatting
format:
	@$(CARGO) fmt --all -- --check

## deny: Check dependency security/licenses
deny:
	@cargo deny check -d

## typos: Spell check
typos:
	@typos

## lint: Run all linters (fmt, clippy, deny, typos)
lint: format clippy deny typos

## pre-commit: Run pre-commit hooks
pre-commit:
	@pre-commit run --all-files

## run: Run the server
run:
	@$(CARGO) run

## release: Create a new release
release: test
	@echo "🧪 Tests passed. Proceeding with release..."
	@git cliff -o CHANGELOG.md
	@git add CHANGELOG.md
	@git commit -m "chore: update changelog" || echo "No changes"
	@git push --follow-tags origin $(MAIN_BRANCH)
	@echo "✅ Release complete."

## help: Show this help message
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
