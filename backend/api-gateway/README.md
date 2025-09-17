# API Gateway Service

REST API service handling all non-realtime operations for the Community Manager platform.

## Overview

The API Gateway service provides:
- User management and profiles (Auth0 sync)
- Community CRUD operations
- Business profile management
- Governance tools (polls, voting)
- Role-based authorization
- File upload coordination

## Architecture

- **Deployment**: AWS Lambda with API Gateway
- **Database**: CockroachDB via sqlx
- **Authentication**: Auth0 JWT validation
- **File Storage**: AWS S3 for uploads
- **Scaling**: Serverless auto-scaling

## Cargo Lambda Configuration

### Development
```bash
cargo lambda deploy --config-file deploy-dev.toml
```

### Staging
```bash
cargo lambda deploy --config-file deploy-staging.toml
```

### Production
```bash
cargo lambda deploy --config-file deploy-prod.toml
```

## Environment Configuration

### Development (dev.env)
- Local PostgreSQL database
- Debug logging enabled
- CORS allowing localhost:3000
- Development Auth0 tenant

### Staging (staging.env)
- CockroachDB staging cluster
- Info level logging
- Staging domain CORS
- Staging Auth0 tenant

### Production (prod.env)
- CockroachDB production cluster
- Warning level logging
- Production domain CORS only
- Production Auth0 tenant
- Performance optimizations

## API Endpoints (Planned)

### Authentication
- `POST /auth/sync` - Sync user from Auth0
- `GET /auth/me` - Get current user profile
- `PUT /auth/profile` - Update user profile

### Communities
- `GET /communities` - List/search communities
- `POST /communities` - Create new community
- `GET /communities/{id}` - Get community details
- `PUT /communities/{id}` - Update community
- `POST /communities/{id}/join` - Join community
- `GET /communities/{id}/members` - List members
- `PUT /communities/{id}/members/{user_id}` - Update member role

### Businesses
- `GET /communities/{id}/businesses` - List businesses in community
- `POST /communities/{id}/businesses` - Create business
- `GET /businesses/{id}` - Get business details
- `PUT /businesses/{id}` - Update business
- `POST /businesses/{id}/products` - Add product
- `GET /businesses/{id}/products` - List products

### Governance
- `GET /communities/{id}/polls` - List active polls
- `POST /communities/{id}/polls` - Create poll
- `POST /polls/{id}/vote` - Cast vote
- `GET /polls/{id}/results` - Get poll results
- `GET /communities/{id}/decisions` - List decisions
- `POST /communities/{id}/decisions` - Create decision

### File Uploads
- `POST /upload/avatar` - Upload user avatar
- `POST /upload/business-image` - Upload business image
- `GET /upload/presigned-url` - Get presigned S3 URL

## Security

- JWT token validation on all protected routes
- Role-based access control per community
- Input validation and sanitization
- Rate limiting via API Gateway
- CORS configuration per environment

## Performance

- Connection pooling with sqlx
- Async/await throughout
- Lambda cold start optimizations
- Response caching where appropriate

## Monitoring

- CloudWatch logs and metrics
- Custom application metrics
- Error tracking and alerting
- Performance monitoring

## Local Development

```bash
# Start with hot reloading
cargo lambda watch --env-file lambda-env/dev.env --port 9001

# Test endpoints
curl http://localhost:9001/health
```

## Deployment

```bash
# Build
cargo lambda build --release

# Deploy to development
cargo lambda deploy --config-file deploy-dev.toml

# Deploy via script
../../../scripts/deploy.sh dev api
```