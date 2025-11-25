# Implementation Plan: Community Features (Federation-Ready)

**Date**: November 25, 2025  
**Agent 2 Planning**: Complete  
**Estimated Time**: ~6-8 hours

---

## 📋 Current Status

### ✅ Already Done
- [x] Database schema with UUIDv7 for communities
- [x] `users.federated_from` field for dual-auth
- [x] Basic community CRUD handlers (partially updated for UUID)
- [x] Authentication system (Auth0)
- [x] Community create/update/delete handlers (need UUID fixes)

### ❌ Needs Work
- [ ] Fix UUID compilation issues (src/Cargo.toml updated)
- [ ] Update tests for UUID community IDs
- [ ] Add membership management endpoints
- [ ] Add public/private community logic
- [ ] Add community discovery (list, search)
- [ ] Add owner/admin management
- [ ] Add HTMX fragments for communities

---

## 🔧 Phase 1: Fix UUID Compilation & Tests (1 hour)

### Step 1.1: Verify UUID build
```bash
cd /Users/mariomureddu/CascadeProjects/community-manager/src
cargo build -p server
```

### Step 1.2: Update integration tests for UUID
**Files**: 
- `src/server/tests/community_crud_integration.rs`
- `src/server/tests/community_crud_integration_test.rs`

Changes needed:
- [ ] Change `i64` to `Uuid` for community_id
- [ ] Update `create_test_community()` to generate UUIDv7
- [ ] Update all test assertions for UUID
- [ ] Update cleanup functions for UUID

### Step 1.3: Run tests
```bash
cargo test -p server
```

---

## 🔧 Phase 2: Community Membership (1.5 hours)

### Step 2.1: Join Community
**Endpoint**: `POST /api/communities/:id/join`
**Auth**: Required

```rust
pub async fn join_community(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<Json<ApiResponse<MembershipResponse>>, StatusCode>
```

**Logic**:
1. Verify community exists
2. Check if public OR user has pending invite
3. Check user not already member
4. Get 'member' role ID from roles table
5. Insert into community_members
6. Return membership details

**SQL**:
```sql
-- Check community is public
SELECT is_public, requires_approval FROM communities WHERE id = $1

-- Check not already member
SELECT id FROM community_members WHERE community_id = $1 AND user_id = $2

-- Get member role
SELECT id FROM roles WHERE name = 'member'

-- Insert membership
INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
VALUES ($1, $2, $3, 'active', NOW())
```

### Step 2.2: Leave Community
**Endpoint**: `POST /api/communities/:id/leave`
**Auth**: Required

```rust
pub async fn leave_community(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode>
```

**Logic**:
1. Verify user is member
2. Check if user is the ONLY admin (prevent orphan)
3. If only admin, return error "Transfer ownership first"
4. Delete from community_members
5. Return success

**SQL**:
```sql
-- Check membership
SELECT role_id FROM community_members WHERE community_id = $1 AND user_id = $2

-- Count admins
SELECT COUNT(*) FROM community_members cm
JOIN roles r ON cm.role_id = r.id
WHERE cm.community_id = $1 AND r.name = 'admin'

-- Delete membership
DELETE FROM community_members WHERE community_id = $1 AND user_id = $2
```

### Step 2.3: List Members
**Endpoint**: `GET /api/communities/:id/members`
**Auth**: Optional (public communities) / Required (private)

```rust
pub async fn list_members(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<MembersListResponse>>, StatusCode>
```

**Response**:
```rust
pub struct MembersListResponse {
    pub members: Vec<MemberResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

pub struct MemberResponse {
    pub user_id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: String,
}
```

### Step 2.4: Update Member Role (Admin only)
**Endpoint**: `PUT /api/communities/:id/members/:user_id/role`
**Auth**: Required (must be admin)

```rust
pub async fn update_member_role(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode>
```

