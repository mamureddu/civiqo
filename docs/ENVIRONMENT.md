# Environment Configuration Template

## Development Environment (.env)

Copy this content to `.env` in the root directory and `backend/.env`:

```bash
# =============================================================================
# DATABASE CONFIGURATION - CockroachDB Cloud
# =============================================================================
# Get this from your CockroachDB Cloud console
# Format: postgresql://username:password@host:port/database?sslmode=verify-full
DATABASE_URL=postgresql://your-user:your-password@your-cluster.cockroachlabs.cloud:26257/community_manager?sslmode=verify-full

# Connection Pool Settings
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=5
DB_ACQUIRE_TIMEOUT_SECONDS=8

# =============================================================================
# AUTH0 CONFIGURATION
# =============================================================================
AUTH0_DOMAIN=your-domain.auth0.com
AUTH0_AUDIENCE=your-audience
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret

# =============================================================================
# AWS CONFIGURATION (for production/staging)
# =============================================================================
# For local development, these can be mock values
# For production, use real AWS credentials
AWS_REGION=eu-central-1
AWS_ACCESS_KEY_ID=your-aws-access-key
AWS_SECRET_ACCESS_KEY=your-aws-secret-key

# S3 Configuration
S3_BUCKET=community-manager-uploads
S3_REGION=eu-central-1

# SQS Configuration (for chat service)
SQS_QUEUE_URL=https://sqs.eu-central-1.amazonaws.com/your-account/chat-queue

# =============================================================================
# APPLICATION CONFIGURATION
# =============================================================================
RUST_LOG=info
ENVIRONMENT=development

# API Configuration
API_PORT=9001
API_HOST=0.0.0.0

# WebSocket Configuration
WEBSOCKET_PORT=9002
WEBSOCKET_HOST=0.0.0.0
```

## Frontend Environment (.env.local)

Copy this content to `frontend/.env.local`:

```bash
# =============================================================================
# NEXTAUTH CONFIGURATION
# =============================================================================
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your-nextauth-secret-min-32-chars

# =============================================================================
# AUTH0 CONFIGURATION
# =============================================================================
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_DOMAIN=your-domain.auth0.com
AUTH0_ISSUER=https://your-domain.auth0.com

# =============================================================================
# BACKEND API CONFIGURATION
# =============================================================================
NEXT_PUBLIC_API_URL=http://localhost:9001
NEXT_PUBLIC_WS_URL=ws://localhost:9002

# =============================================================================
# APPLICATION CONFIGURATION
# =============================================================================
NODE_ENV=development
```

## Test Environment (backend/.env.test)

Copy this content to `backend/.env.test`:

```bash
# Test Database - Use a separate database for tests
DATABASE_URL=postgresql://your-user:your-password@your-cluster.cockroachlabs.cloud:26257/community_manager_test?sslmode=verify-full

# Test Auth0 (can use same as dev)
AUTH0_DOMAIN=your-domain.auth0.com
AUTH0_AUDIENCE=your-audience
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret

# Test Configuration
RUST_LOG=debug
ENVIRONMENT=test
```

## Notes

### Simplified Configuration:
All database credentials are included in the DATABASE_URL connection string. No separate variables needed for database host, port, user, or password.

### CockroachDB Connection String Format:
```
postgresql://[user]:[password]@[host]:[port]/[database]?sslmode=verify-full
```

### How to get your CockroachDB connection string:
1. Go to CockroachDB Cloud console (https://cockroachlabs.cloud/)
2. Select your cluster
3. Click "Connect"
4. Copy the connection string
5. Replace `[database]` with `community_manager` (or `community_manager_test` for tests)

### Generate NEXTAUTH_SECRET:
```bash
openssl rand -base64 32
```

### AWS Configuration:
- For local development: Use mock values or skip AWS features
- For staging/production: Use real AWS credentials
- S3, SQS, SNS will be configured when needed

### Port Configuration:
- **9001**: API Gateway (cargo lambda watch)
- **9002**: Chat WebSocket Service (cargo lambda watch)
- **3000**: Next.js Frontend (npm run dev)
