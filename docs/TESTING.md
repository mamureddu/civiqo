# Community Manager - Comprehensive Test Suite

## Overview

This document describes the comprehensive test suite implemented for the Community Manager project, with special focus on validating the rustls transition from native-tls.

## Test Architecture

### Test Organization

```
backend/
├── shared/
│   ├── src/
│   │   ├── auth/mod.rs           # Auth tests with mock Auth0
│   │   ├── crypto/mod.rs         # Crypto operations tests
│   │   ├── database/mod.rs       # Database connection tests
│   │   ├── error.rs              # Error handling tests
│   │   ├── models/mod.rs         # Model serialization tests
│   │   ├── utils.rs              # Utility function tests
│   │   └── testing.rs            # Test helpers and utilities
├── api-gateway/
│   └── tests/
│       └── integration_tests.rs  # API endpoint integration tests
├── chat-service/
│   └── tests/                    # WebSocket and messaging tests
└── tests/
    └── rustls_validation.rs      # rustls-specific validation tests
```

## Test Categories

### 1. Unit Tests

#### Shared Library Tests
- **Location**: `backend/shared/src/*/mod.rs` and individual modules
- **Coverage**:
  - Authentication functions and JWT validation
  - Cryptographic operations (hashing, encryption, key generation)
  - Database connection management and pooling
  - Error handling and conversion
  - Utility functions (validation, formatting, calculations)
  - Model serialization/deserialization

#### API Gateway Tests
- **Location**: `backend/api-gateway/src/*/mod.rs`
- **Coverage**: Route handlers, middleware, validation

#### Chat Service Tests
- **Location**: `backend/chat-service/src/*/mod.rs`
- **Coverage**: WebSocket handling, message processing

### 2. Integration Tests

#### API Integration Tests
- **Location**: `backend/api-gateway/tests/integration_tests.rs`
- **Coverage**:
  - End-to-end API endpoint testing
  - Database operations through API
  - Authentication flow testing
  - Error response validation
  - Request/response serialization

#### Database Integration Tests
- **Location**: Multiple locations with `#[serial]` attribute
- **Coverage**:
  - Database connections with rustls
  - Migration execution
  - Complex queries and transactions
  - Connection pool behavior

### 3. rustls-Specific Validation Tests

#### rustls Transition Validation
- **Location**: `backend/tests/rustls_validation.rs`
- **Purpose**: Specifically validate the transition from native-tls to rustls
- **Coverage**:
  - Database connections using rustls
  - HTTP client connections using rustls
  - SSL/TLS configuration validation
  - Concurrent connection testing
  - Connection pool performance with rustls
  - Error handling with rustls
  - Compilation feature validation

## Test Infrastructure

### Test Helpers and Utilities

The `shared/src/testing.rs` module provides comprehensive test utilities:

```rust
// Database setup and cleanup
create_test_db() -> Result<Database>
cleanup_test_db(db: &Database) -> Result<()>

// Test data creation
create_test_user(db: &Database, auth0_id: Option<String>) -> Result<User>
create_test_community(db: &Database, creator_id: Uuid, name: Option<String>) -> Result<Community>
create_test_role(db: &Database, name: String, permissions: Vec<String>) -> Result<Role>

// Mock objects
create_mock_jwt_claims(user_id: Uuid, auth0_id: String, email: String) -> Claims

// rustls-specific testing
create_test_tls_config() -> Result<()>
test_rustls_db_connection() -> Result<()>
test_rustls_http_client() -> Result<()>
```

### Test Configuration

#### Environment Variables
```bash
# Test database (using rustls)
TEST_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/community_manager_test

# Auth0 test configuration
AUTH0_DOMAIN=test.auth0.com
AUTH0_AUDIENCE=test-audience
AUTH0_CLIENT_ID=test-client-id
AUTH0_CLIENT_SECRET=test-client-secret

# Connection pool settings
DB_MAX_CONNECTIONS=5
DB_MIN_CONNECTIONS=1
DB_ACQUIRE_TIMEOUT_SECONDS=5

# Logging
RUST_LOG=debug
RUST_BACKTRACE=1
```

#### Test Database Setup
```sql
-- Create test database
CREATE DATABASE community_manager_test;

-- The test suite automatically runs migrations
-- No manual schema setup required
```

## Running Tests

### Quick Test Run
```bash
# Run all tests
cd backend
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture
```

### Comprehensive Test Suite
```bash
# Run the complete test suite
./test-runner.sh
```

### Specific Test Categories

#### Unit Tests Only
```bash
cd backend
cargo test --workspace --lib
```

#### Integration Tests Only
```bash
cd backend
cargo test --workspace --test '*'
```

#### rustls Validation Tests
```bash
cd backend
cargo test --test rustls_validation
```

#### Database Tests (requires test database)
```bash
cd backend
TEST_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/community_manager_test \
cargo test -- database
```

#### Performance Tests
```bash
cd backend
cargo test --release -- concurrent
```

## rustls Transition Validation

### What We're Testing

1. **Database Connections**: Verify SQLx with `runtime-tokio-rustls` feature works correctly
2. **HTTP Clients**: Verify reqwest with `rustls-tls` feature works correctly
3. **SSL/TLS Configuration**: Ensure proper TLS configuration and certificate validation
4. **Connection Pooling**: Verify connection pool performance with rustls
5. **Error Handling**: Ensure rustls errors are properly handled and reported
6. **Concurrent Operations**: Test rustls performance under concurrent load

