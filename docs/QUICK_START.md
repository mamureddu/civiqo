# Quick Start Guide

## 🚀 Launch Server

```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cargo run --bin server
```

Server will be available at: **http://localhost:9001**

## 🧪 Run Tests

```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cargo test --workspace
```

**Result**: 204 tests passing ✅

## 📄 Available Pages

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

## 📚 Documentation

- **`TESTING.md`** - Complete testing guide
- **`TEST_CLEANUP_SUMMARY.md`** - Recent cleanup details
- **`server/tests/disabled/README.md`** - Disabled tests info

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

# Logging
RUST_LOG=info

# Database (optional, for integration tests)
DATABASE_URL=postgresql://user:pass@host:26257/db?sslmode=verify-full
```

## 🏗️ Build

```bash
# Build entire workspace
cargo build --workspace

# Build server only
cargo build --bin server

# Build chat service
cargo build --bin chat-service
```

## 📊 Project Status

✅ **Server compiles and runs**
✅ **204 tests passing**
✅ **11 HTMX pages working**
✅ **Auth0 integration ready**
✅ **Session management configured**
⏳ **Database integration pending**
⏳ **WASM components pending**

## 🔧 Common Commands

```bash
# Run server
cd src && cargo run --bin server

# Run tests
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

## 🐛 Troubleshooting

**Tests failing with DB errors?**
- SQLx uses offline mode with `.sqlx/` cached queries
- Run: `cargo test --workspace`

**Server not starting?**
- Check `.env` file exists in `src/` directory
- Verify Auth0 credentials are set
- Check port 9001 is not in use

**Compilation errors?**
- Run: `cargo clean`
- Then: `cargo build --workspace`

## 📞 Support

See detailed documentation in:
- `TESTING.md` for test information
- `TEST_CLEANUP_SUMMARY.md` for recent changes
- Individual README files in `disabled/` directories
