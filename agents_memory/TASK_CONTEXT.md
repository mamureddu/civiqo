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
- Use their own auth OR federate with civiqo.com
- Optionally register with civiqo.com aggregator

### Current Implementation (Lean)
- Single host, multiple communities
- Auth0 for authentication
- No federation features yet

### Future-Proofing Checklist
- [ ] No hardcoded URLs (use APP_URL env var)
- [ ] HTMX fragments are self-contained (no external dependencies)
- [ ] Auth extractor is trait-based (can swap implementations)
- [ ] Community data doesn't assume single-host
- [ ] API endpoints can have CORS enabled later
- [ ] Config file for deployment settings

### Code Patterns to Follow
```rust
// ✅ Good: Use config
let app_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:9001".to_string());

// ❌ Bad: Hardcoded URL
let app_url = "https://civiqo.com";

// ✅ Good: HTMX fragment is standalone
<div class="community-card">...</div>

// ❌ Bad: HTMX fragment assumes parent context
<div hx-target="#main-content">...</div>  // Assumes #main-content exists
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
