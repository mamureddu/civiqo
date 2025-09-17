# Community Manager - Comprehensive Context Status

## Project Overview

### Application Goals
**Community Manager** is a comprehensive webapp for local community management with the following core features:

1. **Multi-Role User Management**: Role-based access control with community-specific permissions
2. **Geographic Community Creation**: Location-based communities with mapping integration
3. **End-to-End Encrypted Chat**: Real-time messaging with no server-side message storage
4. **Local Business Integration**: Business directory with geographic mapping and discovery
5. **Democratic Governance Tools**: Polls, voting systems, and community decision-making
6. **Progressive Web App**: Mobile-responsive with PWA features
7. **Future Mobile App**: React Native app planned for later phases

### Technology Stack Decisions

#### Backend Architecture
- **Language**: Rust
- **Deployment Strategy**: Progressive serverless scaling
  - Phase 1: AWS Lambda (cargo-lambda deployment)
  - Phase 2: EC2 instances for scale
  - Phase 3: Direct ALB WebSocket for cost optimization
- **Database**: CockroachDB Serverless (PostgreSQL-compatible)
- **Authentication**: Auth0 with custom role management
- **Message Routing**: AWS SQS/SNS for stateless WebSocket connections
- **File Storage**: AWS S3 with presigned URLs
- **TLS**: rustls (not OpenSSL) for Lambda compatibility

#### Frontend Architecture
- **Framework**: Next.js 14+
- **UI Library**: Material UI v5
- **Mapping**: React Leaflet
- **Real-time**: WebSocket with E2E encryption
- **State Management**: React Context + hooks
- **Future**: React Native mobile app

#### Infrastructure Choices
- **Serverless-first**: Lambda functions with cargo-lambda
- **Stateless Design**: No persistent connections, message routing via SQS/SNS
- **Environment-based Config**: .env files for development, AWS Secrets for production
- **Progressive Enhancement**: Start simple, scale to more complex infrastructure

## Current Project Structure

### Backend Services (Rust Workspace)
```
backend/
├── Cargo.toml                 # Workspace configuration
├── api-gateway/               # Main REST API service
│   ├── src/
│   │   ├── main.rs           # Application entry point
│   │   ├── config.rs         # Environment configuration
│   │   ├── handlers/         # HTTP request handlers
│   │   │   ├── auth.rs       # Authentication endpoints
│   │   │   ├── communities.rs # Community management
│   │   │   ├── businesses.rs  # Business directory
│   │   │   ├── governance.rs  # Polls and voting
│   │   │   └── uploads.rs     # File upload management
│   │   ├── middleware/       # HTTP middleware
│   │   │   ├── auth.rs       # JWT validation & auth
│   │   │   ├── rate_limit.rs # Rate limiting protection
│   │   │   └── error_handler.rs # Security-conscious errors
│   │   └── services/         # Business logic
│   ├── Cargo.toml           # Service dependencies
│   └── deploy-*.toml        # Cargo-lambda deployment configs
├── chat-service/             # WebSocket chat service
│   ├── src/main.rs          # Placeholder implementation
│   └── Cargo.toml           # WebSocket dependencies
├── shared/                   # Common library
│   ├── src/
│   │   ├── lib.rs           # Library exports
│   │   ├── auth/mod.rs      # JWT validation & Auth0 integration
│   │   ├── database/mod.rs  # Database connection & migrations
│   │   ├── models/          # Data models and API types
│   │   ├── error.rs         # Error handling types
│   │   └── utils.rs         # Utility functions
│   └── Cargo.toml           # Shared dependencies
├── migrations/               # Database schema
│   ├── 001_initial.sql      # Users, communities, memberships
│   ├── 002_businesses.sql   # Business directory
│   ├── 003_governance.sql   # Polls and voting
│   ├── 004_file_uploads.sql # File metadata
│   └── 005_seed_data.sql    # Initial data
└── .env.test                # Test environment variables
```

### Frontend Structure (Planned)
```
frontend/
├── package.json             # Next.js and Material UI dependencies
├── next.config.js           # Next.js configuration
├── src/
│   ├── app/                 # App router structure
│   ├── components/          # Reusable UI components
│   ├── hooks/               # Custom React hooks
│   ├── contexts/            # React contexts for state
│   ├── services/            # API client functions
│   ├── utils/               # Utility functions
│   └── types/               # TypeScript type definitions
└── public/                  # Static assets
```

