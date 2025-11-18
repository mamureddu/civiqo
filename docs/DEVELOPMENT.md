# Community Manager - Development Guide

## 🚀 Quick Start

### One-Command Development Setup

```bash
./scripts/start-all.sh
```

This single command will:
- ✅ Check all prerequisites (Rust, cargo-lambda, Node.js)
- ✅ Start backend services (API Gateway + Chat WebSocket)
- ✅ Start frontend development server
- ✅ Use tmux for multi-service management (if available)

## 📦 What Gets Started

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

## 🧪 Testing

Run the comprehensive test suite:

```bash
cd backend
cargo test --workspace
```

Tests validate:
- ✅ Database connectivity with CockroachDB
- ✅ Backend compilation with rustls
- ✅ Unit tests (auth, database, error handling)
- ✅ Integration tests for API endpoints
- ✅ WebSocket connection handling

## 🔧 Prerequisites

Required tools:
- **Rust & Cargo** (latest stable)
- **cargo-lambda** - Install with: `cargo install cargo-lambda`
- **Node.js 18+** - For frontend development
- **tmux** (optional) - For multi-service management: `brew install tmux`

## 📊 Database Access

**CockroachDB Cloud Console:**
- Access your cluster at: https://cockroachlabs.cloud/
- View tables, run queries, monitor performance
- Connection string format: `postgresql://user:pass@host:26257/database?sslmode=verify-full`

## ☁️ AWS Services

For local development:
- Use mock AWS credentials in .env (for testing)
- For production features (S3, SQS), configure real AWS credentials
- LocalStack is no longer used in development

## 🔄 Development Workflow

1. **First Time Setup:**
   ```bash
   # Check environment configuration
   ./scripts/check-env.sh
   
   # If errors, copy ENV_TEMPLATE.md to .env files and configure
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
   - Database migrations: `cd backend && sqlx migrate run`

4. **Run Tests:**
   ```bash
   cd backend
   cargo test --workspace
   ```

5. **Stop Services:**
   - If using tmux: `Ctrl+B` then `D` to detach, or `tmux kill-session -t community-manager`
   - If in foreground: `Ctrl+C`

## 🚨 Troubleshooting

### Environment Issues
```bash
# Validate configuration
./scripts/check-env.sh

# Check if DATABASE_URL is correct
cd backend && cargo sqlx database create
```

### Database Connection Issues
```bash
# Test CockroachDB connection
psql "your-connection-string-here"

# Run migrations
cd backend && sqlx migrate run
```

### Backend Issues
```bash
# Check compilation
cd backend && cargo check

# Check specific service
cd backend && cargo check -p api-gateway
cd backend && cargo check -p chat-service
```

### Frontend Issues
```bash
# Clear Next.js cache
cd frontend && rm -rf .next

# Reinstall dependencies
cd frontend && rm -rf node_modules && npm install
```

## 📁 Project Structure

```
community-manager/
├── backend/
│   ├── .env                    # Backend environment variables
│   ├── api-gateway/           # REST API service
│   ├── chat-service/          # WebSocket service
│   ├── shared/                # Common library
│   └── migrations/            # Database migrations
├── frontend/
│   ├── .env.local             # Frontend environment variables
│   └── src/                   # Next.js application
├── scripts/
│   ├── start-all.sh           # 🚀 Start all services
│   ├── start-backend.sh       # Start backend only
│   ├── start-frontend.sh      # Start frontend only
│   ├── check-env.sh           # Validate environment
│   └── deploy.sh              # Deploy to staging/prod
├── ENV_TEMPLATE.md            # Environment configuration template
└── README-DEV.md             # This file
```

## 🌐 Service URLs

Once the development stack is running:

1. **API Testing:** http://localhost:9001/health
2. **Frontend:** http://localhost:3000
3. **Database Console:** https://cockroachlabs.cloud/
4. **WebSocket Testing:** ws://localhost:9002

## 💡 Tips

- **tmux Management:** Use tmux for running multiple services in one terminal
- **Hot Reloading:** Both backend and frontend auto-reload on code changes
- **Cloud Database:** CockroachDB Cloud provides automatic backups and scaling
- **Environment Check:** Run `./scripts/check-env.sh` to validate configuration
- **Individual Services:** Start only what you need with individual scripts

---

**Need Help?** Check ENV_TEMPLATE.md for configuration examples or run `./scripts/check-env.sh` to diagnose issues.
