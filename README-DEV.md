# Community Manager - Development Guide

## 🚀 Quick Start

### One-Command Development Setup

```bash
./scripts/dev-launch.sh
```

This single command will:
- ✅ Check all prerequisites (Docker, Rust, cargo-lambda)
- ✅ Start all Docker services (PostgreSQL, Redis, LocalStack, Adminer)
- ✅ Apply database migrations
- ✅ Validate backend compilation
- ✅ Start the backend API with hot reloading

## 📦 What Gets Started

| Service | Port | Purpose | Access |
|---------|------|---------|---------|
| **PostgreSQL** | 5432 | Main database | `dev:dev123@localhost:5432/community_manager` |
| **Adminer** | 8080 | Database management UI | http://localhost:8080 |
| **Redis** | 6379 | Caching & sessions | `localhost:6379` |
| **LocalStack** | 4566 | AWS services mock | http://localhost:4566 |
| **Backend API** | 9001 | REST API with Lambda | http://localhost:9001 |

## 🛠️ Development Scripts

| Script | Purpose |
|--------|---------|
| `./scripts/dev-launch.sh` | **Start everything** (recommended) |
| `./scripts/dev-status.sh` | Check service status |
| `./scripts/dev-logs.sh` | View service logs |
| `./scripts/dev-stop.sh` | Stop all services |
| `./scripts/dev-reset.sh` | Reset environment |
| `./scripts/test-suite.sh` | Run comprehensive tests |

## 🧪 Testing

The development launcher automatically runs a comprehensive test suite that validates:

- ✅ Docker services health
- ✅ Database connectivity and migrations
- ✅ Backend compilation with rustls
- ✅ Unit tests (auth, database, error handling)
- ✅ Environment configuration
- ✅ Development scripts functionality

## 🔧 Prerequisites

The launcher will check and help install:
- **Docker Desktop** (required)
- **Rust & Cargo** (required)
- **cargo-lambda** (auto-installed)

## 📊 Database Access

**Adminer Web UI:** http://localhost:8080
- Server: `postgres`
- Username: `dev`
- Password: `dev123`
- Database: `community_manager`

**Direct Connection:**
```bash
psql postgresql://dev:dev123@localhost:5432/community_manager
```

## ☁️ AWS Services (LocalStack)

All AWS services are mocked locally:
- **S3:** http://localhost:4566
- **SQS:** http://localhost:4566
- **SNS:** http://localhost:4566
- **Health Check:** http://localhost:4566/_localstack/health

## 🔄 Development Workflow

1. **Start Development:**
   ```bash
   ./scripts/dev-launch.sh
   ```

2. **Make Changes:**
   - Backend code auto-reloads with cargo lambda watch
   - Database changes persist in Docker volumes

3. **Run Tests (after everything is running):**
   ```bash
   ./scripts/test-suite.sh
   ```

4. **Check Status:**
   ```bash
   ./scripts/dev-status.sh
   ```

5. **Stop When Done:**
   ```bash
   ./scripts/dev-stop.sh
   ```

## 🚨 Troubleshooting

### Docker Issues
```bash
# Check Docker is running
docker info

# Restart Docker services
./scripts/dev-reset.sh
```

### Database Issues
```bash
# Check PostgreSQL
docker-compose logs postgres

# Reset database
./scripts/dev-reset.sh  # Choose 'y' to remove volumes
```

### Backend Issues
```bash
# Check compilation
cd backend && cargo check

# View backend logs
./scripts/dev-logs.sh
```

## 📁 Project Structure

```
community-manager/
├── backend/
│   ├── .env                    # Environment variables
│   ├── api-gateway/           # REST API service
│   ├── chat-service/          # WebSocket service
│   ├── shared/                # Common library
│   └── migrations/            # Database migrations
├── scripts/
│   ├── dev-launch.sh          # 🚀 Master launcher
│   ├── dev-status.sh          # Status checker
│   ├── dev-stop.sh            # Stop services
│   ├── dev-reset.sh           # Reset environment
│   └── test-suite.sh          # Comprehensive tests
├── docker-compose.yml         # Docker services
└── README-DEV.md             # This file
```

## 🎯 Next Steps After Setup

Once the development stack is running:

1. **API Testing:** http://localhost:9001/health
2. **Database Exploration:** http://localhost:8080
3. **Frontend Development:** Ready for Next.js setup
4. **AWS Integration:** LocalStack ready at :4566

## 💡 Tips

- **One Terminal:** Everything runs in one terminal with clear status updates
- **Persistent Data:** Database data survives container restarts
- **Hot Reloading:** Backend automatically reloads on code changes
- **Comprehensive Testing:** Built-in test suite validates everything
- **Easy Cleanup:** Reset script for fresh environment

---

**Ready to develop!** 🎉