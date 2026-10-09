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
	@cargo deny check advisories

## typos: Spell check
typos:
	@typos

## lint: Run all linters (fmt, clippy, deny, typos)
lint: format clippy deny typos

## setup-hooks: Install git hooks (required before first commit)
setup-hooks:
	@git config core.hooksPath .githooks
	@echo "✅ hooksPath set to .githooks (runs fmt+check on every commit)"
	@echo "   Run 'make pre-commit-install' to also install pre-commit.io hooks"

## pre-commit-install: Install pre-commit.io (optional, conflicts with .githooks)
pre-commit-install:
	@echo "⚠️  Unsetting .githooks to install pre-commit..."
	@git config --unset core.hooksPath 2>/dev/null || true
	@pre-commit install
	@pre-commit install --hook-type commit-msg
	@echo "✅ pre-commit hooks installed"

## pre-commit: Run pre-commit hooks
pre-commit:
	@pre-commit run --all-files

## pr: Create a PR (self-PR flow — local validation first)
pr: lint test
	@echo "✅ Local checks passed. Creating PR..."
	@read -p "Enter PR title: " title; \
	 hub pull-request -m "$$title"
	@echo "✅ PR created"

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
