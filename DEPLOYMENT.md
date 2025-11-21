# 🚀 Community Manager - Deployment Guide

## Quick Start

Deploy the entire service (API Server + Authorizer) with one command:

```bash
./deploy.sh
```

The script will:
1. ✅ Let you select the environment stage (dev, staging, prod)
2. ✅ Check prerequisites (cargo-lambda, AWS CLI, Rust)
3. ✅ Let you select AWS account/profile
4. ✅ Build both services for ARM64
5. ✅ Verify/create IAM role
6. ✅ Deploy API Server to Lambda
7. ✅ Deploy Authorizer to Lambda
8. ✅ Show next steps for API Gateway configuration

---

## Environment Stages

The script supports three deployment stages:

### Development (`dev`)
- **Function Names**: 
  - `community-manager-api-dev`
  - `community-manager-authorizer-dev`
- **Use Case**: Testing, development, experimentation
- **No confirmation needed**

### Staging (`staging`)
- **Function Names**:
  - `community-manager-api-staging`
  - `community-manager-authorizer-staging`
- **Use Case**: Pre-production testing, UAT
- **No confirmation needed**

### Production (`prod`)
- **Function Names**:
  - `community-manager-api`
  - `community-manager-authorizer`
- **Use Case**: Live environment
- **⚠️ Requires confirmation** (type "yes" to proceed)

---

## What Gets Deployed

### API Server
- **Function Name**: `community-manager-api` (or with stage suffix)
- **Memory**: 512 MB
- **Timeout**: 60 seconds
- **Runtime**: Rust (custom runtime)
- **Architecture**: ARM64 (Graviton2)

### Authorizer
- **Function Name**: `community-manager-authorizer` (or with stage suffix)
- **Memory**: 256 MB
- **Timeout**: 30 seconds
- **Runtime**: Rust (custom runtime)
- **Architecture**: ARM64 (Graviton2)

---

## Prerequisites

### Required
- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **AWS CLI**: `brew install awscli` (macOS)
- **AWS Credentials**: `aws configure`

### Optional (auto-installed if missing)
- **cargo-lambda**: `pip3 install cargo-lambda`

---

## Usage

### Standard Deployment
```bash
./deploy.sh
```

### Skip Pre-flight Checks (faster)
```bash
./deploy.sh --skip-checks
```

### Skip Build (use existing binaries)
```bash
./deploy.sh --skip-build
```

### Skip Both
```bash
./deploy.sh --skip-checks --skip-build
```

### Show Help
```bash
./deploy.sh --help
```

---

## Environment Variables

Set these before deploying to configure the services:

```bash
# Database
export DATABASE_URL="postgresql://user:pass@host:5432/db"

# Auth0
export AUTH0_DOMAIN="your-tenant.auth0.com"
export AUTH0_CLIENT_ID="your-client-id"
export AUTH0_CLIENT_SECRET="your-client-secret"
export AUTH0_CALLBACK_URL="https://your-api.com/auth/callback"

# Session & JWT
export SESSION_SECRET="your-session-secret-min-32-chars"
export JWT_SECRET="your-jwt-secret"

# Then deploy
./deploy.sh
```

Or inline:
```bash
DATABASE_URL="..." AUTH0_DOMAIN="..." ./deploy.sh
```

---

## Configuration

Edit these variables in `deploy.sh` to customize:

```bash
# Server Configuration
SERVER_FUNCTION_NAME="community-manager-api"
SERVER_MEMORY=512
SERVER_TIMEOUT=60

# Authorizer Configuration
AUTHORIZER_FUNCTION_NAME="community-manager-authorizer"
AUTHORIZER_MEMORY=256
AUTHORIZER_TIMEOUT=30

# AWS Configuration
REGION="eu-central-1"
ROLE_NAME="lambda-execution-role"
```

---

## AWS Account Selection

The script lists all configured AWS profiles:

```
Available AWS profiles:
  1) default (Account: 123456789012)
  2) production (Account: 987654321098)
  3) staging (Account: 555555555555)

Select profile (1-3): 2
```

Select the account where you want to deploy.

---

## What Happens After Deployment

### 1. API Gateway Configuration

You need to manually configure API Gateway to use these Lambda functions:

```bash
# Create REST API
aws apigateway create-rest-api \
  --name community-manager-api \
  --description "Community Manager API"

# Get API ID
API_ID="abc123xyz"

# Create Lambda Authorizer
AUTHORIZER_ARN="arn:aws:lambda:region:account:function:community-manager-authorizer"

aws apigateway put-authorizer \
  --rest-api-id $API_ID \
  --name community-manager-authorizer \
  --type TOKEN \
  --authorizer-uri $AUTHORIZER_ARN \
  --identity-source method.request.header.Authorization \
  --authorizer-result-ttl-in-seconds 3600

# Create Lambda integration for API Server
SERVER_ARN="arn:aws:lambda:region:account:function:community-manager-api"

aws apigateway put-integration \
  --rest-api-id $API_ID \
  --resource-id resource-id \
  --http-method ANY \
  --type AWS_PROXY \
  --integration-http-method POST \
  --uri $SERVER_ARN
```

