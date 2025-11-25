# 🚀 Development Guide - Community Manager

## Quick Start

### One-Command Development Setup

```bash
./scripts/start-all.sh
```

This single command will:
- ✅ Check all prerequisites (Rust, cargo-lambda, Node.js)
- ✅ Start backend services (API Gateway + Chat WebSocket)
- ✅ Start frontend development server
- ✅ Use tmux for multi-service management (if available)

### Manual Quick Start

```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cargo run --bin server
```

Server will be available at: **http://localhost:9001**

## 📦 Services Overview

| Service | Port | Purpose | Access |
|---------|------|---------|---------|
| **API Gateway** | 9001 | REST API with Lambda | http://localhost:9001 |
| **Chat Service** | 9002 | WebSocket service | ws://localhost:9002 |
| **Frontend** | 3000 | Next.js development server | http://localhost:3000 |
| **CockroachDB** | Cloud | Main database | Via connection string in .env |

## 🛠️ Development Scripts

| Script | Purpose |
|--------|---------|
| `./scripts/start-all.sh` | **Start all services** (backend + frontend) |
| `./scripts/start-backend.sh` | Start backend services only |
| `./scripts/start-frontend.sh` | Start frontend only |
| `./scripts/check-env.sh` | Validate environment configuration |
| `./scripts/test-suite.sh` | Run comprehensive tests |
| `./scripts/deploy.sh` | Deploy to staging/production |

## 🔧 Prerequisites

Required tools:
- **Rust & Cargo** (latest stable)
- **cargo-lambda** - Install with: `cargo install cargo-lambda`
- **Node.js 18+** - For frontend development
- **tmux** (optional) - For multi-service management: `brew install tmux`

## ⚙️ Environment Variables

Required in `/Users/mariomureddu/CascadeProjects/community-manager/src/.env`:

```bash
# Auth0
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback

# Session
SESSION_SECRET=your-random-secret-min-32-chars
SESSION_COOKIE_NAME=community_manager_session
SESSION_MAX_AGE=86400

# Database
DATABASE_URL=postgresql://user:pass@host:26257/db?sslmode=verify-full

# Logging
RUST_LOG=info
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
cd src
cargo test --workspace
```

**Result**: 204 tests passing ✅

Tests validate:
- ✅ Database connectivity with CockroachDB
- ✅ Backend compilation with rustls
- ✅ Unit tests (auth, database, error handling)
- ✅ Integration tests for API endpoints
- ✅ WebSocket connection handling

### Test Commands

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test --test pages_test

# Check compilation
cargo check --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace
```

## 📊 Database Access

**CockroachDB Cloud Console:**
- Access your cluster at: https://cockroachlabs.cloud/
- View tables, run queries, monitor performance
- Connection string format: `postgresql://user:pass@host:26257/database?sslmode=verify-full`

### Database Commands

```bash
# Run migrations
cd src && sqlx migrate run

# Test connection
psql "your-connection-string-here"

# Update SQLx cache
cargo sqlx prepare --workspace
```

## 🌐 Available Pages

- **Homepage**: http://localhost:9001/
- **Dashboard**: http://localhost:9001/dashboard
- **Communities**: http://localhost:9001/communities
- **Businesses**: http://localhost:9001/businesses
- **Governance**: http://localhost:9001/governance
- **Map/POI**: http://localhost:9001/poi
- **Chat**: http://localhost:9001/chat
- **Health**: http://localhost:9001/health

## 🔐 Auth Flow (Auth0)

1. **Login**: http://localhost:9001/auth/login
2. **Callback**: http://localhost:9001/auth/callback (automatic)
3. **Logout**: http://localhost:9001/auth/logout
4. **Current User**: http://localhost:9001/auth/me

## 🔄 Development Workflow

1. **First Time Setup:**
   ```bash
   # Check environment configuration
   ./scripts/check-env.sh
   
   # If errors, configure .env files
   ```

2. **Start Development:**
   ```bash
   # Start all services (backend + frontend)
   ./scripts/start-all.sh
   
   # Or start individually:
   ./scripts/start-backend.sh    # Backend only
   ./scripts/start-frontend.sh   # Frontend only
   ```

