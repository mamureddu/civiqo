# Contributing to Civiqo

Thank you for your interest in contributing to Civiqo!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/civiqo.git`
3. Run `./setup.sh` or follow the manual setup in the README
4. Create a feature branch: `git checkout -b feature/your-feature`

## Development Setup

**Prerequisites:**
- Rust (stable, >= 1.75)
- PostgreSQL 18
- Node.js >= 18 (for Tailwind CSS)

**Run the server:**
```bash
cd src && cargo run -p server
# Server at http://localhost:9001
```

**Run tests:**
```bash
cd src && cargo test --workspace
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Follow existing code patterns and naming conventions
- Write tests for new functionality

## Pull Request Process

1. Ensure all tests pass: `cd src && cargo test --workspace`
2. Ensure no clippy warnings: `cd src && cargo clippy -- -D warnings`
3. Ensure formatting: `cd src && cargo fmt --check`
4. Update documentation if needed
5. Use conventional commit messages (e.g., `feat:`, `fix:`, `refactor:`, `docs:`)
6. Submit your PR against the `main` branch

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add poll voting endpoint`
- `fix: correct member role validation`
- `refactor: extract auth middleware`
- `docs: update API reference`
- `test: add community creation tests`

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Include steps to reproduce for bugs
- Check existing issues before opening a new one

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
