# Environment Setup Guide

## File: `src/.env`

Copia questo contenuto in `/Users/mariomureddu/CascadeProjects/community-manager/src/.env`:

```bash
# ============================================
# Auth0 Configuration
# ============================================
# Get these from Auth0 Dashboard → Applications → Your App → Settings
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id-here
AUTH0_CLIENT_SECRET=your-client-secret-here
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback

# ============================================
# Session Configuration
# ============================================
SESSION_SECRET=your-random-secret-key-here-min-32-chars
SESSION_COOKIE_NAME=community_manager_session
SESSION_MAX_AGE=86400

# ============================================
# Logging
# ============================================
RUST_LOG=info

# ============================================
# Database (CockroachDB Cloud)
# ============================================
# Format: postgresql://user:password@host:port/database?sslmode=verify-full
DATABASE_URL=postgresql://user:pass@cluster.cockroachlabs.cloud:26257/database?sslmode=verify-full
```

## Step-by-Step Setup

### 1. Auth0 Account
- Vai a https://auth0.com
- Sign up (gratuito)
- Crea tenant: "community-manager-dev"

### 2. Create Application
1. Dashboard → Applications → Create Application
2. Name: "Community Manager Web"
3. Type: **Regular Web Application**
4. Click Create

### 3. Configure Callbacks
In Settings, aggiungi:

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

### 4. Get Credentials
Copia da Settings:
- **Domain**: `your-tenant.auth0.com`
- **Client ID**: (lungo string)
- **Client Secret**: (lungo string)

### 5. Create .env File
```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cat > .env << 'EOF'
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback
SESSION_SECRET=your-random-secret-key-min-32-chars
SESSION_COOKIE_NAME=community_manager_session
SESSION_MAX_AGE=86400
RUST_LOG=info
EOF
```

### 6. Verify .env
```bash
cat src/.env
```

Should show your Auth0 credentials (without exposing secrets in logs).

## Launch Commands

### Terminal 1 - Backend Server
```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cargo run --bin server
```

Expected output:
```
 INFO API Gateway listening on http://0.0.0.0:9001
 INFO HTMX pages available at http://localhost:9001
```

### Terminal 2 - Test Endpoints
```bash
# Homepage
curl http://localhost:9001

# API Fragment
curl http://localhost:9001/api/communities/recent

# Health Check
curl http://localhost:9001/health

# Auth (when integrated)
curl http://localhost:9001/auth/me
```

### Browser
- Homepage: http://localhost:9001
- Communities: http://localhost:9001/communities
- Chat: http://localhost:9001/chat/room-1

## Troubleshooting

### "Auth0 not configured"
- Check `src/.env` exists
- Verify `AUTH0_DOMAIN`, `AUTH0_CLIENT_ID`, `AUTH0_CLIENT_SECRET` are set
- Run `cargo run --bin server` again

### "Connection refused"
- Port 9001 already in use
- Kill: `pkill -f "target/debug/server"`
- Try again

### "Template not found"
- Make sure you're in `src/` directory when running
- Or use full path: `cargo run --bin server --manifest-path src/server/Cargo.toml`

## Next Steps

1. ✅ Create Auth0 account
2. ✅ Create application
3. ✅ Configure callbacks
4. ✅ Create .env file
5. ⏳ Launch server
6. ⏳ Test endpoints
7. ⏳ Integrate auth routes (fix type issues)
8. ⏳ Add login/logout UI