### Development Scripts
- `scripts/setup.sh`: Environment setup and dependencies
- `scripts/dev.sh`: Local development server startup
- `scripts/deploy.sh`: Deployment automation
- `scripts/test.sh`: Test execution

## Implementation Status

### ✅ COMPLETED (Major Achievements)

#### 1. Project Foundation
- ✅ Complete Rust workspace setup with three services
- ✅ Cargo-lambda deployment configurations (dev/staging/prod)
- ✅ Database schema with 5 comprehensive migrations
- ✅ Development automation scripts
- ✅ Environment configuration structure

#### 2. API Gateway Service (Fully Implemented)
- ✅ **REST API Endpoints**: All routes implemented
  - Authentication: `/auth/me`, `/auth/sync`, `/auth/profile`
  - Communities: Full CRUD + membership management
  - Businesses: Structure ready (placeholder implementations)
  - Governance: Structure ready (placeholder implementations)
  - File uploads: S3 presigned URL generation
- ✅ **Middleware Stack**: Complete security layer
  - JWT validation with Auth0 JWKS fetching
  - Rate limiting with user/IP tracking
  - CORS with restrictive configuration
  - Security-conscious error handling
- ✅ **Database Integration**: Connection pooling and transactions

#### 3. Shared Library (Core Infrastructure)
- ✅ **Authentication Module**: Auth0 integration with JWT validation
- ✅ **Database Module**: PostgreSQL connection with configurable pooling
- ✅ **Models**: Complete API types and database models
- ✅ **Error Handling**: Comprehensive error types with security focus
- ✅ **Utilities**: Text validation, slug generation, etc.

#### 4. Critical Security Implementation
- ✅ **JWT Security**: Proper JWKS fetching (fixed from placeholder implementation)
- ✅ **Input Validation**: Comprehensive validation using `validator` crate
- ✅ **CORS Configuration**: Restrictive instead of permissive settings
- ✅ **Rate Limiting**: User-based and IP-based protection
- ✅ **Error Sanitization**: No information leakage in error messages
- ✅ **Database Security**: Configurable pools, transaction boundaries

### 🔄 IN PROGRESS

#### 1. rustls Migration (Current Task)
- **Issue**: OpenSSL cross-compilation issues for Lambda deployment
- **Solution**: Migrating from `native-tls` to `rustls` features
- **Status**: Workspace Cargo.toml updated, testing compilation
- **Files Modified**:
  - `backend/Cargo.toml`: Updated sqlx to use `runtime-tokio-rustls`
  - `backend/api-gateway/Cargo.toml`: Already using `rustls-tls` for reqwest

#### 2. Code Validation (Next Immediate Task)
- **Plan**: Use debugger agent to validate current codebase
- **Purpose**: Ensure no compilation errors before proceeding
- **Scope**: All Rust services and dependencies

### ⏳ PENDING (Next Tasks in Order)

#### Backend Completion (3-4 tasks)
1. **Chat WebSocket Service**: Real-time messaging with connection handling
2. **CockroachDB Integration**: Replace PostgreSQL configuration
3. **AWS SQS/SNS Setup**: Message routing for stateless architecture
4. **Role-Based Access Control**: Complete RBAC implementation

#### Frontend Implementation (12+ tasks)
1. **Next.js Setup**: Material UI integration and basic routing
2. **Auth0 Frontend**: Authentication flow and user management
3. **Community UI**: Create, join, manage communities
4. **Business Directory**: Search, maps, business profiles
5. **Chat Interface**: Real-time messaging with E2E encryption
6. **Governance Tools**: Polls, voting, decision tracking
7. **Geographic Features**: Mapping with React Leaflet
8. **File Uploads**: Avatar and business image handling
9. **Mobile Optimization**: Responsive design and PWA features
10. **Testing**: End-to-end application testing
11. **Documentation**: API docs and deployment guides
12. **React Native Planning**: Mobile app architecture

## Detailed Todo List (Current Session Status)

### 🔄 **IN PROGRESS**
1. **Fix rustls dependencies across all services** - Currently updating from native-tls to rustls for Lambda compatibility

### ⏳ **PENDING - IMMEDIATE NEXT**
2. **Use debugger agent to validate current codebase** - Ensure no compilation errors before proceeding
3. **Implement chat WebSocket service with connection handling** - Real-time messaging infrastructure

