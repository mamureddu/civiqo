# Auth0 Integration Guide

## Current Status

✅ **Session Infrastructure**: tower-sessions integrated with MemoryStore
✅ **Auth Module**: Created with Auth0Config and SessionData structures
✅ **Auth Handlers**: login, callback, logout, get_current_user implemented
✅ **Session Layer**: Configured and working
⏳ **Routes**: Auth routes ready (need type fixes for Axum integration)
⏳ **Full Integration**: Ready for next phase

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

## Environment Variables Required

```bash
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback
```

## Implementation Status

### ✅ Completed
- Session store setup (MemoryStore)
- Session layer configuration
- Auth0Config struct
- SessionData struct
- Auth module with handlers

### ⏳ In Progress
- Fix type issues with auth route handlers
- Implement OAuth2 code exchange
- Add session middleware to protected routes
- Create login/logout UI in templates

### 📋 TODO
- Test Auth0 OAuth2 flow
- Add CSRF protection
- Implement session persistence (SQLx store)
- Add role-based access control
- Create protected page examples

## Quick Integration Steps

### 1. Fix Auth Route Types
The auth routes need proper type handling for Axum extractors. Current issue:
- `login()` returns `impl IntoResponse` but needs specific type
- `callback()` needs proper session handling
- `get_current_user()` needs correct response type

### 2. Update Templates
Add login/logout buttons to `base.html`:
```html
<div hx-get="/auth/me" hx-trigger="load" hx-target="#user-info">
    <!-- User info loaded here -->
</div>

<a href="/auth/login">Sign In</a>
<a href="/auth/logout">Sign Out</a>
```

### 3. Test Flow
```bash
# 1. Visit login page
curl http://localhost:9001/auth/login

# 2. Get redirected to Auth0
# 3. Auth0 redirects back to callback
# 4. Session created
# 5. Check session
curl http://localhost:9001/auth/me
```

## Session Storage Options

### Current: MemoryStore
- ✅ Simple, no dependencies
- ❌ Lost on server restart
- ❌ Not suitable for production

### Future: SQLx Store
```rust
use tower_sessions_sqlx_store::PostgresStore;

let session_store = PostgresStore::new(pool);
```

## Security Notes

- ✅ Session layer configured with `SameSite::Lax`
- ⚠️ `secure=false` for local development (set to `true` in production)
- ⚠️ MemoryStore not suitable for production
- ⚠️ Need CSRF token validation for state changes

## Next Steps

1. **Fix Type Issues**: Resolve Axum handler type mismatches
2. **Test OAuth2 Flow**: Verify Auth0 integration works
3. **Add UI**: Create login/logout UI in templates
4. **Persist Sessions**: Switch to SQLx store for production
5. **Add Protection**: Middleware for protected routes

## References

- [tower-sessions docs](https://docs.rs/tower-sessions/)
- [Auth0 OAuth2 flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow)
- [Axum extractors](https://docs.rs/axum/latest/axum/extract/)
