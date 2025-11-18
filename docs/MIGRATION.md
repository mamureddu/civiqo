# Cloud-First Development Migration Guide

This document explains the migration from Docker-based local development to a cloud-first approach using CockroachDB Cloud and native services.

## Overview

### What Changed

**Before (Docker-based):**
- Local PostgreSQL in Docker container
- LocalStack for AWS mocking
- Redis in Docker
- docker-compose for orchestration
- Slower startup, more resource usage

**After (Cloud-first):**
- CockroachDB Cloud for database (unified dev/prod)
- Native cargo-lambda watch for backend
- Native Next.js dev server for frontend
- No Docker required
- Faster, lighter, more consistent with production

---

## Benefits

### 1. **Unified Database Environment**
- Same database technology in development and production
- No schema drift between environments
- Automatic backups and scaling
- Production-like performance testing

### 2. **Faster Development**
- No Docker overhead
- Instant hot reloading
- Native process debugging
- Lower resource consumption

### 3. **Simplified Setup**
- One connection string for database
- No container orchestration
- Fewer moving parts
- Easier troubleshooting

### 4. **Better Testing**
- Test against production-like database
- Realistic performance metrics
- No mock service limitations

---

## Migration Steps

### Step 1: CockroachDB Cloud Setup

1. **Create Account**
   - Go to https://cockroachlabs.cloud/
   - Sign up for free tier (5GB storage, 1 vCPU)

2. **Create Cluster**
   - Choose "Serverless" plan
   - Select region (eu-central-1 recommended)
   - Name your cluster (e.g., "community-manager-dev")

3. **Create Databases**
   ```sql
   CREATE DATABASE community_manager;
   CREATE DATABASE community_manager_test;
   ```

4. **Get Connection String**
   - Click "Connect" in console
   - Copy connection string
   - Format: `postgresql://user:pass@host:26257/database?sslmode=verify-full`

### Step 2: Environment Configuration

1. **Copy Environment Template**
   ```bash
   cp docs/ENVIRONMENT.md backend/.env
   ```

2. **Update DATABASE_URL**
   ```bash
   # Replace with your CockroachDB connection string
   DATABASE_URL=postgresql://your-user:your-password@your-cluster.cockroachlabs.cloud:26257/community_manager?sslmode=verify-full
   ```

3. **Configure Test Database**
   ```bash
   # In backend/.env.test
   DATABASE_URL=postgresql://your-user:your-password@your-cluster.cockroachlabs.cloud:26257/community_manager_test?sslmode=verify-full
   ```

4. **Validate Configuration**
   ```bash
   ./scripts/check-env.sh
   ```

### Step 3: Run Migrations

```bash
cd backend
sqlx migrate run
```

Verify migrations:
```bash
# Check tables exist
sqlx migrate info
```

### Step 4: Install Prerequisites

```bash
# Install cargo-lambda
cargo install cargo-lambda

# Install tmux (optional but recommended)
brew install tmux  # macOS
sudo apt-get install tmux  # Linux
```

### Step 5: Start Development

```bash
# Start all services
./scripts/start-all.sh

# Or start individually:
./scripts/start-backend.sh   # Backend only
./scripts/start-frontend.sh  # Frontend only
```

---

## Removed Components

### Docker Files (Removed)
- ❌ `docker-compose.yml` - No longer needed
- ❌ `backend/Dockerfile.dev` - Not used
- ❌ `scripts/dev.sh` - Replaced by start-all.sh
- ❌ `scripts/init-test-db.sql` - Handled by migrations

### Environment Variables (Removed)
- ❌ `POSTGRES_DB` - Included in DATABASE_URL
- ❌ `POSTGRES_USER` - Included in DATABASE_URL
- ❌ `POSTGRES_PASSWORD` - Included in DATABASE_URL
- ❌ `REDIS_URL` - Not used in architecture
- ❌ `LOCALSTACK_ENDPOINT` - Not needed for dev
- ❌ `AWS_ENDPOINT_URL` - Use real AWS or mock in tests

---

## New Scripts

### `./scripts/start-all.sh`
Starts all services (backend + frontend) using tmux for multi-service management.

**Features:**
- Checks prerequisites
- Validates environment
- Starts API Gateway (port 9001)
- Starts Chat Service (port 9002)
- Starts Frontend (port 3000)
- Uses tmux for easy management

**Usage:**
```bash
./scripts/start-all.sh

# Tmux commands:
# - Switch window: Ctrl+B then 0/1/2
# - Detach: Ctrl+B then D
# - Reattach: tmux attach -t community-manager
# - Kill all: tmux kill-session -t community-manager
```

### `./scripts/start-backend.sh`
Starts backend services only.

**Usage:**
```bash
./scripts/start-backend.sh        # Start all backend services
./scripts/start-backend.sh api    # Start API Gateway only
./scripts/start-backend.sh chat   # Start Chat Service only
```

### `./scripts/start-frontend.sh`
Starts frontend development server.

**Usage:**
```bash
./scripts/start-frontend.sh
```

### `./scripts/check-env.sh`
Validates environment configuration.

