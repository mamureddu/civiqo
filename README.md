# Civiqo

A decentralized community management platform enabling local communities to organize, govern, and collaborate.

## Features

- Multi-role community management (owner, admin, moderator, member)
- Geographic-based community discovery
- End-to-end encrypted real-time chat
- Local business integration
- Governance tools (polls, voting, proposals)
- Internationalization (Italian, English)

## Tech Stack

- **Backend**: Rust / Axum
- **Database**: PostgreSQL 18 (via SQLx, migrations auto-run)
- **Auth**: Custom JWT (argon2 + HS256) with session-based HTMX pages
- **Frontend**: Server-rendered Tera templates + HTMX + Alpine.js
- **Styling**: Tailwind CSS
- **Deployment**: VPS with Caddy reverse proxy + systemd

## Quick Start

```bash
git clone https://github.com/mamureddu/civiqo.git
cd civiqo
./setup.sh
```

The setup script will:
1. Check and optionally install prerequisites (Rust, PostgreSQL, Node.js)
2. Create and configure the database
3. Generate a secure JWT secret
4. Build the project
5. Optionally configure Civiqo as a system service

### Manual Setup

```bash
# Prerequisites: Rust, PostgreSQL 18, Node.js (for Tailwind)

# 1. Configure environment
cp src/.env.example src/.env
# Edit src/.env with your database credentials

# 2. Build and run
cd src && cargo run -p server
# Open http://localhost:9001
```

### Development Commands

```bash
cd src && cargo run -p server          # Start web server
cd src && cargo run -p chat-service    # Start chat service
cd src && cargo test --workspace       # Run all tests
cd src && cargo clippy                 # Lint
cd src && cargo fmt --check            # Check formatting
```

## Project Structure

```
civiqo/
├── src/
│   ├── server/              # Axum web server (HTMX + API)
│   │   ├── src/             # Handlers, auth, middleware
│   │   ├── templates/       # Tera HTML templates
│   │   └── static/          # CSS, JS, images
│   ├── services/
│   │   └── chat-service/    # WebSocket chat service
│   ├── shared/              # Shared library (DB, models, JWT)
│   └── migrations/          # SQLx database migrations
├── deploy/                  # VPS deployment (Caddy, systemd)
├── scripts/                 # Development scripts
├── docs/                    # Documentation
└── setup.sh                 # Interactive setup script
```

## Documentation

- **[Development Guide](docs/DEVELOPMENT.md)** - Setup and workflow
- **[Architecture](docs/ARCHITECTURE.md)** - System design
- **[Authentication](docs/AUTHENTICATION.md)** - Auth flow details
- **[API Reference](docs/API.md)** - API endpoints
- **[Database Schema](docs/SCHEMA.md)** - Schema documentation
- **[Deployment](docs/DEPLOYMENT.md)** - VPS deployment guide
- **[i18n Guide](docs/I18N_GUIDE.md)** - Internationalization

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

To report vulnerabilities, see [SECURITY.md](SECURITY.md).

## License

MIT License - see [LICENSE](LICENSE) for details.
