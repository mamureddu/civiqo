# Disabled Tests

These tests have been temporarily disabled because they require:
1. Full database setup with migrations
2. Running CockroachDB instance
3. Complex test fixtures

## Tests Moved Here

- `auth_middleware_tests.rs` - Auth middleware integration tests (requires DB)
- `business_api_tests.rs` - Business API tests (requires DB)
- `comprehensive_api_tests.rs` - Comprehensive API tests (requires DB)
- `governance_api_tests.rs` - Governance API tests (requires DB)
- `integration_tests.rs` - Full integration tests (requires DB)
- `performance_tests.rs` - Performance tests (requires DB)
- `validation_security_tests.rs` - Validation and security tests (requires DB)

## To Re-enable

1. Setup CockroachDB Cloud connection
2. Run migrations: `sqlx migrate run`
3. Move tests back to `tests/` directory
4. Update references from `api_gateway` to `server` (already done)

## Currently Active Tests

- `pages_test.rs` - Page rendering tests (no DB required) ✅
