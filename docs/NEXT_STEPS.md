# 🎯 Next Steps - Immediate Actions

## 🚀 Prossime 24 Ore

> [!IMPORTANT]
> **Brand Compliance**: Tutte le nuove implementazioni UI devono seguire [BRAND_GUIDELINES.md](../docs/BRAND_GUIDELINES.md). Usa i colori `civiqo-*` e i font `font-brand`/`font-sans`.

### ✅ 1. OAuth2 Code Exchange (COMPLETATO!)
**File**: `src/server/src/auth.rs`

**Status**: ✅ DONE
- [x] Code exchange implementation
- [x] User info retrieval from Auth0
- [x] Session creation
- [x] Error handling with redirects
- [x] Logout endpoint

**What's implemented**:
- `login()` - Redirects to Auth0 authorization
- `callback()` - Handles OAuth2 callback, exchanges code for token, gets user info
- `sync_user_to_database()` - Syncs Auth0 user to local database
- `logout()` - Deletes session
- `AuthUser` extractor - For protected routes
- `OptionalAuthUser` extractor - For optional auth

---

### 2. UI Login/Logout & Dashboard (1-2 ore) ⭐ NEXT
**File**: `src/server/templates/base.html` + handlers

**What needs to be done**:
1. Create base template with navbar
2. Add login/logout buttons
3. Show user info when authenticated
4. Create dashboard page
5. Update all handlers to pass auth context

**Implementation**:
```html
<!-- src/server/templates/base.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{% block title %}Community Manager{% endblock %}</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <nav class="navbar">
        <div class="nav-brand">
            <a href="/">Community Manager</a>
        </div>
        <div class="nav-menu">
            <a href="/communities">Communities</a>
            {% if logged_in %}
                <a href="/dashboard">Dashboard</a>
                <span class="user-info">
                    {% if picture %}
                        <img src="{{ picture }}" alt="{{ username }}" class="avatar">
                    {% endif %}
                    {{ username }}
                </span>
                <button onclick="logout()" class="btn-logout">Logout</button>
            {% else %}
                <a href="/auth/login" class="btn-login">Login</a>
            {% endif %}
        </div>
    </nav>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <script>
        async function logout() {
            await fetch('/auth/logout', { method: 'POST' });
            window.location.href = '/';
        }
    </script>
</body>
</html>
```

**Update handlers**:
```rust
pub async fn index(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    
    if let Some(user) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &user.name.unwrap_or(user.email.clone()));
        ctx.insert("picture", &user.picture);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    let html = state.tera.render("index.html", &ctx)?;
    Ok(Html(html).into_response())
}
```

---

### 3. Dashboard Page (1 ora)
**File**: `src/server/templates/dashboard.html`

**What needs to be done**:
1. Create dashboard template (protected route)
2. Show user profile info
3. Show user's communities
4. Show recent activity
5. Add create community button

**Route**:
```rust
pub async fn dashboard(
    AuthUser(user): AuthUser,  // Protected - requires auth
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    // Get user's communities from database
    let communities = sqlx::query_as!(
        Community,
        "SELECT * FROM communities WHERE created_by = $1",
        Uuid::parse_str(&user.user_id)?
    )
    .fetch_all(&state.db.pool)
    .await?;
    
    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("communities", &communities);
    
    let html = state.tera.render("dashboard.html", &ctx)?;
    Ok(Html(html).into_response())
}
```

---

## 🚀 Prossimi 3 Giorni

### Giorno 1: Auth Core ✅ (DONE)
- [x] OAuth2 code exchange
- [x] User sync to database
- [x] Session management
- [x] Logout endpoint
- [x] Auth extractors (AuthUser, OptionalAuthUser)

### Giorno 1.5: UI & Dashboard ✅ (COMPLETED)
- [x] Base template with navbar
- [x] Login/logout buttons (conditional)
- [x] Dashboard page (protected with AuthUser)
- [x] User profile display (name, email, avatar)
- [x] Communities list in dashboard (from DB)
- [x] Recent activity in dashboard (from DB)
- [x] HTMX endpoints for dynamic loading
- [x] Database queries with proper joins
- [x] Tests created
- [x] Build successful (200 tests passing)

