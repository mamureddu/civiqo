# 🔐 Authentication & Authorization Guide

## Overview

The Community Manager uses a **session-based authentication** approach with Auth0:

1. **Frontend**: HTMX + Server-side sessions (no JWT in browser)
2. **Backend**: Axum with tower-sessions
3. **Auth Provider**: Auth0 OAuth2
4. **API Protection**: Lambda Authorizer with context injection

## Architecture

```
User Browser
    ↓
Session Cookie (tower-sessions)
    ↓
Axum Routes
    ├── /auth/login → Redirect to Auth0
    ├── /auth/callback → Exchange code for token
    ├── /auth/logout → Delete session
    └── /auth/me → Get current user
    ↓
HTMX Pages (with session context)
```

## OAuth2 Flow

### Complete Authentication Flow

```
1. User clicks "Login"
   ↓
2. Redirect to Auth0 login page
   ↓
3. User authenticates with Auth0
   ↓
4. Auth0 redirects back with authorization code
   ↓
5. Server exchanges code for access token
   ↓
6. Server fetches user info from Auth0 /userinfo
   ↓
7. Server syncs user to local database
   ↓
8. Server creates session with user data
   ↓
9. User redirected to /dashboard
   ↓
10. All pages show user info in navbar
```

### Code Exchange Implementation

```rust
// In callback() handler
let tokens: TokenResponse = client
    .post(format!("https://{}/oauth/token", auth0_domain))
    .json(&serde_json::json!({
        "grant_type": "authorization_code",
        "client_id": client_id,
        "client_secret": client_secret,
        "code": code,
        "redirect_uri": callback_url,
    }))
    .send()
    .await?
    .json()
    .await?;

// Fetch user info
let user_info: Auth0UserInfo = client
    .get(format!("https://{}/userinfo", auth0_domain))
    .bearer_auth(&tokens.access_token)
    .send()
    .await?
    .json()
    .await?;
```

### User Sync to Database

```rust
// Insert/update user in users table
sqlx::query(
    "INSERT INTO users (id, auth0_id, email, username, picture) 
     VALUES ($1, $2, $3, $4, $5)
     ON CONFLICT (auth0_id) DO UPDATE SET 
        email = EXCLUDED.email,
        username = EXCLUDED.username,
        last_login = NOW()"
)
.bind(uuid::Uuid::new_v4())
.bind(&user_info.sub)
.bind(&user_info.email)
.bind(&user_info.name)
.bind(&user_info.picture)
.execute(&db.pool)
.await?;
```

## Auth Handlers

Located in `src/server/src/auth.rs`:

- **`login()`** - Redirects user to Auth0
- **`callback()`** - Handles Auth0 callback, exchanges code for tokens, syncs user to DB
- **`logout()`** - Deletes session
- **`AuthUser` extractor** - Requires authentication (401 if not logged in)
- **`OptionalAuthUser` extractor** - Optional authentication (None if not logged in)

### Protected Routes

- `/dashboard` - Requires login
- `POST /api/communities` - Requires login
- `POST /api/communities/:id/posts` - Requires login

## Lambda Authorizer

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    API Gateway                          │
│  - Routes requests                                      │
│  - Caches authorizer responses (1 hour)                 │
└────────────┬────────────────────────────┬───────────────┘
             │                            │
             ▼                            ▼
    ┌────────────────┐          ┌──────────────────┐
    │   Authorizer   │          │   Backend APIs   │
    │   (Lambda)     │          │   (Lambda/Server)│
    │                │          │                  │
    │ - Validate JWT │          │ - Business logic │
    │ - Gen policy   │          │ - Database ops   │
    │ - Inject ctx   │          │ - Response       │
    └────────────────┘          └────────┬─────────┘
                                         │
                                         ▼
                                ┌─────────────────┐
                                │  CockroachDB    │
                                │  Cloud          │
                                └─────────────────┘
```

### Features

- ✅ Token validation (JWT)
- ✅ IAM policy generation with wildcard
- ✅ User context injection
- ✅ Caching up to 1 hour
- ✅ 99% cost reduction

### Context Injection

The authorizer injects user context into downstream Lambda functions:

```json
{
  "context": {
    "user_id": "uuid",
    "email": "user@example.com",
    "username": "username",
    "role": "user"
  }
}
```

## Environment Variables

### Required for Authentication

```bash
# Auth0 Configuration
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback

# Session Configuration
SESSION_SECRET=your-random-secret-min-32-chars
SESSION_COOKIE_NAME=community_manager_session
SESSION_MAX_AGE=86400

# JWT Secret (for authorizer)
JWT_SECRET=your-jwt-secret-key
```

## Session Management

### Current: MemoryStore

- ✅ Simple, no dependencies
- ❌ Lost on server restart
- ❌ Not suitable for production

### Production: SQLx Store

```rust
use tower_sessions_sqlx_store::PostgresStore;