3. **Make Changes:**
   - Backend code auto-reloads with `cargo lambda watch`
   - Frontend auto-reloads with Next.js dev server
   - Database migrations: `cd src && sqlx migrate run`

4. **Run Tests:**
   ```bash
   cd src
   cargo test --workspace
   ```

5. **Stop Services:**
   - If using tmux: `Ctrl+B` then `D` to detach, or `tmux kill-session -t community-manager`
   - If in foreground: `Ctrl+C`

## 🏗️ Build Commands

```bash
# Build entire workspace
cargo build --workspace

# Build server only
cargo build --bin server

# Build chat service
cargo build --bin chat-service

# Build release
cargo build --workspace --release
```

## 🚨 Troubleshooting

### Environment Issues
```bash
# Validate configuration
./scripts/check-env.sh

# Check if DATABASE_URL is correct
cd src && cargo sqlx database create
```

### Database Connection Issues
```bash
# Test CockroachDB connection
psql "your-connection-string-here"

# Run migrations
cd src && sqlx migrate run
```

### Backend Issues
```bash
# Check compilation
cd src && cargo check

# Check specific service
cd src && cargo check -p server
cd src && cargo check -p chat-service
```

### Frontend Issues
```bash
# Clear Next.js cache
cd frontend && rm -rf .next

# Reinstall dependencies
cd frontend && rm -rf node_modules && npm install
```

### Tests failing with DB errors?
- SQLx uses offline mode with `.sqlx/` cached queries
- Run: `cargo test --workspace`

### Server not starting?
- Check `.env` file exists in `src/` directory
- Verify Auth0 credentials are set
- Check port 9001 is not in use

### Compilation errors?
- Run: `cargo clean`
- Then: `cargo build --workspace`

## 📁 Project Structure

```
community-manager/
├── authorizer/              # 🔐 Lambda Authorizer (standalone)
│   ├── src/main.rs          # Auth handler con caching
│   ├── Cargo.toml           # Dipendenze authorizer
│   └── deploy.sh            # Script deploy
│
├── src/                     # 🏗️ Main application
│   ├── server/              # Web server (Axum)
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── auth.rs      # Auth handlers (login/callback)
│   │   │   └── handlers/    # API & page handlers
│   │   ├── templates/       # Tera HTMX templates
│   │   └── static/          # CSS, JS, assets
│   │
│   ├── services/
│   │   └── chat-service/    # WebSocket chat service
│   │
│   ├── shared/              # Shared library
│   │   └── src/
│   │       ├── database/    # Database connection
│   │       └── models/      # Data models
│   │
│   └── migrations/          # SQLx database migrations
│
├── scripts/
│   ├── start-all.sh         # 🚀 Start all services
│   ├── start-backend.sh     # Start backend only
│   ├── start-frontend.sh    # Start frontend only
│   ├── check-env.sh         # Validate environment
│   └── deploy.sh            # Deploy to staging/prod
│
├── Cargo.toml               # 📦 Workspace root
├── .env                     # Environment variables
└── docs/                    # Documentation
```

## 💡 Development Tips

- **tmux Management**: Use tmux for running multiple services in one terminal
- **Hot Reloading**: Both backend and frontend auto-reload on code changes
- **Cloud Database**: CockroachDB Cloud provides automatic backups and scaling
- **Environment Check**: Run `./scripts/check-env.sh` to validate configuration
- **Individual Services**: Start only what you need with individual scripts
- **SQLx Offline Mode**: Tests run without database connection using cached queries

## 📊 Project Status

✅ **Server compiles and runs**
✅ **204 tests passing**
✅ **11 HTMX pages working**
✅ **Auth0 integration ready**
✅ **Session management configured**
✅ **Database connected (CockroachDB Cloud)**
✅ **All migrations applied**
⏳ **WASM components pending**
⏳ **Mobile app planning**

---

**Need Help?** Check other documentation files or run `./scripts/check-env.sh` to diagnose issues.