**Usage:**
```bash
./scripts/check-env.sh
```

**Checks:**
- ✅ backend/.env exists and configured
- ✅ frontend/.env.local exists and configured
- ✅ backend/.env.test exists (optional)
- ✅ All required variables present
- ✅ No placeholder values

---

## Service Ports

| Service | Port | Purpose | URL |
|---------|------|---------|-----|
| API Gateway | 9001 | REST API | http://localhost:9001 |
| Chat Service | 9002 | WebSocket | ws://localhost:9002 |
| Frontend | 3000 | Next.js | http://localhost:3000 |
| CockroachDB | Cloud | Database | Via connection string |

---

## Troubleshooting

### Database Connection Issues

**Problem:** Can't connect to CockroachDB
```
Error: connection refused
```

**Solution:**
1. Check connection string in .env
2. Verify SSL mode: `?sslmode=verify-full`
3. Test connection:
   ```bash
   psql "your-connection-string-here"
   ```

### cargo-lambda Not Found

**Problem:**
```
Error: cargo-lambda not found
```

**Solution:**
```bash
cargo install cargo-lambda
```

### Port Already in Use

**Problem:**
```
Error: Address already in use (port 9001)
```

**Solution:**
```bash
# Find process using port
lsof -i :9001

# Kill process
kill -9 <PID>
```

### tmux Not Available

**Problem:**
```
Warning: tmux not found
```

**Solution:**
```bash
# macOS
brew install tmux

# Linux
sudo apt-get install tmux

# Or run services individually without tmux
./scripts/start-backend.sh api    # Terminal 1
./scripts/start-backend.sh chat   # Terminal 2
./scripts/start-frontend.sh       # Terminal 3
```

### Migrations Fail

**Problem:**
```
Error: migration failed
```

**Solution:**
```bash
# Check current migration status
cd backend
sqlx migrate info

# Revert last migration
sqlx migrate revert

# Re-run migrations
sqlx migrate run
```

---

## Testing

### Run Tests

```bash
cd backend
cargo test --workspace
```

### Test with CockroachDB

Tests automatically use `backend/.env.test` if present, otherwise fall back to `backend/.env`.

**Best Practice:** Use separate test database:
```bash
# backend/.env.test
DATABASE_URL=postgresql://user:pass@host:26257/community_manager_test?sslmode=verify-full
```

---

## Development Workflow

### Daily Workflow

1. **Start Development**
   ```bash
   ./scripts/start-all.sh
   ```

2. **Make Changes**
   - Backend auto-reloads with cargo lambda watch
   - Frontend auto-reloads with Next.js
   - Database changes via migrations

3. **Run Tests**
   ```bash
   cd backend
   cargo test --workspace
   ```

4. **Stop Services**
   - Detach from tmux: `Ctrl+B` then `D`
   - Or kill session: `tmux kill-session -t community-manager`

### Adding Database Changes

1. **Create Migration**
   ```bash
   cd backend
   sqlx migrate add <migration_name>
   ```

2. **Edit Migration File**
   ```bash
   vim migrations/<timestamp>_<migration_name>.sql
   ```

3. **Run Migration**
   ```bash
   sqlx migrate run
   ```

4. **Update SQLx Offline Data**
   ```bash
   cargo sqlx prepare --workspace
   ```

---

## Comparison: Before vs After

| Aspect | Docker (Before) | Cloud-First (After) |
|--------|----------------|---------------------|
| **Startup Time** | ~30 seconds | ~5 seconds |
| **Memory Usage** | ~2GB | ~500MB |
| **Database** | Local PostgreSQL | CockroachDB Cloud |
| **Hot Reload** | Slower | Instant |
| **Setup Steps** | 5+ commands | 2 commands |
| **Debugging** | Container logs | Native process |
| **Production Parity** | Different DB | Same DB |
| **Backup** | Manual | Automatic |

---

## Cost Considerations

### CockroachDB Cloud Pricing

**Free Tier:**
- 5GB storage
- 1 vCPU
- 50M Request Units/month
- Perfect for development

**Paid Tier (if needed):**
- ~$1/GB storage/month
- ~$0.10/vCPU hour
- Scales automatically

**Estimated Dev Cost:** $0-5/month

---

## Rollback (If Needed)

If you need to go back to Docker:

1. **Restore docker-compose.yml** from git history
2. **Update .env** with local PostgreSQL connection
3. **Start Docker services**
   ```bash
   docker-compose up -d
   ```

However, we recommend staying with cloud-first approach for better development experience.

---

## Next Steps

1. ✅ Complete migration following this guide
2. ✅ Verify all services start correctly
3. ✅ Run test suite to ensure everything works
4. ✅ Update team documentation
5. ✅ Share CockroachDB credentials securely

---

## Support

- **CockroachDB Docs:** https://www.cockroachlabs.com/docs/
- **cargo-lambda Docs:** https://www.cargo-lambda.info/
- **Project Issues:** Check docs/DEVELOPMENT.md

---

**Migration Date**: November 18, 2025  
**Status**: Complete  
**Recommended**: Yes - Significantly better developer experience
