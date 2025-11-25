# Implementation Plan: Community CRUD Routes

**Agent 1 Executor**: Follow this step-by-step plan  
**Update Progress**: Check off items as you complete them

---

## Phase 1: Setup & Dependencies (30 min)

### Step 1.1: Add Dependencies
- [ ] Add `validator = "0.16"` to `server/Cargo.toml`
- [ ] Add `validator_derive = "0.16"` to `server/Cargo.toml`
- [ ] Run `cargo build --workspace` to verify

### Step 1.2: Create Models Module
- [ ] Create `src/server/src/models/` directory if not exists
- [ ] Create `src/server/src/models/mod.rs`
- [ ] Create `src/server/src/models/community.rs`
- [ ] Add `pub mod models;` to `src/server/src/lib.rs`

---

## Phase 2: POST /api/communities - Create (1-2 hours)

### Step 2.1: Define Request/Response Structs
- [ ] In `models/community.rs`, create `CreateCommunityRequest`
  - Fields: name, description, slug, is_public
  - Add validation attributes
- [ ] Create `CommunityResponse` struct
  - Fields: id (i64), name, description, slug, is_public, created_by

### Step 2.2: Implement Create Handler
- [ ] In `handlers/api.rs`, create `create_community` function
  - Signature: `async fn create_community(AuthUser(user): AuthUser, State(state): State<Arc<AppState>>, Form(req): Form<CreateCommunityRequest>) -> Result<Response, AppError>`
- [ ] Validate request using `.validate()`
- [ ] Check slug uniqueness in database
- [ ] Start database transaction
- [ ] Insert into communities table
- [ ] Insert creator as admin in community_members
- [ ] Commit transaction
- [ ] Return 201 Created with redirect or JSON

### Step 2.3: Add Route
- [ ] In `main.rs`, add route: `.route("/api/communities", post(api::create_community))`

### Step 2.4: Test Manually
- [ ] Test with curl: `curl -X POST http://localhost:9001/api/communities -d "name=Test&slug=test&description=Test"`
- [ ] Test duplicate slug (should return 409)
- [ ] Test invalid data (should return 400)
- [ ] Test without auth (should return 401)

---

## Phase 3: PUT /api/communities/:id - Update (1 hour)

### Step 3.1: Define Request Struct
- [ ] In `models/community.rs`, create `UpdateCommunityRequest`
  - Fields: name (Option), description (Option), is_public (Option)
  - Add validation attributes
  - At least one field must be Some

### Step 3.2: Implement Update Handler
- [ ] In `handlers/api.rs`, create `update_community` function
  - Signature: `async fn update_community(AuthUser(user): AuthUser, State(state): State<Arc<AppState>>, Path(id): Path<i64>, Form(req): Form<UpdateCommunityRequest>) -> Result<Response, AppError>`
- [ ] Validate request
- [ ] Check if community exists (404 if not)
- [ ] Check if user is owner (403 if not)
- [ ] Update only provided fields
- [ ] Update `updated_at` timestamp
- [ ] Return 200 OK with updated data

### Step 3.3: Add Route
- [ ] In `main.rs`, add route: `.route("/api/communities/:id", put(api::update_community))`

### Step 3.4: Test Manually
- [ ] Test update as owner (should succeed)
- [ ] Test update as non-owner (should return 403)
- [ ] Test update non-existent community (should return 404)
- [ ] Test without auth (should return 401)

---

## Phase 4: DELETE /api/communities/:id - Delete (30 min)

### Step 4.1: Implement Delete Handler
- [ ] In `handlers/api.rs`, create `delete_community` function
  - Signature: `async fn delete_community(AuthUser(user): AuthUser, State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<Response, AppError>`
- [ ] Check if community exists (404 if not)
- [ ] Check if user is owner (403 if not)
- [ ] Delete community (CASCADE handles related records)
- [ ] Return 200 OK or 204 No Content

### Step 4.2: Add Route
- [ ] In `main.rs`, add route: `.route("/api/communities/:id", delete(api::delete_community))`

### Step 4.3: Test Manually
- [ ] Test delete as owner (should succeed)
- [ ] Test delete as non-owner (should return 403)
- [ ] Test delete non-existent community (should return 404)
- [ ] Verify cascade delete (check community_members, etc.)
- [ ] Test without auth (should return 401)

---

## Phase 5: Testing (1-2 hours)

### Step 5.1: Unit Tests
- [ ] Test `CreateCommunityRequest` validation
- [ ] Test `UpdateCommunityRequest` validation
- [ ] Test slug format validation

### Step 5.2: Integration Tests
- [ ] Create `src/server/tests/community_crud_test.rs`
- [ ] Test POST /api/communities - Success (201)
- [ ] Test POST /api/communities - Duplicate slug (409)
- [ ] Test POST /api/communities - Invalid data (400)
- [ ] Test POST /api/communities - Unauthenticated (401)
- [ ] Test PUT /api/communities/:id - Success (200)
- [ ] Test PUT /api/communities/:id - Not owner (403)
- [ ] Test PUT /api/communities/:id - Not found (404)
- [ ] Test PUT /api/communities/:id - Unauthenticated (401)
- [ ] Test DELETE /api/communities/:id - Success (200/204)
- [ ] Test DELETE /api/communities/:id - Not owner (403)
- [ ] Test DELETE /api/communities/:id - Not found (404)
- [ ] Test DELETE /api/communities/:id - Unauthenticated (401)

### Step 5.3: Run All Tests
- [ ] Run `cargo test --workspace` from `src/` directory
- [ ] Verify all tests pass (189+)
- [ ] Fix any failing tests

---

## Phase 6: Documentation & Cleanup (30 min)

### Step 6.1: Add Documentation
- [ ] Add rustdoc comments to all handlers
- [ ] Add examples to comments
- [ ] Document error cases

### Step 6.2: Code Review Prep
- [ ] Run `cargo fmt` to format code
- [ ] Run `cargo clippy` to check for issues
- [ ] Commit changes with descriptive message
- [ ] Push to feature branch
- [ ] Create PR for Agent 2 review

---

## 📊 Progress Tracking

**Phase 1**: ⏳ Not Started  
**Phase 2**: ⏳ Not Started  
**Phase 3**: ⏳ Not Started  
**Phase 4**: ⏳ Not Started  
**Phase 5**: ⏳ Not Started  
**Phase 6**: ⏳ Not Started

**Overall**: 0% Complete

---

## 🚨 Blockers

(Document any blockers in BLOCKERS_AND_NOTES.md)

---

## ⏱️ Time Estimates

- Phase 1: 30 min
- Phase 2: 1-2 hours
- Phase 3: 1 hour
- Phase 4: 30 min
- Phase 5: 1-2 hours
- Phase 6: 30 min

**Total**: 4-6 hours