**Logic**:
1. Verify requester is admin of community
2. Verify target user is member
3. Validate role name exists
4. Update role_id in community_members
5. Return success

### Step 2.5: Remove Member (Admin only)
**Endpoint**: `DELETE /api/communities/:id/members/:user_id`
**Auth**: Required (must be admin)

---

## 🔧 Phase 3: Public/Private Communities (45 min)

### Step 3.1: Update Community Detail Access
**File**: `src/server/src/handlers/api.rs` - `get_community_detail`

**Logic**:
- Public community: Anyone can view
- Private community: Only members can view
- Already partially implemented, verify it works

### Step 3.2: Update Community List Access
**File**: `src/server/src/handlers/api.rs` - `list_communities`

**Logic**:
- Show all public communities
- Show private communities user is member of
- Filter: `WHERE is_public = true OR user is member`

### Step 3.3: Join Request for Private Communities
**Endpoint**: `POST /api/communities/:id/request-join`
**Auth**: Required

For communities with `requires_approval = true`:
1. Create pending membership request
2. Notify admins
3. Return "Request pending"

### Step 3.4: Approve/Reject Join Request (Admin only)
**Endpoint**: `PUT /api/communities/:id/requests/:request_id`
**Auth**: Required (must be admin)

---

## 🔧 Phase 4: Community Discovery (1 hour)

### Step 4.1: Search Communities
**Endpoint**: `GET /api/communities/search`
**Auth**: Optional

```rust
pub async fn search_communities(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<ApiResponse<CommunitiesListResponse>>, StatusCode>
```

**Query Params**:
```rust
pub struct SearchParams {
    pub q: Option<String>,        // Search term
    pub sort: Option<String>,     // "newest", "popular", "name"
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
```

**SQL**:
```sql
SELECT c.*, COUNT(m.user_id) as member_count
FROM communities c
LEFT JOIN community_members m ON c.id = m.community_id
WHERE c.is_public = true
  AND (c.name ILIKE $1 OR c.description ILIKE $1)
GROUP BY c.id
ORDER BY member_count DESC
LIMIT $2 OFFSET $3
```

### Step 4.2: My Communities
**Endpoint**: `GET /api/communities/my`
**Auth**: Required

Returns communities user is member of.

### Step 4.3: Trending/Popular Communities
**Endpoint**: `GET /api/communities/trending`
**Auth**: Optional

Returns communities sorted by recent activity/member count.

---

## 🔧 Phase 5: Owner/Admin Management (45 min)

### Step 5.1: Transfer Ownership
**Endpoint**: `POST /api/communities/:id/transfer-ownership`
**Auth**: Required (must be owner)

```rust
pub async fn transfer_ownership(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
    Json(payload): Json<TransferOwnershipRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode>
```

**Logic**:
1. Verify requester is owner (`created_by = user_id`)
2. Verify new owner is member
3. Update `communities.created_by` to new owner
4. Ensure new owner has admin role
5. Return success

### Step 5.2: Promote to Admin
**Endpoint**: `PUT /api/communities/:id/members/:user_id/promote`
**Auth**: Required (must be owner)

### Step 5.3: Demote from Admin
**Endpoint**: `PUT /api/communities/:id/members/:user_id/demote`
**Auth**: Required (must be owner)

---

## 🔧 Phase 6: HTMX Fragments (1 hour)

### Step 6.1: Community Card Fragment
**Endpoint**: `GET /api/htmx/communities/:id/card`

```html
<div class="community-card" id="community-{{ id }}">
    <h3>{{ name }}</h3>
    <p>{{ description }}</p>
    <span class="member-count">{{ member_count }} members</span>
    <div hx-get="/api/htmx/communities/{{ id }}/join-button" 
         hx-trigger="load"
         hx-swap="innerHTML"></div>
</div>
```

### Step 6.2: Join/Leave Button Fragment
**Endpoint**: `GET /api/htmx/communities/:id/join-button`

