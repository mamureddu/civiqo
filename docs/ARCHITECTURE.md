# 🏗️ Architecture & Project Status - Community Manager

## Project Overview

**Community Manager** is a comprehensive webapp for local community management with the following core features:

1. **Multi-Role User Management**: Role-based access control with community-specific permissions
2. **Geographic Community Creation**: Location-based communities with mapping integration
3. **End-to-End Encrypted Chat**: Real-time messaging with no server-side message storage
4. **Local Business Integration**: Business directory with geographic mapping and discovery
5. **Democratic Governance Tools**: Polls, voting systems, and community decision-making
6. **Progressive Web App**: Mobile-responsive with PWA features
7. **Future Mobile App**: React Native app planned for later phases

## Technology Stack

### Backend Architecture
- **Language**: Rust
- **Deployment Strategy**: Progressive serverless scaling
  - Phase 1: AWS Lambda (cargo-lambda deployment)
  - Phase 2: EC2 instances for scale
  - Phase 3: Direct ALB WebSocket for cost optimization
- **Database**: CockroachDB Cloud (PostgreSQL-compatible, unified dev/prod)
- **ORM**: SQLx with raw SQL queries and compile-time checking
- **Authentication**: Auth0 with custom role management
- **Message Routing**: AWS SQS/SNS for stateless WebSocket connections
- **File Storage**: AWS S3 with presigned URLs
- **TLS**: rustls (not OpenSSL) for Lambda compatibility
- **Local Development**: cargo-lambda watch for Lambda emulation

### Frontend Architecture
- **Base Layer**: HTMX for server-side rendered interactions
- **Interactive Layer**: Leptos WASM for complex features (chat, maps)
- **Templating**: Tera (Rust templates)
- **Styling**: TailwindCSS
- **Micro-interactions**: Alpine.js
- **Authentication**: Session-based with cookies (server-side)
- **Real-time**: WebSocket + WASM components
- **Mobile**: Native Android (Kotlin) + iOS (Swift) - separate from web

## Project Structure

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

## Current Status

### ✅ Completed Achievements

#### 1. **Infrastructure & Setup** ✅
- [x] Struttura progetto MVC
- [x] Database CockroachDB Cloud integrato
- [x] Migrations (6) applicate
- [x] Server Axum con HTMX
- [x] Lambda Authorizer con context injection
- [x] 204 tests passing
- [x] Cloud-first development stack
- [x] Zero compilation errors (rustls migration)

#### 2. **Authentication & Authorization** ✅
- [x] Auth0 handlers (login, callback, logout)
- [x] OAuth2 code exchange implementation
- [x] User sync to database
- [x] Session management
- [x] Protected routes with AuthUser extractor
- [x] Dashboard with user authentication

#### 3. **API & Database** ✅
- [x] API REST endpoints (users, communities, posts)
- [x] Database connection pooling
- [x] SQLx offline mode with cached queries
- [x] Templates Tera per pagine principali
- [x] HTMX endpoints for dynamic loading

#### 4. **Frontend & UI** ✅
- [x] Base template with conditional auth
- [x] Dashboard page with real data
- [x] User communities and activity endpoints
- [x] TailwindCSS integration
- [x] Mobile-responsive design

### 🚧 In Progress

#### 1. **Community CRUD Operations** 🚧
- [ ] Create community endpoint (POST /api/communities)
- [ ] Edit community endpoint (PUT /api/communities/:id)
- [ ] Delete community endpoint (DELETE /api/communities/:id)
- [ ] Community members management

#### 2. **Chat Service Enhancement** 🚧
- [ ] WebSocket JWT authentication
- [ ] Message persistence (optional)
- [ ] Room management UI
- [ ] Real-time notifications

### ⏳ Next Phases

#### Phase 2: Business Features (3-5 days)
- [ ] Business directory implementation
- [ ] Geographic mapping integration
- [ ] Business discovery and search
- [ ] Business owner dashboard

#### Phase 3: Governance Tools (5-7 days)
- [ ] Polls and voting system
- [ ] Community decision-making
- [ ] Role-based permissions
- [ ] Community moderation

#### Phase 4: Advanced Features (7-10 days)
- [ ] WASM components (Leptos)
- [ ] Real-time chat with WebSocket
- [ ] File upload with S3
- [ ] Mobile PWA features