### 🏗️ **PENDING - BACKEND**
4. **Configure CockroachDB integration and test connections** - Replace PostgreSQL with CockroachDB Serverless
5. **Set up AWS SQS/SNS for message routing between instances** - Stateless WebSocket message routing
6. **Implement role-based access control throughout the app** - Complete RBAC system
7. **Add file upload functionality for avatars and business images** - S3 integration with presigned URLs

### 🎨 **PENDING - FRONTEND**
8. **Set up Next.js frontend with Material UI and basic routing** - Frontend foundation
9. **Configure Material UI theme and create base components** - Design system setup
10. **Implement Auth0 integration in both frontend and backend** - Authentication flow
11. **Create authentication pages (login, signup, profile)** - User authentication UI
12. **Implement community management UI (create, join, manage)** - Core community features
13. **Implement business directory UI with search and maps** - Business discovery
14. **Implement real-time chat interface with E2EE** - Encrypted messaging UI
15. **Implement governance tools UI (polls, voting, decisions)** - Democratic tools
16. **Add geographic mapping features with React Leaflet** - Location-based features

### 🚀 **PENDING - OPTIMIZATION**
17. **Test the complete application flow end-to-end** - Full integration testing
18. **Optimize for mobile responsiveness and PWA features** - Mobile experience
19. **Prepare API documentation and deployment guides** - Documentation
20. **Plan React Native app architecture and shared components** - Mobile app planning

### 📊 **Progress Summary**
- **Overall Progress**: ~15% Complete
- **Current Focus**: rustls migration → debugger validation → chat service
- **Completed**: 3 major foundation tasks (deployment configs, API Gateway, security fixes)
- **Active**: 1 task (rustls dependencies)
- **Pending**: 19 tasks across backend and frontend

## Technical Decisions Made

### Security Architecture
1. **Authentication**: Auth0 with JWT tokens, JWKS validation
2. **Authorization**: Role-based access control with community-specific permissions
3. **Rate Limiting**: 100 requests per minute per user/IP
4. **Input Validation**: Comprehensive validation on all endpoints
5. **Error Handling**: Generic error messages to prevent information leakage
6. **CORS**: Restrictive configuration with specific origins/headers
7. **File Security**: Type validation, size limits, user-scoped storage

### Database Design
1. **Schema**: Users, communities, memberships, businesses, governance, files
2. **Relationships**: Many-to-many with role-based memberships
3. **Transactions**: Atomic operations for critical data changes
4. **Pooling**: Configurable connection pools for performance
5. **Migrations**: Version-controlled schema evolution

### Deployment Strategy
1. **Progressive Scaling**: Lambda → EC2 → Direct WebSocket
2. **Environment Management**: .env for dev, AWS Secrets for prod
3. **Cargo-lambda**: Rust-native Lambda deployment tool
4. **rustls**: TLS implementation optimized for serverless

### API Design
1. **RESTful**: Standard HTTP methods and status codes
2. **JSON**: Consistent request/response format
3. **Pagination**: Built-in pagination for list endpoints
4. **Versioning**: Prepared for API evolution
5. **Error Format**: Standardized error response structure

## Environment Configuration

### Required Environment Variables
```bash
# Database Configuration
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/community_manager
TEST_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/community_manager_test

# Database Pool Settings
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=5
DB_ACQUIRE_TIMEOUT_SECONDS=8

# Auth0 Configuration
AUTH0_DOMAIN=your-domain.auth0.com
AUTH0_AUDIENCE=your-audience
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret

# API Configuration
CORS_ORIGINS=http://localhost:3000
DEVELOPMENT_MODE=true

# AWS Configuration
AWS_REGION=us-east-1
S3_BUCKET=community-manager-uploads
S3_REGION=us-east-1

# Logging
LOG_LEVEL=info
```

### Current Environment Files
- `.env.test`: Test configuration with placeholder values
- Need to create: `.env.development`, `.env.staging`, `.env.production`

## Dependencies and Versions

### Workspace Dependencies (Cargo.toml)
```toml
# Runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "chrono", "uuid"] }

# HTTP and Lambda
lambda_runtime = "0.8"
lambda-web = "0.2.1"
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Auth
jsonwebtoken = "9.0"

# AWS SDK
aws-config = "1.0"
aws-sdk-s3 = "1.0"
aws-sdk-sqs = "1.0"
aws-sdk-sns = "1.0"

# Additional
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.16", features = ["derive"] }
```

