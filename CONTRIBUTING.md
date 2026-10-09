# Contributing to kite-rust-axum-template

Thank you for your interest in contributing!

## Development Setup

```bash
# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Install required tools
cargo install taplo-cli typos-cli cargo-deny --locked
```

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — New feature
- `fix:` — Bug fix
- `refactor:` — Code change that neither fixes a bug nor adds a feature
- `docs:` — Documentation only
- `ci:` — CI/CD changes
- `test:` — Adding or fixing tests
- `chore:` — Maintenance tasks

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with conventional commit messages
3. Run `make lint` to check code quality
4. Run `make test` to ensure all tests pass
5. Open a PR against `main`
6. PR Agent will automatically review the changes
7. CI must pass before merging

## Release Process

Releases are automated via Release Please. When a PR with changes is merged
to `main`, Release Please creates a release PR that bumps the version and
updates `CHANGELOG.md` automatically.
