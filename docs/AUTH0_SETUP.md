# Auth0 Setup for HTMX + WASM

## Overview

Con HTMX, l'autenticazione sarà **session-based** invece di JWT frontend. Auth0 gestirà l'autenticazione, ma le sessioni saranno gestite server-side con cookies.

## Architettura

```
User Browser
     ↓
  HTMX Page
     ↓
  Actix-web (session cookie)
     ↓
  Auth0 (OAuth2 flow)
```

## Setup Auth0

### 1. Crea Application

1. Vai su [Auth0 Dashboard](https://manage.auth0.com/)
2. Applications → Create Application
3. Nome: "Community Manager Web"
4. Type: **Regular Web Application**
5. Technology: **Rust**

### 2. Configura Callbacks

**Allowed Callback URLs:**
```
http://localhost:9001/auth/callback
https://your-domain.com/auth/callback
```

**Allowed Logout URLs:**
```
http://localhost:9001
https://your-domain.com
```

**Allowed Web Origins:**
```
http://localhost:9001
https://your-domain.com
```

### 3. Ottieni Credenziali

Copia da Settings:
- **Domain**: `your-tenant.auth0.com`
- **Client ID**: `abc123...`
- **Client Secret**: `xyz789...`

### 4. Configura .env

```bash
# Auth0 Configuration
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_CLIENT_ID=your-client-id
AUTH0_CLIENT_SECRET=your-client-secret
AUTH0_CALLBACK_URL=http://localhost:9001/auth/callback

# Session Configuration
SESSION_SECRET=your-random-secret-key-here
SESSION_COOKIE_NAME=community_manager_session
SESSION_MAX_AGE=86400  # 24 hours
```

## Implementazione (TODO)

### 1. Session Middleware

```rust
// Aggiungeremo tower-sessions per gestire le sessioni
use tower_sessions::{Session, SessionManagerLayer};

// In main.rs
let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false) // true in production
    .with_same_site(SameSite::Lax);

app.layer(session_layer)
```

### 2. Auth Routes

```rust
// GET /auth/login - Redirect to Auth0
// GET /auth/callback - Handle Auth0 callback
// GET /auth/logout - Clear session and logout
// GET /auth/me - Get current user (for HTMX)
```

### 3. Protected Routes

```rust
// Middleware per verificare sessione
async fn require_auth(
    session: Session,
    request: Request,
    next: Next,
) -> Response {
    if let Some(user_id) = session.get::<String>("user_id") {
        // User authenticated
        next.run(request).await
    } else {
        // Redirect to login
        Redirect::to("/auth/login")
    }
}
```

### 4. HTMX Integration

```html
<!-- In templates, check auth status -->
<div hx-get="/auth/me" hx-trigger="load">
    <!-- User info loaded here -->
</div>

<!-- Protected actions -->
<button hx-post="/communities" 
        hx-headers='{"X-CSRF-Token": "..."}'>
    Create Community
</button>
```

## Flow Completo

### Login Flow

1. User clicca "Sign In" → GET `/auth/login`
2. Server redirect a Auth0 login page
3. User autentica su Auth0
4. Auth0 redirect a `/auth/callback?code=...`
5. Server scambia code per tokens
6. Server salva user_id in session
7. Server redirect a homepage
8. HTMX carica user info da `/auth/me`

### Protected Page Flow

1. User visita `/communities/create`
2. Middleware verifica session
3. Se no session → redirect `/auth/login`
4. Se session valida → render page
5. HTMX actions includono session cookie automaticamente

## Vantaggi Session-Based

✅ **Sicurezza**: No JWT in localStorage  
✅ **Semplicità**: Cookie gestiti automaticamente  
✅ **HTMX-friendly**: No JavaScript per auth  
✅ **Server-side**: Controllo completo sessioni  
✅ **Revocabile**: Logout immediato  

## Prossimi Passi

1. [ ] Installare `tower-sessions`
2. [ ] Implementare auth routes
3. [ ] Creare middleware auth
4. [ ] Aggiornare templates con auth
5. [ ] Testare flow completo

---

**Nota**: Per ora le pagine HTMX funzionano senza autenticazione. Auth0 sarà integrato nella prossima fase.