### Service-Specific Dependencies
- **api-gateway**: reqwest (rustls-tls), validator, lazy_static, regex
- **chat-service**: tokio-tungstenite, futures-util, dashmap
- **shared**: All core dependencies from workspace

## Known Issues and Blockers

### Current Blockers
1. **rustls Migration**: Need to complete transition from native-tls
2. **Compilation Testing**: Need to verify all services compile with rustls
3. **Context Misalignment**: Two conversation contexts need synchronization

### Technical Debt
1. **Placeholder Implementations**: Business and governance handlers need completion
2. **Development Feature Flags**: Auth module has development-only features
3. **Error Recovery**: Some middleware could have better error recovery
4. **Testing**: Unit tests exist but integration tests needed

### Future Considerations
1. **Database Migration**: PostgreSQL → CockroachDB transition
2. **Scaling**: Lambda → EC2 migration strategy
3. **WebSocket Evolution**: Direct ALB integration for cost optimization
4. **Mobile App**: React Native implementation strategy

## Recent Conversation History

### Last Session Achievements
1. **Security Review**: Used code-reviewer agent to identify critical vulnerabilities
2. **Security Fixes**: Implemented all critical security improvements
3. **JWT Implementation**: Fixed placeholder JWT validation with proper JWKS
4. **Input Validation**: Added comprehensive validation across all endpoints
5. **Rate Limiting**: Implemented user and IP-based rate limiting
6. **Error Handling**: Created security-conscious error response system

### User Requests and Decisions
1. **User Priority**: Fix security issues before proceeding (explicitly chosen option "a")
2. **Architecture Preference**: Serverless/stateless design over traditional monolith
3. **Deployment Strategy**: cargo-lambda for agile development and deployment
4. **Security Focus**: Accept env file credentials for development, proper secrets in production
5. **Technology Choices**: Material UI over custom components, rustls over OpenSSL

### Current User Context
- User is managing two conversation contexts that became misaligned
- User wants to synchronize contexts using this comprehensive status
- User specifically requested rustls implementation
- User wants to use debugger agent for code validation

## Next Immediate Steps

### Priority 1: Technical Foundation
1. Complete rustls migration and test compilation
2. Run debugger agent to validate current codebase
3. Fix any compilation or runtime issues discovered

### Priority 2: Backend Completion
1. Implement chat WebSocket service with connection handling
2. Complete business and governance placeholder implementations
3. Set up CockroachDB integration
4. Implement AWS SQS/SNS message routing

### Priority 3: Frontend Development
1. Set up Next.js project with Material UI
2. Implement Auth0 authentication flow
3. Build core UI components and layouts
4. Implement real-time features

## Development Commands

### Current Working Directory
`/Users/mariomureddu/CascadeProjects/community-manager/backend`

### Key Commands
```bash
# Build with cargo-lambda
cargo lambda build --package api-gateway

# Run locally
cargo lambda watch --package api-gateway

# Deploy
cargo lambda deploy --package api-gateway

# Test
cargo test

# Check compilation
cargo check
```

## File Locations Summary

### Critical Files Modified in This Context
- `backend/Cargo.toml`: Workspace dependencies with rustls
- `backend/api-gateway/src/main.rs`: Application setup with security middleware
- `backend/api-gateway/src/handlers/auth.rs`: JWT validation and user management
- `backend/api-gateway/src/handlers/communities.rs`: Community CRUD with validation
- `backend/api-gateway/src/handlers/uploads.rs`: File upload with security validation
- `backend/api-gateway/src/middleware/`: Rate limiting and error handling
- `backend/shared/src/auth/mod.rs`: Auth0 integration with JWKS
- `backend/shared/src/database/mod.rs`: Database connection with configurable pooling

### Configuration Files
- `backend/.env.test`: Test environment configuration
- `backend/api-gateway/deploy-dev.toml`: Development deployment config
- `backend/api-gateway/deploy-staging.toml`: Staging deployment config
- `backend/api-gateway/deploy-prod.toml`: Production deployment config

This document represents the complete state of the Community Manager project as of the current conversation context. Use this to align with other contexts and continue development from the current state.