#### Phase 5: Production Deployment (3-5 days)
- [ ] AWS Lambda deployment
- [ ] API Gateway configuration
- [ ] Domain and SSL setup
- [ ] Monitoring and logging

## Architecture Patterns

### 1. **Authentication Flow**
```
User → API Gateway → Authorizer (validate) → Backend (authorized)
                          ↓
                     Cache (1h)
```

### 2. **MVC Architecture**
- **Model**: `src/shared/src/models/`
- **View**: `src/server/templates/` (Tera HTMX)
- **Controller**: `src/server/src/handlers/`

### 3. **Service Separation**
- **Authorizer**: Standalone Lambda (authentication)
- **Server**: Web app + API (business logic)
- **Chat**: WebSocket service (real-time)
- **Shared**: Common code (models, DB)

### 4. **Database Architecture**
```
┌─────────────────────────────────────────────────────────┐
│                CockroachDB Cloud                         │
│  - Unified dev/prod environment                          │
│  - PostgreSQL-compatible                                 │
│  - Automatic backups and scaling                         │
│  - 22 production tables                                  │
└─────────────────────────────────────────────────────────┘
```

## Key Technical Decisions

### Why Rust?
- **Performance**: Zero-cost abstractions and memory safety
- **Concurrency**: Async/await with Tokio runtime
- **Safety**: Compile-time guarantees for production reliability
- **WebAssembly**: Native WASM support for frontend components

### Why CockroachDB Cloud?
- **Unified Environment**: Same database in dev and production
- **PostgreSQL Compatible**: Works with existing SQLx ecosystem
- **Auto-scaling**: Handles growth without manual intervention
- **Global Distribution**: Multi-region deployment capability

### Why HTMX + WASM?
- **Progressive Enhancement**: Basic functionality works without JavaScript
- **Performance**: Server-side rendering with selective client-side interactivity
- **Developer Experience**: Rust for both backend and frontend WASM
- **Mobile Ready**: PWA capabilities with minimal complexity

### Why Auth0?
- **Enterprise Security**: OAuth2/OpenID Connect standards
- **Social Logins**: Google, Facebook, etc. out of the box
- **Role Management**: Custom roles and permissions
- **Scalability**: Handles millions of users

## Performance & Scaling

### Current Performance
- **Server Startup**: ~2 seconds cold start
- **API Response**: <100ms average
- **Database Queries**: <50ms with connection pooling
- **Test Suite**: 204 tests in ~22 seconds

### Scaling Strategy
1. **Phase 1**: AWS Lambda (auto-scaling, pay-per-use)
2. **Phase 2**: EC2 instances for consistent load
3. **Phase 3**: ALB + WebSocket for cost optimization

### Monitoring & Observability
- **Logging**: Structured logging with tracing
- **Metrics**: CloudWatch integration planned
- **Error Tracking**: Sentry integration planned
- **Health Checks**: `/health` endpoint implemented

## Security Considerations

### Implemented Security
- **Authentication**: Auth0 OAuth2 with session management
- **Authorization**: Lambda authorizer with caching
- **Data Validation**: Input validation on all endpoints
- **TLS**: rustls for secure connections
- **CORS**: Proper cross-origin resource sharing

### Planned Security Enhancements
- [ ] Rate limiting per user
- [ ] CSRF protection
- [ ] Content Security Policy
- [ ] Security headers middleware
- [ ] Audit logging

## Development Workflow

### Local Development
```bash
# Start all services
./scripts/start-all.sh

# Run tests
cargo test --workspace

# Build for deployment
cargo build --workspace --release
```

### Code Quality
- **Testing**: 204 tests with >90% coverage
- **Linting**: cargo clippy for code quality
- **Formatting**: cargo fmt for consistent style
- **Dependencies**: Regular security updates

### CI/CD Pipeline
- **GitHub Actions**: Automated testing on PR
- **Deployment**: Automated deploy to staging
- **Rollback**: One-click rollback capability
- **Monitoring**: Post-deployment health checks

---

**Last Updated**: November 25, 2025  
**Next Review**: After Community CRUD completion  
**Architecture Owner**: Development Team
