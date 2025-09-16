# Backend Services - Cargo Lambda Deployment

Rust microservices with cargo-lambda for rapid development and deployment.

## Architecture Overview

This backend uses a microservices architecture with two main services:
- **api-gateway**: REST API handling all non-realtime operations
- **chat-service**: WebSocket service for real-time messaging with E2EE

## Services

### api-gateway/
REST API service handling:
- User management and profiles
- Community CRUD operations
- Business profile management
- Governance tools (polls, voting)
- Role-based authorization
- File upload coordination

**Deployment**: Always runs on Lambda for cost-effectiveness
**Database**: CockroachDB via sqlx

### chat-service/
Real-time WebSocket service handling:
- E2EE message routing (no permanent storage)
- Connection management
- Presence indicators
- Ephemeral offline message queuing via SQS
- Room management

**Deployment**: Lambda WebSocket (Phase 1) → EC2 Spot (Phase 2) → Direct ALB (Phase 3)
**Storage**: SQS for offline messages only (24h TTL)

### shared/
Common Rust code:
- Database models and migrations
- Auth0 JWT validation
- Error handling and logging
- Common utilities and types
- E2EE encryption helpers

## Development Workflow

### Local Development
```bash
# Start all services with hot reloading
./scripts/dev.sh

# Or start individual services
cargo lambda watch api-gateway --env-file api-gateway/lambda-env/dev.env
cargo lambda watch chat-service --env-file chat-service/lambda-env/dev.env
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific service tests
cargo test -p api-gateway
cargo test -p chat-service
```

### Deployment
```bash
# Build all services
cargo lambda build --release --workspace

# Deploy individual services
cargo lambda deploy api-gateway --config-file api-gateway/deploy-dev.toml
cargo lambda deploy chat-service --config-file chat-service/deploy-dev.toml

# Or use deployment script
./scripts/deploy.sh dev all
```

## Configuration Management

Each service has environment-specific configuration:
- `deploy-dev.toml` - Development environment
- `deploy-staging.toml` - Staging environment
- `deploy-prod.toml` - Production environment
- `lambda-env/` - Environment variables per deployment

## Key Features

### Stateless Design
All services are stateless to support:
- Horizontal scaling
- Lambda cold start optimization
- Easy migration between Lambda and EC2

### Progressive Scaling
- **Phase 1**: Pure Lambda deployment
- **Phase 2**: API Gateway on Lambda, Chat on EC2
- **Phase 3**: Direct ALB WebSocket handling

### Security
- JWT token validation for all requests
- Role-based access control
- Input validation and sanitization
- Rate limiting via API Gateway

## Database Integration

Uses CockroachDB (PostgreSQL-compatible):
- Async connections via sqlx
- Connection pooling
- Automatic retries and failover
- Geographic data support

## Message Queue Integration

For chat service offline message handling:
- **SQS**: Individual user message queues
- **SNS**: Message routing between instances
- **TTL**: 24-hour automatic message expiry