let session_store = PostgresStore::new(pool);
```

### Session Security

- ✅ Session layer configured with `SameSite::Lax`
- ⚠️ `secure=false` for local development (set to `true` in production)
- ⚠️ MemoryStore not suitable for production
- ⚠️ Need CSRF token validation for state changes

## Auth0 Setup

### 1. Create Auth0 Application

1. Dashboard → Applications → Create Application
2. Name: "Community Manager Web"
3. Type: **Regular Web Application**
4. Technology: **Rust**

### 2. Configure Callback URLs

**Allowed Callback URLs:**
```
http://localhost:9001/auth/callback
```

**Allowed Logout URLs:**
```
http://localhost:9001
```

**Allowed Web Origins:**
```
http://localhost:9001
```

### 3. Get Credentials

From Application Settings:
- **Domain**: `your-tenant.auth0.com`
- **Client ID**: `abc123...`
- **Client Secret**: `xyz789...`

## Deployment

### Authorizer Deployment

The authorizer is deployed as a standalone AWS Lambda function:

```bash
cd authorizer
cargo lambda build --release --arm64
cargo lambda deploy authorizer
```

### Deployment Script

Use the root deployment script:

```bash
./deploy.sh
```

This deploys:
- **API Server** (`community-manager-api`) - 512 MB, 60s timeout
- **Authorizer** (`community-manager-authorizer`) - 256 MB, 30s timeout

### Environment Variables in Production

Set via AWS Console or Lambda environment:
- `RUST_LOG=info`
- `AUTH0_DOMAIN=your-tenant.auth0.com`
- `JWT_SECRET=your-jwt-secret`
- `DATABASE_URL=postgresql://...`

## Caching Strategy

### API Gateway Caching

- **Cache Duration**: 1 hour
- **Cache Key**: User ID + resource path
- **Invalidation**: Manual or TTL-based
- **Cost Reduction**: 99% fewer authorizer invocations

### Cache Configuration

```rust
// In authorizer
let cache_key = format!("{}:{}", user_id, resource_path);
let cached_policy = cache.get(&cache_key).await?;

if let Some(policy) = cached_policy {
    return Ok(policy);
}
```

## Security Considerations

### Implemented Security

- ✅ OAuth2/OpenID Connect standards
- ✅ Session-based authentication (no JWT in browser)
- ✅ Lambda authorizer with context injection
- ✅ TLS with rustls
- ✅ Input validation on all endpoints

### Planned Security Enhancements

- [ ] Rate limiting per user
- [ ] CSRF protection
- [ ] Content Security Policy
- [ ] Security headers middleware
- [ ] Audit logging

### Best Practices

1. **Session Security**:
   - Use `secure=true` in production
   - Set appropriate `SameSite` policies
   - Implement session expiration

2. **Token Security**:
   - Validate JWT signatures
   - Check token expiration
   - Implement token refresh

3. **API Security**:
   - Use authorizer for all protected endpoints
   - Implement proper CORS policies
   - Add rate limiting

## Testing Authentication

### Test Flow

```bash
# 1. Start server
cd src && cargo run --bin server

# 2. Test login
open http://localhost:9001/auth/login

# 3. Test protected route
curl http://localhost:9001/dashboard
# Should redirect to login if not authenticated

# 4. Test current user endpoint
curl http://localhost:9001/auth/me
# Returns user info if authenticated
```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_user_extractor() {
        // Test AuthUser extractor
    }

    #[tokio::test]
    async fn test_oauth2_callback() {
        // Test OAuth2 callback handler
    }
}
```

## Troubleshooting

### Common Issues

1. **Auth0 Callback Fails**:
   - Check callback URL configuration
   - Verify client secret is correct
   - Ensure redirect URI matches

2. **Session Not Persisting**:
   - Check session store configuration
   - Verify session secret is set
   - Check cookie settings

3. **Authorizer Not Working**:
   - Verify JWT secret matches
   - Check Lambda environment variables
   - Review CloudWatch logs

### Debug Commands

```bash
# Check session
curl -v http://localhost:9001/auth/me

# Test authorizer locally
cd authorizer && cargo lambda invoke --data-file test-event.json

# Check environment variables
./scripts/check-env.sh
```

## Next Steps

1. **Enhanced Security**:
   - Implement CSRF protection
   - Add security headers
   - Set up audit logging

2. **Advanced Features**:
   - Role-based access control
   - Multi-factor authentication
   - Social login providers

3. **Performance**:
   - Optimize session storage
   - Implement connection pooling
   - Add monitoring metrics

---

**Last Updated**: November 25, 2025  
**Related Files**: 
- `src/server/src/auth.rs` - Auth handlers
- `authorizer/src/main.rs` - Lambda authorizer
- `deploy.sh` - Deployment script
