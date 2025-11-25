# Community Manager

A decentralized community management platform enabling local communities to organize, govern, and collaborate with end-to-end encrypted communication.

## Features
- Multi-role community management (owner, socio, investor, affiliate, supporter)
- Geographic-based community discovery and creation
- End-to-end encrypted real-time chat
- Local business integration with geographic mapping
- Decentralized governance tools (polls, voting, decision-making)
- Mobile-first design (web + future React Native app)

## 🎨 Design & Brand
> **Strict Adherence Required**: This project follows the **Civiqo Brand Book v1.1**.
> 
> All UI/UX decisions must align with the [Brand Guidelines](docs/BRAND_GUIDELINES.md).
> - **Primary Color**: Civiqo Blue (`#3B7FBA`)
> - **Fonts**: Nunito (Headings), Inter (Body)
> - **Tone**: Human, Clear, Reassuring

## Architecture
- **Frontend**: HTMX + Leptos WASM (100% Rust stack)
- **Backend**: Rust microservices with cargo-lambda (Lambda → EC2 progression)
- **Database**: CockroachDB Cloud (PostgreSQL-compatible)
- **Authentication**: Auth0 with custom role management
- **Chat**: Stateless WebSocket service with WASM client
- **Infrastructure**: AWS with progressive scaling (Lambda → EC2 Spot)
- **Mobile**: Native Android (Kotlin) + iOS (Swift)

## Development Phases
1. **Foundation** (4-6 weeks): Auth, communities, basic roles
2. **Core Features** (3-4 weeks): Chat, business profiles, maps
3. **Advanced** (3-4 weeks): E2EE, governance, advanced roles
4. **Mobile** (4-5 weeks): Native Android + iOS app development

## Quick Start
```bash
# Prerequisites: Rust, cargo-lambda, CockroachDB Cloud account

# 1. Configure environment
cp docs/ENVIRONMENT.md src/.env      # Copy and configure with your CockroachDB credentials
./scripts/check-env.sh                # Validate configuration

# 2. Start development
cd src && cargo run --bin server      # Start server (HTMX pages + API)
# Open http://localhost:9001
```

### Development Commands
- `cd src && cargo run --bin server`: Start web server
- `cd src && cargo run --bin chat-service`: Start chat service (future)
- `cd src && cargo test --workspace`: Run all tests
- `cd wasm && trunk serve`: Develop WASM components (future)

## Architecture Evolution
- **Phase 1**: Lambda + API Gateway (~$15/month)
- **Phase 2**: Lambda + EC2 WebSocket (~$40/month)
- **Phase 3**: Direct ALB WebSocket (~$25/month)

## Development
- **Backend**: Rust with cargo-lambda for agile deployment
- **Frontend**: HTMX + Leptos WASM for 100% Rust stack
- **Database**: CockroachDB Cloud with PostGIS-compatible geographic features
- **Real-time**: WebSocket with WASM client and ephemeral message queuing

## Deployment
```bash
# Deploy to development
./scripts/deploy.sh dev all

# Deploy individual services
./scripts/deploy.sh dev api
./scripts/deploy.sh dev chat
```

## Project Structure
```
community-manager/
├── src/                      # Backend Rust
│   ├── server/              # Web server (HTMX + API)
│   │   ├── templates/       # Tera templates
│   │   ├── static/          # CSS, WASM, images
│   │   └── src/             # Rust code
│   ├── services/            # Microservices
│   │   └── chat-service/    # WebSocket service
│   ├── shared/              # Common Rust code
│   └── migrations/          # Database migrations
├── wasm/                    # WASM components (Leptos)
├── scripts/                 # Development automation
└── docs/                    # Documentation
    ├── DEVELOPMENT.md       # Development guide
    ├── ENVIRONMENT.md       # Environment setup
    ├── SCHEMA.md            # Database schema
    ├── MIGRATION.md         # Cloud migration guide
    └── TESTING.md           # Test suite documentation
```

## Documentation

- **[Development Guide](docs/DEVELOPMENT.md)** - Complete development setup and workflow
- **[Environment Setup](docs/ENVIRONMENT.md)** - Environment configuration template
- **[Database Schema](docs/SCHEMA.md)** - Complete database schema documentation
- **[Migration Guide](docs/MIGRATION.md)** - Cloud-first infrastructure migration
- **[HTMX + WASM Migration](docs/HTMX_WASM_MIGRATION.md)** - Frontend architecture migration
- **[Testing Guide](docs/TESTING.md)** - Comprehensive test suite documentation
- **[Project Status](docs/CLAUDE.md)** - Detailed project status and achievements

## Contributing
1. Install prerequisites: Rust, cargo-lambda
2. Configure CockroachDB Cloud connection in src/.env
3. Start development with `cd src && cargo run --bin server`
4. Follow conventional commit messages
5. Test before submitting PRs: `cd src && cargo test --workspace`

## License
MIT License - see LICENSE file for details