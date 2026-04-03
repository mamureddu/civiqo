# Contributing

Civiqo is a solo project but I'm happy to accept contributions. Whether it's a bug fix, a feature idea, or just a typo — feel free to open a PR or an issue.

## Getting started

```bash
git clone https://github.com/mamureddu/civiqo.git
cd civiqo
./setup.sh
```

Or manually:

```bash
# You need: Rust (>= 1.75), PostgreSQL (>= 15), Node.js (>= 18)
cp src/.env.example src/.env   # edit with your DB credentials
cd src && cargo run -p server  # http://localhost:9001
```

## Before submitting a PR

```bash
cd src
cargo fmt --all                    # format
cargo clippy -- -D warnings        # lint
cargo test --workspace             # test
```

CI runs all three — if they pass locally, they'll pass on GitHub.

## Commit style

I use [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `refactor:`, `docs:`, `test:`.

## Reporting bugs

Open a [GitHub Issue](https://github.com/mamureddu/civiqo/issues). Include steps to reproduce if you can.
