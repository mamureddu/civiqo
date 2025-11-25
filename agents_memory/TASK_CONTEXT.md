# Task Context: Federation-Ready Community Architecture

**Date**: November 25, 2025  
**Agent 2 Planning**: Complete  
**Status**: Ready for Agent 1 Implementation

---

## 🎯 Objectives

Build community features with **federation-ready architecture**:

1. **Multi-Tenant**: Each host can run multiple communities
2. **Auth Flexibility**: Pluggable auth (local now, federated later)
3. **HTMX Ready**: Endpoints designed for cross-origin use
4. **Open Source Ready**: No hardcoded assumptions
5. **Lean First**: Build core features, add federation later

---

## 🏗️ Architecture Overview

### Current Focus (Lean)
```
┌─────────────────────────────────────────────────────────┐
│                    civiqo.com                            │
│                                                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│  │ Community A │  │ Community B │  │ Community C │      │
│  │ (public)    │  │ (private)   │  │ (public)    │      │
│  └─────────────┘  └─────────────┘  └─────────────┘      │
│                                                          │
│  Single Host, Multiple Communities, One Auth System     │
└─────────────────────────────────────────────────────────┘
```

### Future (Federation)
```
┌─────────────────┐     ┌─────────────────┐
│   civiqo.com    │     │  self-hosted    │
│  ┌───┐ ┌───┐   │     │  ┌───┐ ┌───┐   │
│  │ A │ │ B │   │ ←→  │  │ X │ │ Y │   │
│  └───┘ └───┘   │     │  └───┘ └───┘   │
└─────────────────┘     └─────────────────┘
```

### Design Principles for Federation-Ready Code

1. **No hardcoded host URLs** - Use config/env variables
2. **HTMX endpoints return fragments** - Can be embedded anywhere
3. **Auth is pluggable** - Interface-based, not hardcoded to Auth0
4. **Community IDs are unique** - BIGINT with unique_rowid()
5. **CORS-ready endpoints** - Can enable later for federation
6. **Clean API boundaries** - JSON API + HTMX endpoints separate

---

## ✅ Acceptance Criteria

### Functional (Current Sprint)
- [ ] Multiple communities per host
- [ ] Community CRUD (create, read, update, delete)
- [ ] Community membership (join, leave, roles)
- [ ] Public/private communities
- [ ] Community discovery (list, search)
- [ ] Owner/admin management

### Federation-Ready (Architecture)
- [ ] No hardcoded URLs in code
- [ ] HTMX endpoints return standalone fragments
- [ ] Auth layer is abstracted (AuthProvider trait)
- [ ] Config-driven deployment
- [ ] CORS can be enabled per-endpoint
- [ ] Community data is self-contained

### Non-Functional
- [ ] Zero compilation errors
- [ ] All tests passing
- [ ] Clean separation of concerns
- [ ] Documented API endpoints

---

## 🔑 Key Requirements

### Authentication
- All endpoints require `AuthUser` extractor
- Unauthenticated requests return 401

### Authorization
- Update/Delete: Only owner can modify
- Check `created_by = user_id`
- Return 403 if not owner

### Validation
- **name**: 3-100 chars, required
- **description**: max 1000 chars, optional
- **slug**: 3-50 chars, unique, lowercase, alphanumeric + hyphens
- **is_public**: boolean, optional, default true

### Database
- Use BIGINT for community.id (not UUID)
- Use transactions for create (community + member)
- CASCADE delete handles related records
- Update `updated_at` timestamp

---

## 🚨 Important Constraints

1. **BIGINT IDs**: communities.id is BIGINT (i64), not UUID
2. **Slug Uniqueness**: Check before insert, return 409 if exists
3. **Owner Check**: Always verify created_by = user_id for update/delete
4. **Transaction**: Create must insert community + add creator as admin member
5. **Cascade Delete**: Test thoroughly - deletes members, boundaries, businesses, etc.

---

## 📊 Technical Approach

### Stack
- **Framework**: Axum
- **Database**: SQLx with CockroachDB
- **Validation**: validator crate
- **Auth**: tower-sessions + AuthUser extractor
- **Templates**: Tera (for error pages if needed)

### Response Format
- Success: Redirect or JSON response
- Error: Proper HTTP status codes + error details

---

## ⚡ Performance Targets

- Create: < 200ms
- Update: < 150ms
- Delete: < 150ms

---

## 🔗 Related Files

- `src/server/src/handlers/api.rs` - Add handlers here
- `src/server/src/models/community.rs` - Create this file
- `src/server/src/main.rs` - Add routes
- `src/migrations/001_initial_schema_with_bigint.sql` - Schema reference

---

## 🔮 Federation-Ready Architecture Notes

### Why This Matters
This code will be released open-source. Self-hosted instances should:
- Run multiple communities on one host
- Each community can federate independently (via unique code)
- Accept users from local auth OR federated auth (UUID collision unlikely)
- Optionally register with civiqo.com aggregator

### Key Concepts

#### 1. Community Codes
Each community has a unique `code` (like a federation ID):
```sql
ALTER TABLE communities ADD COLUMN code VARCHAR(20) UNIQUE NOT NULL;
-- Example: "cvq_abc123", "cvq_xyz789"
```
- Used for federation identification
- Allows individual community federation
- Format: `{instance_prefix}_{random}` (e.g., `cvq_abc123`)

