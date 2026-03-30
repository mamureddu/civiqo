# Civiqo Community Manager

## Stack
- Rust/Axum backend, Tera templates, HTMX + Alpine.js frontend
- PostgreSQL 18 via SQLx
- Custom JWT auth (argon2 + HS256) — NO Auth0, NO AWS Lambda
- Workspace: `src/server`, `src/shared`, `src/services/chat-service`

## Development
- Server runs from `src/` directory: `cd src && cargo run -p server`
- Server binds to http://localhost:9001
- Env file at `src/.env`
- Migrations auto-run on startup

## Auth system
- Email/password with argon2 hashing
- Session-based for HTMX pages (tower-sessions MemoryStore)
- JWT (HS256) for API endpoints
- SSO-ready: users table has `provider` + `provider_id` columns
- Key file: `src/server/src/auth.rs` — login/register handlers + AuthUser/OptionalAuthUser extractors
- Key file: `src/shared/src/auth/mod.rs` — JwtService (issue/validate/refresh)

## Deployment
- Target: VPS (Hetzner/Contabo) with Caddy + systemd
- Scripts in `deploy/` directory
- Cross-compile with cargo-zigbuild for x86_64-unknown-linux-gnu