### Giorno 2: Authorizer Deploy 🚀
**Tasks**:
```bash
# 1. Build authorizer
cd authorizer
cargo lambda build --release --arm64

# 2. Deploy
cargo lambda deploy authorizer \
    --iam-role arn:aws:iam::YOUR_ACCOUNT:role/lambda-execution-role \
    --env-var AUTH0_DOMAIN=your-tenant.auth0.com \
    --env-var JWT_SECRET=your-secret \
    --env-var RUST_LOG=info

# 3. Test
cargo lambda invoke authorizer --data-file test-event.json

# 4. Configure API Gateway
# - Add Lambda Authorizer
# - Set cache TTL: 3600
# - Identity source: Authorization header

# 5. Test with real token
curl -H "Authorization: Bearer YOUR_TOKEN" \
  https://your-api.execute-api.region.amazonaws.com/prod/communities
```

### Giorno 3: Communities Complete 📝
**Tasks**:
- [ ] Implement create community (protected)
- [ ] Implement update community (owner only)
- [ ] Implement delete community (owner only)
- [ ] Add members management
- [ ] UI improvements
- [ ] Tests

---

## 📋 Checklist Settimanale

### Week 1: Auth & Core
- [ ] OAuth2 complete
- [ ] User sync
- [ ] Authorizer deployed
- [ ] API Gateway configured
- [ ] Communities CRUD
- [ ] Posts CRUD
- [ ] Basic UI

### Week 2: Features
- [ ] Comments system
- [ ] User profiles
- [ ] Search & filters
- [ ] Business entities
- [ ] Governance proposals

### Week 3: Advanced
- [ ] Chat WebSocket
- [ ] Notifications
- [ ] Payment integration
- [ ] Analytics
- [ ] Admin panel

### Week 4: Polish
- [ ] UI/UX improvements
- [ ] Performance optimization
- [ ] Testing
- [ ] Documentation
- [ ] Beta launch

---

## 🎯 Quick Commands

### Development
```bash
# Start server
cd src && cargo run --bin server

# Run tests
cargo test --workspace

# Run migrations
sqlx migrate run

# Update SQLx cache
cargo sqlx prepare --workspace

# Build authorizer
cd authorizer && cargo lambda build --release --arm64

# Test authorizer locally
cargo lambda invoke authorizer --data-file test-event.json
```

### Database
```bash
# Connect to CockroachDB
cockroach sql --url="$DATABASE_URL"

# Run specific migration
sqlx migrate run --source src/migrations

# Revert last migration
sqlx migrate revert
```

### Deploy
```bash
# Deploy authorizer
cd authorizer && ./deploy.sh

# Deploy server (esempio fly.io)
cd src && fly deploy

# Check logs
fly logs
```

---

## 📚 Resources

### Documentation
- [PROJECT_ROADMAP.md](./PROJECT_ROADMAP.md) - Piano completo
- [LAMBDA_AUTHORIZER_GUIDE.md](./LAMBDA_AUTHORIZER_GUIDE.md) - Guida authorizer
- [AUTH_GUIDE.md](./AUTH_GUIDE.md) - Guida autenticazione
- [API_GUIDE.md](./API_GUIDE.md) - Guida API

### External
- [Auth0 Docs](https://auth0.com/docs)
- [Axum Docs](https://docs.rs/axum)
- [SQLx Docs](https://docs.rs/sqlx)
- [cargo-lambda](https://www.cargo-lambda.info/)

---

## 💡 Tips

### Debug Auth Issues
```rust
// Add logging
tracing::info!("Code received: {}", code);
tracing::info!("Token response: {:?}", token_response);
tracing::info!("User info: {:?}", user_info);

// Check session
let user = session.get::<SessionData>("user").await?;
tracing::info!("Session user: {:?}", user);
```

### Test Auth Flow
```bash
# 1. Clear cookies
# 2. Go to /auth/login
# 3. Login with Auth0
# 4. Check redirect to /dashboard
# 5. Verify session at /auth/me
# 6. Check database for user
psql $DATABASE_URL -c "SELECT * FROM users WHERE auth0_id = 'auth0|...'"
```

### Common Issues
- **"No code" error**: Check Auth0 callback URL configuration
- **Token exchange fails**: Verify client_id and client_secret
- **User not in DB**: Check migration ran successfully
- **Session not persisting**: Verify SESSION_SECRET is set

---

**Start with OAuth2 code exchange - it's the foundation for everything else!** 🚀