### Key Test Cases

#### Database Connection Validation
```rust
#[tokio::test]
async fn test_rustls_database_connection() {
    let db = Database::connect(&test_database_url).await?;

    // Test basic query
    let result: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&db.pool)
        .await?;

    assert_eq!(result.0, 1);
}
```

#### HTTP Client Validation
```rust
#[tokio::test]
async fn test_rustls_http_client() {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;

    assert!(response.status().is_success());
}
```

#### Concurrent Connection Testing
```rust
#[tokio::test]
async fn test_concurrent_rustls_connections() {
    let futures = (0..10).map(|i| {
        tokio::spawn(async move {
            let db = Database::connect(&url).await?;
            // Perform operations...
        })
    });

    let results = futures::future::join_all(futures).await;
    // Validate all connections succeeded
}
```

## Test Dependencies

### Core Testing Libraries
- `tokio-test` - Async testing utilities
- `serial_test` - Sequential test execution for database tests
- `wiremock` - HTTP mocking for Auth0 API simulation
- `rstest` - Parameterized testing
- `test-case` - Test case generation
- `tempfile` - Temporary file/directory management
- `insta` - Snapshot testing for serialization validation

### Validation Tools
- `axum-test` - HTTP server testing for API endpoints
- `mockall` - Mock object generation

## CI/CD Integration

### GitHub Actions Workflow

The test suite is integrated with GitHub Actions (`/.github/workflows/test.yml`):

1. **Basic Tests**: Compilation, formatting, linting
2. **Unit Tests**: All unit tests across workspace
3. **Integration Tests**: API endpoints and database operations
4. **rustls Validation**: Specific rustls transition validation
5. **Performance Tests**: Connection pooling and concurrent operations
6. **Security Audit**: Dependency vulnerability scanning
7. **Code Coverage**: Coverage reporting with codecov

### Test Stages

1. **Compilation Check**: Verify all projects compile with rustls features
2. **Unit Testing**: Run all unit tests
3. **rustls Validation**: Specific rustls functionality tests
4. **Integration Testing**: End-to-end API testing
5. **Performance Testing**: Connection pool and concurrent operation tests
6. **Security Validation**: Dependency and configuration security checks

## Performance Considerations

### Database Connection Pool Testing

Tests validate that rustls performs well with connection pooling:

- Connection establishment time
- Query execution performance
- Pool exhaustion handling
- Connection reuse efficiency
- Memory usage patterns

### Concurrent Operation Testing

Tests ensure rustls handles concurrent operations efficiently:

- Multiple simultaneous database connections
- Concurrent HTTP requests
- SSL handshake performance
- Connection timeout behavior

## Security Testing

### TLS Configuration Validation

Tests verify secure TLS configuration:

- Certificate validation
- Cipher suite selection
- Protocol version enforcement
- Certificate chain validation

### Authentication Testing

Tests validate Auth0 integration security:

- JWT token validation
- JWKS endpoint security
- Token expiration handling
- Malicious token rejection

## Test Data Management

### Database Test Data

- Tests use isolated test database
- Automatic cleanup after each test
- Deterministic test data generation
- Transaction rollback for test isolation

### Mock Data Generation

- Consistent test user generation
- Realistic community and business data
- Geographic coordinate validation
- Time-based data handling

## Troubleshooting Tests

### Common Issues

#### Database Connection Failures
```bash
# Check if PostgreSQL is running
pg_isready -h localhost -p 5432

# Verify test database exists
psql -h localhost -p 5432 -U postgres -l | grep community_manager_test
```

#### Environment Variable Issues
```bash
# Load test environment
cd backend
source .env.test

# Verify variables are set
env | grep TEST_DATABASE_URL
```

#### rustls Compilation Issues
```bash
# Clean build cache
cargo clean

# Rebuild with verbose output
cargo build --workspace -v
```

### Test Debugging

#### Enable Detailed Logging
```bash
RUST_LOG=debug cargo test -- --nocapture
```

#### Run Specific Failing Tests
```bash
cargo test test_name_here -- --nocapture
```

#### Database Query Debugging
```bash
# Enable SQLx query logging
RUST_LOG=sqlx=debug cargo test -- --nocapture
```

## Metrics and Coverage

### Coverage Goals

- **Unit Tests**: >90% line coverage
- **Integration Tests**: All major API endpoints covered
- **rustls Validation**: 100% of rustls functionality validated
- **Error Handling**: All error paths tested

### Performance Benchmarks

- Database connection establishment: <500ms
- API response time: <200ms (95th percentile)
- Concurrent connections: Support 100+ simultaneous connections
- Memory usage: <50MB baseline, <200MB under load

## Maintenance

### Adding New Tests

1. **Unit Tests**: Add to appropriate module with `#[cfg(test)]`
2. **Integration Tests**: Add to `tests/` directory
3. **Update Documentation**: Update this file with new test descriptions
4. **CI/CD**: Update GitHub Actions if needed

### Test Data Updates

1. Update migration files in `migrations/`
2. Update test helper functions in `testing.rs`
3. Verify all existing tests still pass

### Dependency Updates

1. Update `Cargo.toml` files
2. Run full test suite
3. Update documentation if API changes
4. Verify rustls features still work correctly

---

This comprehensive test suite ensures the reliability, security, and performance of the Community Manager application, with particular attention to validating the successful transition from native-tls to rustls.