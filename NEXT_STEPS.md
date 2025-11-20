# 🎯 Next Steps - Immediate Actions

## 🚀 Prossime 24 Ore

### 1. OAuth2 Code Exchange (2-3 ore)
**File**: `src/server/src/auth.rs`

```rust
// Nel callback handler, dopo aver ricevuto il code:
pub async fn callback(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    session: Session,
) -> Result<Redirect, AppError> {
    let code = params.get("code").ok_or("No code")?;
    
    // 1. Exchange code for tokens
    let client = reqwest::Client::new();
    let token_response = client
        .post(format!("https://{}/oauth/token", env::var("AUTH0_DOMAIN")?))
        .json(&serde_json::json!({
            "grant_type": "authorization_code",
            "client_id": env::var("AUTH0_CLIENT_ID")?,
            "client_secret": env::var("AUTH0_CLIENT_SECRET")?,
            "code": code,
            "redirect_uri": env::var("AUTH0_CALLBACK_URL")?,
        }))
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;
    
    // 2. Get user info
    let user_info = client
        .get(format!("https://{}/userinfo", env::var("AUTH0_DOMAIN")?))
        .bearer_auth(&token_response.access_token)
        .send()
        .await?
        .json::<UserInfo>()
        .await?;
    
    // 3. Save to session
    session.insert("user", SessionData {
        user_id: user_info.sub,
        email: user_info.email,
        name: user_info.name,
    }).await?;
    
    // 4. Sync with database (next step)
    
    Ok(Redirect::to("/dashboard"))
}
```

**Test**:
```bash
# 1. Start server
cd src && cargo run --bin server

# 2. Open browser
open http://localhost:9001/auth/login

# 3. Login with Auth0
# 4. Verify redirect to /dashboard
# 5. Check session in /auth/me
```

---

### 2. User Sync Auth0 ↔ Database (1-2 ore)
**File**: `src/server/src/auth.rs`

```rust
// Dopo aver ottenuto user_info nel callback:

// Sync user to database
let user_id = sqlx::query_scalar::<_, Uuid>(
    "INSERT INTO users (id, auth0_id, email, username, picture, last_login)
     VALUES ($1, $2, $3, $4, $5, NOW())
     ON CONFLICT (auth0_id) DO UPDATE SET
        email = EXCLUDED.email,
        username = EXCLUDED.username,
        picture = EXCLUDED.picture,
        last_login = NOW()
     RETURNING id"
)
.bind(Uuid::new_v4())
.bind(&user_info.sub)
.bind(&user_info.email)
.bind(&user_info.name)
.bind(&user_info.picture)
.fetch_one(&state.db.pool)
.await?;

// Update session with local user_id
session.insert("user", SessionData {
    user_id: user_id.to_string(),
    auth0_id: user_info.sub,
    email: user_info.email,
    name: user_info.name,
}).await?;
```

**Migration necessaria**:
```bash
cd src
sqlx migrate add add_auth0_id_to_users
```

**File**: `src/migrations/007_add_auth0_id_to_users.sql`
```sql
ALTER TABLE users ADD COLUMN IF NOT EXISTS auth0_id VARCHAR(255) UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_login TIMESTAMP;
CREATE INDEX IF NOT EXISTS idx_users_auth0_id ON users(auth0_id);
```

**Run migration**:
```bash
sqlx migrate run
cargo sqlx prepare --workspace
```

---

### 3. UI Login/Logout (1 ora)
**File**: `src/server/templates/base.html`

```html
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

**Aggiornare tutti i handlers per passare `logged_in`**:
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

## 🚀 Prossimi 3 Giorni

### Giorno 1: Auth Complete ✅
- [x] OAuth2 code exchange
- [x] User sync
- [x] UI login/logout
- [ ] Test flow completo

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