#### 2. Dual-Auth Support
Users can authenticate via:
- **Local auth** (instance's own Auth0/OAuth)
- **Federated auth** (civiqo.com or other federated instance)

Since user IDs are UUIDs, collision is extremely unlikely (~1 in 2^122).
Both auth sources can coexist in the same `users` table.

```sql
-- users table supports both auth sources
CREATE TABLE users (
    id UUID PRIMARY KEY,           -- UUID from ANY auth source
    auth0_id VARCHAR(255) UNIQUE,  -- Local auth identifier
    federated_from VARCHAR(255),   -- NULL = local, or "civiqo.com", "other.instance"
    ...
);
```

### Current Implementation (Lean)
- Single host, multiple communities
- Auth0 for authentication
- Community codes for future federation
- No federation features yet (but ready)

### Future-Proofing Checklist
- [ ] No hardcoded URLs (use APP_URL env var)
- [ ] Community codes generated on creation
- [ ] HTMX fragments are self-contained
- [ ] Auth extractor accepts any valid UUID
- [ ] User table has `federated_from` field (NULL for local)
- [ ] API endpoints can have CORS enabled later
- [ ] Config file for deployment settings

---

## 🎨 Code Patterns & Styling Guidelines

### Rust Code Style

```rust
// ============================================================================
// ✅ GOOD PATTERNS
// ============================================================================

// 1. Use environment variables for URLs
let app_url = std::env::var("APP_URL")
    .unwrap_or_else(|_| "http://localhost:9001".to_string());

// 2. Generate community codes on creation
fn generate_community_code() -> String {
    let prefix = std::env::var("INSTANCE_PREFIX").unwrap_or_else(|_| "cvq".to_string());
    let random: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}_{}", prefix, random.to_lowercase())
}

// 3. Auth extractor accepts any UUID (local or federated)
pub struct AuthUser {
    pub user_id: Uuid,           // Works for any auth source
    pub federated_from: Option<String>,  // NULL = local
}

// 4. Standalone HTMX handlers (no assumptions about parent)
pub async fn community_card(
    Path(community_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    // Return self-contained HTML fragment
    Html(format!(r#"
        <div class="community-card" data-community-id="{}">
            <!-- All styles inline or from shared CSS -->
        </div>
    "#, community_id))
}

// ============================================================================
// ❌ BAD PATTERNS - AVOID THESE
// ============================================================================

// 1. Hardcoded URLs
let app_url = "https://civiqo.com";  // ❌ Never do this

// 2. Assuming single auth source
if !user.auth0_id.starts_with("auth0|") {  // ❌ Too restrictive
    return Err(Unauthorized);
}

// 3. HTMX assuming parent context
Html(r#"<div hx-target="#main-content">..."#)  // ❌ Assumes #main-content exists

// 4. Hardcoded instance prefix
let code = format!("civiqo_{}", random);  // ❌ Should use env var
```

### SQL Style

```sql
-- ============================================================================
-- ✅ GOOD: Federation-ready schema
-- ============================================================================

-- Community with unique code for federation
CREATE TABLE communities (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    code VARCHAR(20) UNIQUE NOT NULL,  -- Federation identifier
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    -- ... other fields
);

-- User supports multiple auth sources
CREATE TABLE users (
    id UUID PRIMARY KEY,
    auth0_id VARCHAR(255) UNIQUE,
    federated_from VARCHAR(255),  -- NULL = local, or source instance URL
    email VARCHAR(255) NOT NULL,
    -- ... other fields
);

-- Index for federated lookups
CREATE INDEX idx_users_federated_from ON users(federated_from);
CREATE INDEX idx_communities_code ON communities(code);
```

### HTMX/HTML Style

```html
<!-- ============================================================================
     ✅ GOOD: Standalone HTMX fragments
     ============================================================================ -->

<!-- Self-contained card (works anywhere) -->
<div class="community-card" 
     id="community-{{ community.id }}"
     data-code="{{ community.code }}">
    <h3>{{ community.name }}</h3>
    <p>{{ community.description }}</p>
    
    <!-- Use relative URLs or data attributes -->
    <button hx-post="/api/communities/{{ community.id }}/join"
            hx-target="this"
            hx-swap="outerHTML">
        Join
    </button>
</div>

<!-- ============================================================================
     ❌ BAD: Fragments with external dependencies
     ============================================================================ -->

<!-- Assumes parent has #main-content -->
<div hx-target="#main-content">...</div>

<!-- Hardcoded absolute URL -->
<a href="https://civiqo.com/communities/123">...</a>

<!-- Assumes specific CSS class exists in parent -->
<div class="parent-specific-layout">...</div>
```

### Environment Variables

```bash
# ============================================================================
# Required for federation-ready deployment
# ============================================================================

# Instance identification
APP_URL=https://my-instance.com
INSTANCE_PREFIX=myinst              # Used in community codes: myinst_abc123

# Authentication
AUTH0_DOMAIN=my-tenant.auth0.com
AUTH0_CLIENT_ID=xxx
AUTH0_CLIENT_SECRET=xxx

# Federation (future - not required now)
# FEDERATION_ENABLED=false
# AGGREGATOR_URL=https://civiqo.com
# FEDERATION_PRIVATE_KEY=xxx
```

### Federation Details
See `federation_management_plan/` for detailed federation implementation plans.

---

## 📝 Notes

- Schema already optimized with BIGINT
- Indexes already exist on slug and created_by
- Auth system already working
- Database already connected and tested
- Federation planning complete (implement later)