Returns:
- "Join" button if not member
- "Leave" button if member
- "Pending" if request pending
- Nothing if owner

### Step 6.3: Member List Fragment
**Endpoint**: `GET /api/htmx/communities/:id/members`

### Step 6.4: Community List Fragment
**Endpoint**: `GET /api/htmx/communities`

---

## 🔧 Phase 7: Routes & Final Testing (30 min)

### Step 7.1: Add all routes to main.rs

```rust
// Membership
.route("/api/communities/:id/join", post(api::join_community))
.route("/api/communities/:id/leave", post(api::leave_community))
.route("/api/communities/:id/members", get(api::list_members))
.route("/api/communities/:id/members/:user_id/role", put(api::update_member_role))
.route("/api/communities/:id/members/:user_id", delete(api::remove_member))

// Discovery
.route("/api/communities/search", get(api::search_communities))
.route("/api/communities/my", get(api::my_communities))
.route("/api/communities/trending", get(api::trending_communities))

// Admin
.route("/api/communities/:id/transfer-ownership", post(api::transfer_ownership))
.route("/api/communities/:id/members/:user_id/promote", put(api::promote_member))
.route("/api/communities/:id/members/:user_id/demote", put(api::demote_member))

// HTMX
.route("/api/htmx/communities/:id/card", get(htmx::community_card))
.route("/api/htmx/communities/:id/join-button", get(htmx::join_button))
.route("/api/htmx/communities/:id/members", get(htmx::community_members))
```

### Step 7.2: Regenerate SQLx cache
```bash
cd src && source .env && cargo sqlx prepare --workspace
```

### Step 7.3: Run all tests
```bash
cargo test --workspace
```

### Step 7.4: Manual testing
- Test join/leave flow
- Test public/private access
- Test admin operations
- Test HTMX fragments

---

## 📊 New Types

### Request Types
```rust
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,  // "admin", "moderator", "member"
}

#[derive(Debug, Deserialize)]
pub struct TransferOwnershipRequest {
    pub new_owner_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
```

### Response Types
```rust
#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Serialize)]
pub struct MembersListResponse {
    pub members: Vec<MemberResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub community_id: String,
    pub role: String,
    pub joined_at: String,
}
```

---

## ✅ Verification Checklist

### Phase 1: UUID & Tests
- [ ] Server builds with UUIDv7
- [ ] All tests updated for UUID
- [ ] Tests pass

### Phase 2: Membership
- [ ] join_community works (public)
- [ ] leave_community works
- [ ] list_members works
- [ ] update_member_role works (admin only)
- [ ] remove_member works (admin only)

### Phase 3: Public/Private
- [ ] Private communities hidden from non-members
- [ ] Join requests for approval-required communities
- [ ] Approve/reject flow works

### Phase 4: Discovery
- [ ] search_communities works
- [ ] my_communities works
- [ ] trending_communities works

### Phase 5: Admin
- [ ] transfer_ownership works
- [ ] promote_member works
- [ ] demote_member works

### Phase 6: HTMX
- [ ] All fragments standalone
- [ ] No hardcoded URLs
- [ ] Fragments work in isolation

### Phase 7: Final
- [ ] All routes added
- [ ] SQLx cache regenerated
- [ ] All tests pass
- [ ] Zero compilation errors

---

## 🚀 Execution Order

1. **Phase 1**: Fix UUID & Tests (CRITICAL)
2. **Phase 2**: Membership endpoints
3. **Phase 3**: Public/Private logic
4. **Phase 4**: Discovery endpoints
5. **Phase 5**: Admin management
6. **Phase 6**: HTMX fragments
7. **Phase 7**: Routes & final testing

---

## 📝 Federation-Ready Notes

All code must follow these patterns:
- UUIDv7 for community IDs: `Uuid::now_v7()`
- No hardcoded URLs: Use `APP_URL` env var
- HTMX fragments standalone: No parent assumptions
- Auth accepts any UUID: Local or federated users
