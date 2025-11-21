# 📁 Project Structure

## Overview

```
community-manager/
├── authorizer/              # 🔐 Lambda Authorizer (standalone)
│   ├── src/
│   │   └── main.rs         # Auth handler con caching
│   ├── Cargo.toml          # Dipendenze authorizer
│   ├── README.md           # Guida authorizer
│   ├── CACHING_WARNING.md  # ⚠️ Guida critica caching
│   ├── build.sh            # Script build
│   ├── deploy.sh           # Script deploy
│   └── test-event.json     # Evento test
│
├── src/                     # 🏗️ Main application
│   ├── server/             # Web server (Axum)
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── auth.rs     # Auth handlers (login/callback)
│   │   │   └── handlers/   # API & page handlers
│   │   ├── templates/      # Tera HTMX templates
│   │   └── static/         # CSS, JS, assets
│   │
│   ├── services/
│   │   └── chat-service/   # WebSocket chat service
│   │
│   ├── shared/             # Shared library
│   │   └── src/
│   │       ├── database/   # Database connection
│   │       └── models/     # Data models
│   │
│   └── migrations/         # SQLx database migrations
│
├── Cargo.toml              # 📦 Workspace root
├── .env                    # Environment variables
└── docs/                   # Documentation
    ├── LAMBDA_AUTHORIZER_GUIDE.md
    ├── AUTH_GUIDE.md
    ├── API_GUIDE.md
    └── USAGE_GUIDE.md
```

## Components

### 🔐 Authorizer (Standalone Lambda)

**Location**: `/authorizer`

**Purpose**: AWS Lambda function per autenticazione e autorizzazione con caching.

**Key Features**:
- ✅ Token validation (JWT)
- ✅ IAM policy generation con wildcard
- ✅ User context injection
- ✅ Caching fino a 1 ora
- ✅ 99% riduzione costi

**Build & Deploy**:
```bash
cd authorizer
./build.sh          # Build per AWS Lambda
./deploy.sh         # Deploy su AWS
cargo test          # Run tests
```

**Documentation**:
- `authorizer/README.md` - Guida completa
- `authorizer/CACHING_WARNING.md` - ⚠️ CRITICO: Guida caching

### 🌐 Server (Web Application)

**Location**: `/src/server`

**Purpose**: Web server Axum con HTMX pages e API endpoints.

**Key Features**:
- ✅ HTMX server-side rendering
- ✅ REST API endpoints
- ✅ Auth0 integration
- ✅ Database connection
- ✅ Session management

**Run**:
```bash
cd src
cargo run --bin server
```

### 💬 Chat Service

**Location**: `/src/services/chat-service`

**Purpose**: WebSocket service per chat real-time.

**Key Features**:
- ✅ WebSocket connections
- ✅ Room management
- ✅ Message routing
- ✅ Rate limiting

### 📚 Shared Library

**Location**: `/src/shared`

**Purpose**: Codice condiviso tra tutti i servizi.

**Contains**:
- Database connection pool
- Data models
- Error types
- Utilities

## Workspace Configuration

### Root Cargo.toml

```toml
[workspace]
members = [
    "src/server",
    "src/services/chat-service",
    "src/shared",
    "authorizer"              # ← Authorizer nel workspace
]
```

### Shared Dependencies

Tutte le dipendenze comuni sono definite in `[workspace.dependencies]`:
- `tokio` - Async runtime
- `serde` - Serialization
- `sqlx` - Database
- `lambda_runtime` - Lambda (per authorizer)
- `tracing` - Logging

## Build Commands

### Build All

```bash
# Build tutto il workspace
cargo build --workspace

# Build release
cargo build --workspace --release
```

### Build Specific Components

```bash
# Server
cargo build --bin server

# Chat service
cargo build --bin chat-service

# Authorizer (per AWS Lambda)
cd authorizer && cargo lambda build --release --arm64
```

## Test Commands

```bash
# Test tutto
cargo test --workspace

# Test server
cargo test --package server

# Test authorizer
cd authorizer && cargo test
```

## Deploy Commands

### Server (Local/Cloud)

```bash
# Local development
cd src && cargo run --bin server

# Deploy to cloud (esempio con fly.io)
fly deploy
```

### Authorizer (AWS Lambda)

```bash
cd authorizer

# Deploy con cargo-lambda
cargo lambda deploy authorizer

# Deploy con SAM
sam build && sam deploy

# Deploy con Serverless Framework
serverless deploy
```

## Environment Variables

### Server (.env)

```bash
# Database
DATABASE_URL=postgresql://...

# Auth0
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=...
AUTH0_CLIENT_SECRET=...

# Session
SESSION_SECRET=...
```

### Authorizer (AWS Lambda)

```bash
# Set via AWS Console or CLI
RUST_LOG=info
AUTH0_DOMAIN=your-tenant.auth0.com
JWT_SECRET=...
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    API Gateway                          │
│  - Routes requests                                      │
│  - Caches authorizer responses (1 hour)                 │
└────────────┬────────────────────────────┬───────────────┘
             │                            │
             ▼                            ▼
    ┌────────────────┐          ┌──────────────────┐
    │   Authorizer   │          │   Backend APIs   │
    │   (Lambda)     │          │   (Lambda/Server)│
    │                │          │                  │
    │ - Validate JWT │          │ - Business logic │
    │ - Gen policy   │          │ - Database ops   │
    │ - Inject ctx   │          │ - Response       │
    └────────────────┘          └────────┬─────────┘
                                         │
                                         ▼
                                ┌─────────────────┐
                                │  CockroachDB    │
                                │  Cloud          │
                                └─────────────────┘
```

## Key Patterns

### 1. Authentication Flow

```
User → API Gateway → Authorizer (validate) → Backend (authorized)
                          ↓
                     Cache (1h)
```

### 2. MVC Architecture

- **Model**: `src/shared/src/models/`
- **View**: `src/server/templates/` (Tera HTMX)
- **Controller**: `src/server/src/handlers/`

### 3. Service Separation

- **Authorizer**: Standalone Lambda (authentication)
- **Server**: Web app + API (business logic)
- **Chat**: WebSocket service (real-time)
- **Shared**: Common code (models, DB)

## Documentation

- `LAMBDA_AUTHORIZER_GUIDE.md` - Guida completa authorizer
- `AUTH_GUIDE.md` - Autenticazione con Auth0
- `API_GUIDE.md` - API endpoints
- `USAGE_GUIDE.md` - Come usare il sistema
- `DATABASE_INTEGRATION.md` - Setup database

## Quick Start

```bash
# 1. Clone repo
git clone <repo-url>
cd community-manager

# 2. Setup environment
cp .env.example .env
# Edit .env with your values

# 3. Run migrations
cd src
sqlx migrate run

# 4. Start server
cargo run --bin server

# 5. Build authorizer (optional)
cd ../authorizer
./build.sh

# 6. Deploy authorizer (optional)
./deploy.sh
```

## Tips

- **Authorizer è standalone**: Può essere deployato indipendentemente
- **Workspace unificato**: Tutte le dipendenze condivise
- **cargo-lambda**: Usa per build e deploy authorizer
- **Caching critico**: Leggi `authorizer/CACHING_WARNING.md`
- **Test locale**: Usa `cargo lambda invoke` per testare authorizer

## Support

- Issues: GitHub Issues
- Docs: `/docs` directory
- Examples: `authorizer/test-event.json`