### 2. Test the Deployment

```bash
# Test API Server health
curl https://your-api.com/health

# Test with authorization
curl -H "Authorization: Bearer your-token" \
  https://your-api.com/api/communities
```

### 3. Monitor Logs

```bash
# API Server logs
aws logs tail /aws/lambda/community-manager-api --follow

# Authorizer logs
aws logs tail /aws/lambda/community-manager-authorizer --follow
```

### 4. Update Environment Variables (if needed)

```bash
# Update API Server
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --environment Variables={DATABASE_URL=...,AUTH0_DOMAIN=...}

# Update Authorizer
aws lambda update-function-configuration \
  --function-name community-manager-authorizer \
  --environment Variables={AUTH0_DOMAIN=...,JWT_SECRET=...}
```

---

## Troubleshooting

### "No AWS profiles found"
```bash
aws configure
# or
aws configure --profile production
```

### "cargo-lambda not found"
The script will offer to install it. Or manually:
```bash
pip3 install cargo-lambda
```

### "Role does not exist"
The script will offer to create it. Or manually:
```bash
aws iam create-role \
  --role-name lambda-execution-role \
  --assume-role-policy-document '{
    "Version": "2012-10-17",
    "Statement": [{
      "Effect": "Allow",
      "Principal": {"Service": "lambda.amazonaws.com"},
      "Action": "sts:AssumeRole"
    }]
  }'

aws iam attach-role-policy \
  --role-name lambda-execution-role \
  --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
```

### "Deployment failed"
Check the error message. Common issues:
- Role not ready yet (wait a few seconds and retry)
- Invalid AWS credentials
- Insufficient IAM permissions
- Build failed (check Rust compilation errors)

View logs:
```bash
aws logs tail /aws/lambda/community-manager-api --follow
aws logs tail /aws/lambda/community-manager-authorizer --follow
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    API Gateway                          │
│  (Handles HTTP requests, routes to Lambda)              │
└──────────────────────┬──────────────────────────────────┘
                       │
        ┌──────────────┴──────────────┐
        │                             │
        ▼                             ▼
┌──────────────────┐         ┌──────────────────────┐
│  Authorizer      │         │  API Server          │
│  (Lambda)        │         │  (Lambda)            │
│                  │         │                      │
│ - Validates      │         │ - Handles requests   │
│   tokens         │         │ - Queries database   │
│ - Injects user   │         │ - Returns responses  │
│   context        │         │                      │
│ - Caches for     │         │ - Uses context from  │
│   1 hour         │         │   authorizer         │
└──────────────────┘         └──────────────────────┘
        │                             │
        └──────────────┬──────────────┘
                       │
                       ▼
        ┌──────────────────────────┐
        │   CockroachDB Cloud      │
        │   (Database)             │
        └──────────────────────────┘
```

---

## Performance

### Deployment Time
- First deployment: ~5-10 minutes (includes build)
- Subsequent deployments: ~2-3 minutes (with --skip-build)

### Runtime Performance
- API Server: 512 MB memory, 60s timeout
- Authorizer: 256 MB memory, 30s timeout
- Caching: 1 hour (3600 seconds)

### Cost Optimization
- ARM64 architecture (Graviton2): ~20% cheaper than x86
- Caching: Reduces authorizer invocations by 90%
- Memory allocation: Optimized for typical workloads

---

## Advanced Usage

### Deploy to Multiple Accounts
```bash
# Deploy to staging
./deploy.sh  # Select staging profile

# Deploy to production
./deploy.sh  # Select production profile
```

### Automated Deployment (CI/CD)
```bash
# Set profile via environment
export AWS_PROFILE=production

# Skip interactive prompts
./deploy.sh --skip-checks

# Or with environment variables
DATABASE_URL="..." AUTH0_DOMAIN="..." ./deploy.sh --skip-checks
```

### Local Testing Before Deploy
```bash
# Build locally
cargo lambda build --release --arm64

# Test API Server locally
cargo lambda invoke community-manager-api --data-file test-event.json

# Test Authorizer locally
cargo lambda invoke community-manager-authorizer --data-file authorizer/test-event.json

# Then deploy
./deploy.sh --skip-build
```

---

## Monitoring & Maintenance

### View Logs
```bash
# Real-time logs
aws logs tail /aws/lambda/community-manager-api --follow
aws logs tail /aws/lambda/community-manager-authorizer --follow

# Search for errors
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "ERROR"
```

### Update Configuration
```bash
# Update memory
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --memory 1024

# Update timeout
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --timeout 120

# Update environment variables
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --environment Variables={KEY1=value1,KEY2=value2}
```

### Redeploy After Code Changes
```bash
# Make code changes
# Then:
./deploy.sh
```

---

## Support

For issues:
1. Check the troubleshooting section above
2. Review AWS CloudWatch logs
3. Check the [AUTHORIZER_DEPLOYMENT.md](./docs/AUTHORIZER_DEPLOYMENT.md) guide
4. Review the script comments for details

---

**Happy deploying!** 🚀
