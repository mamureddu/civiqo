# 🚀 Deployment Guide

Guida completa per il deployment del Community Manager in ambienti di staging e produzione su AWS Lambda.

## 📋 Indice

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Environment Configuration](#environment-configuration)
4. [Deployment Process](#deployment-process)
5. [Post-Deployment](#post-deployment)
6. [Monitoring & Troubleshooting](#monitoring--troubleshooting)
7. [Rollback Procedures](#rollback-procedures)

---

## Overview

Il Community Manager utilizza un'architettura serverless su AWS Lambda:

### Services Deployed
- **API Server** (`community-manager-api`) - Main application server
- **Authorizer** (`community-manager-authorizer`) - JWT validation service

### Deployment Strategy
- **Progressive scaling**: Lambda → EC2 → ALB + WebSocket
- **Environment-specific**: dev, staging, production configs
- **Zero-downtime**: Automated deployment with health checks

---

## Prerequisites

### Required Tools
```bash
# Install AWS CLI v2
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# Install cargo-lambda
cargo install cargo-lambda

# Configure AWS credentials
aws configure
```

### AWS Permissions
Required IAM permissions:
- `lambda:*` - Create and manage functions
- `iam:*` - Create execution roles
- `cloudwatch:*` - View logs and metrics
- `apigateway:*` - Configure API Gateway (future)

### Environment Files
Create environment files in project root:
```bash
# .env.dev
.env.staging
.env.production
```

---

## Environment Configuration

### Environment Variables

| Variable | Required | Description | Example |
|----------|----------|-------------|---------|
| `DATABASE_URL` | ✅ | CockroachDB connection string | `postgresql://...` |
| `AUTH0_DOMAIN` | ✅ | Auth0 tenant domain | `tenant.auth0.com` |
| `AUTH0_CLIENT_ID` | ✅ | Auth0 application client ID | `abc123...` |
| `AUTH0_CLIENT_SECRET` | ✅ | Auth0 application secret | `xyz789...` |
| `AUTH0_CALLBACK_URL` | ✅ | OAuth2 callback URL | `https://app.domain.com/auth/callback` |
| `SESSION_SECRET` | ✅ | Session encryption key | `random-32-chars...` |
| `JWT_SECRET` | ✅ | JWT signing secret | `jwt-secret-key...` |
| `RUST_LOG` | ✅ | Log level | `info`/`debug`/`trace` |

### Example .env.production
```bash
# Database
DATABASE_URL=postgresql://community-manager:***@prod-cluster.cockroachlabs.cloud:26257/community-manager?sslmode=verify-full

# Auth0
AUTH0_DOMAIN=civiqo-prod.eu.auth0.com
AUTH0_CLIENT_ID=prod-client-id
AUTH0_CLIENT_SECRET=prod-client-secret
AUTH0_CALLBACK_URL=https://app.civiqo.com/auth/callback

# Secrets
SESSION_SECRET=super-secure-session-secret-key-32-chars-min
JWT_SECRET=super-secure-jwt-signing-secret-key

# Logging
RUST_LOG=info
```

---

## Deployment Process

### 1. Automated Deployment Script

```bash
# Interactive deployment
./deploy.sh

# Skip pre-flight checks
./deploy.sh --skip-checks

# Skip build (use existing binaries)
./deploy.sh --skip-build

# Show help
./deploy.sh --help
```

### 2. Manual Deployment Steps

#### Step 1: Build Services
```bash
# Build for AWS Lambda (ARM64)
cargo build --workspace --release --target aarch64-unknown-linux-musl

# Or use cargo-lambda
cargo lambda build --release --arm64
```

#### Step 2: Deploy Authorizer
```bash
cd authorizer
cargo lambda deploy authorizer \
  --iam-role arn:aws:iam::ACCOUNT:role/lambda-execution-role \
  --env-var DATABASE_URL="$DATABASE_URL" \
  --env-var JWT_SECRET="$JWT_SECRET" \
  --env-var RUST_LOG=info \
  --memory 256 \
  --timeout 30
```

#### Step 3: Deploy API Server
```bash
cd src
cargo lambda deploy community-manager-api \
  --iam-role arn:aws:iam::ACCOUNT:role/lambda-execution-role \
  --env-var DATABASE_URL="$DATABASE_URL" \
  --env-var AUTH0_DOMAIN="$AUTH0_DOMAIN" \
  --env-var AUTH0_CLIENT_ID="$AUTH0_CLIENT_ID" \
  --env-var AUTH0_CLIENT_SECRET="$AUTH0_CLIENT_SECRET" \
  --env-var AUTH0_CALLBACK_URL="$AUTH0_CALLBACK_URL" \
  --env-var SESSION_SECRET="$SESSION_SECRET" \
  --env-var RUST_LOG=info \
  --memory 512 \
  --timeout 60
```

### 3. Environment-Specific Deployment

#### Development
```bash
# Deploy to dev environment
./deploy.sh
# Select option 1: Development
```

#### Staging
```bash
# Deploy to staging environment
./deploy.sh
# Select option 2: Staging
```

#### Production
```bash
# Deploy to production environment
./deploy.sh
# Select option 3: Production
# Requires explicit confirmation
```

---

## Post-Deployment

### 1. Verify Deployment

#### Check Lambda Functions
```bash
# List functions
aws lambda list-functions --query 'Functions[?contains(FunctionName, `community-manager`)]'

# Get function configuration
aws lambda get-function-configuration --function-name community-manager-api

# Get authorizer configuration
aws lambda get-function-configuration --function-name community-manager-authorizer
```

#### Test Functions
```bash
# Test API server
aws lambda invoke \
  --function-name community-manager-api \
  --payload '{"httpMethod":"GET","path":"/health"}' \
  response.json

# Test authorizer
aws lambda invoke \
  --function-name community-manager-authorizer \
  --payload file://test-event.json \
  auth-response.json
```

### 2. Update DNS and Configuration

#### API Gateway (Future)
```bash
# When API Gateway is implemented
aws apigateway update-stage \
  --rest-api-id API_ID \
  --stage-name prod \
  --patch-operations op=replace,path=/~1deploymentId,value=DEPLOYMENT_ID
```

#### Custom Domain
```bash
# Update Route 53 records
aws route53 change-resource-record-sets \
  --hosted-zone-id ZONE_ID \
  --change-batch file://dns-config.json
```

### 3. Health Checks

```bash
# Monitor function metrics
aws cloudwatch get-metric-statistics \
  --namespace AWS/Lambda \
  --metric-name Invocations \
  --dimensions Name=FunctionName,Value=community-manager-api \
  --start-time 2025-01-01T00:00:00Z \
  --end-time 2025-01-01T01:00:00Z \
  --period 60 \
  --statistics Sum
```

---

## Monitoring & Troubleshooting

### 1. CloudWatch Logs

#### View Logs
```bash
# API Server logs
aws logs tail /aws/lambda/community-manager-api --follow

# Authorizer logs
aws logs tail /aws/lambda/community-manager-authorizer --follow

# Specific time range
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --start-time 1704067200000 \
  --end-time 1704153600000
```

#### Common Log Patterns
```bash
# Search for errors
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "ERROR"

# Search for Auth0 issues
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "Auth0"

# Search for database issues
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "DATABASE"
```

### 2. Performance Monitoring

#### Key Metrics
- **Invocations**: Number of function calls
- **Duration**: Average execution time
- **Errors**: Error rate and types
- **Throttles**: Rate limiting events
- **Iterator Age**: For async functions (future)

#### Monitoring Commands
```bash
# Get function metrics
aws cloudwatch get-metric-statistics \
  --namespace AWS/Lambda \
  --metric-name Duration \
  --dimensions Name=FunctionName,Value=community-manager-api \
  --start-time $(date -d '1 hour ago' +%s)000 \
  --end-time $(date +%s)000 \
  --period 300 \
  --statistics Average

# Check error rates
aws cloudwatch get-metric-statistics \
  --namespace AWS/Lambda \
  --metric-name Errors \
  --dimensions Name=FunctionName,Value=community-manager-api \
  --start-time $(date -d '1 hour ago' +%s)000 \
  --end-time $(date +%s)000 \
  --period 300 \
  --statistics Sum
```

### 3. Common Issues

#### Cold Start Issues
```bash
# Check provisioned concurrency
aws lambda get-function-configuration \
  --function-name community-manager-api \
  --query 'ProvisionedConcurrencyConfigurations'

# Enable provisioned concurrency if needed
aws lambda put-provisioned-concurrency-config \
  --function-name community-manager-api \
  --qualifier $LATEST \
  --provisioned-concurrent-executions 5
```

#### Memory Issues
```bash
# Check memory usage
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "memory"

# Increase memory if needed
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --memory-size 1024
```

#### Database Connection Issues
```bash
# Check database connectivity
aws logs filter-log-events \
  --log-group-name /aws/lambda/community-manager-api \
  --filter-pattern "DATABASE_URL|connection"

# Verify environment variables
aws lambda get-function-configuration \
  --function-name community-manager-api \
  --query 'Environment.Variables'
```

---

## Rollback Procedures

### 1. Quick Rollback

#### To Previous Version
```bash
# List previous versions
aws lambda list-versions-by-function \
  --function-name community-manager-api

# Rollback to specific version
aws lambda publish-version \
  --function-name community-manager-api \
  --description "Rollback to stable version"

aws lambda update-alias \
  --function-name community-manager-api \
  --name prod \
  --function-version $VERSION_NUMBER
```

#### Emergency Rollback
```bash
# Disable function (emergency)
aws lambda put-function-concurrency \
  --function-name community-manager-api \
  --reserved-concurrent-executions 0

# Re-enable after fix
aws lambda delete-function-concurrency \
  --function-name community-manager-api
```

### 2. Blue-Green Deployment

#### Setup Blue-Green
```bash
# Deploy to green environment
./deploy.sh --environment green

# Test green environment
curl https://green-api.civiqo.com/health

# Switch traffic
aws lambda update-alias \
  --function-name community-manager-api \
  --name prod \
  --function-version $GREEN_VERSION
```

---

## Security Considerations

### 1. Environment Variables

#### Secure Storage
```bash
# Use AWS Secrets Manager for sensitive data
aws secretsmanager create-secret \
  --name community-manager/prod/database \
  --secret-string file://database-secret.json

# Reference in Lambda
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --environment Variables={DATABASE_URL=arn:aws:secretsmanager:region:account:secret:community-manager/prod/database}
```

### 2. Network Security

#### VPC Configuration
```bash
# Configure VPC (if needed)
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --vpc-config SubnetIds=subnet-123,subnet-456,SecurityGroupIds=sg-789
```

### 3. IAM Roles

#### Least Privilege
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "rds-db:connect"
      ],
      "Resource": "arn:aws:rds-db:region:account:dbuser:cluster/username"
    }
  ]
}
```

---

## Performance Optimization

### 1. Lambda Configuration

#### Memory and Timeout
```bash
# Optimize based on usage patterns
aws lambda update-function-configuration \
  --function-name community-manager-api \
  --memory-size 768 \
  --timeout 90
```

#### Concurrent Executions
```bash
# Set reserved concurrency
aws lambda put-function-concurrency \
  --function-name community-manager-api \
  --reserved-concurrent-executions 100
```

### 2. Database Optimization

#### Connection Pooling
```rust
// In application code
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(20)  // Lambda-friendly
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await?;
```

---

## Cost Management

### 1. Monitoring Costs

```bash
# Check Lambda costs
aws ce get-cost-and-usage \
  --time-period Start=2025-01-01,End=2025-01-31 \
  --filter file://lambda-cost-filter.json \
  --granularity MONTHLY \
  --metrics BlendedCost

# Monitor with budgets
aws budgets create-budget \
  --account-id ACCOUNT_ID \
  --budget file://lambda-budget.json
```

### 2. Optimization Strategies

- **Right-sizing memory**: Find optimal memory/CPU ratio
- **Provisioned concurrency**: Reduce cold starts for critical paths
- **Lambda layers**: Reduce deployment package size
- **Event-driven architecture**: Minimize always-on resources

---

## Automation

### 1. CI/CD Pipeline

#### GitHub Actions Example
```yaml
name: Deploy to Production
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: aarch64-unknown-linux-musl
      - name: Deploy Lambda
        run: ./deploy.sh --environment production
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

### 2. Automated Testing

#### Pre-deployment Tests
```bash
#!/bin/bash
# pre-deploy-checks.sh

echo "Running pre-deployment checks..."

# Build test
cargo build --workspace --release
if [ $? -ne 0 ]; then
  echo "Build failed - aborting deployment"
  exit 1
fi

# Unit tests
cargo test --workspace
if [ $? -ne 0 ]; then
  echo "Tests failed - aborting deployment"
  exit 1
fi

# Environment validation
./scripts/check-env.sh
if [ $? -ne 0 ]; then
  echo "Environment validation failed - aborting deployment"
  exit 1
fi

echo "All checks passed - proceeding with deployment"
```

---

## Troubleshooting Quick Reference

### Common Error Codes

| Error | Cause | Solution |
|-------|-------|----------|
| `502 Bad Gateway` | Lambda timeout | Increase timeout or optimize code |
| `503 Service Unavailable` | Lambda throttling | Increase concurrent executions |
| `403 Forbidden` | IAM permissions | Check execution role permissions |
| `500 Internal Server` | Application error | Check CloudWatch logs |

### Debug Commands

```bash
# Quick health check
./scripts/health-check.sh

# Full deployment status
./scripts/deployment-status.sh

# Rollback to last known good
./scripts/emergency-rollback.sh
```

---

**Last Updated**: November 25, 2025  
**Version**: 1.0  
**Next Review**: After next production deployment  

For additional support, check the [DEVELOPMENT.md](./DEVELOPMENT.md) guide or consult the CloudWatch logs.
