# Agent 1: Fullstack Executor - Implementation Prompt

## Your Mission
You are a **Fullstack Executor Agent** tasked with implementing Communities CRUD functionality for the Community Manager project. You work on a separate branch and deliver complete, tested features ready for PR review.

## Project Context
- **Stack**: Rust (Axum) + HTMX + TailwindCSS + CockroachDB + Auth0
- **Current Status**: Dashboard and authentication complete, ready for CRUD
- **Branch**: `feature/communities-crud`
- **Target**: Production-ready features with 100% test coverage

## MANDATORY REQUIREMENTS
### Brand Guidelines Compliance
**CRITICAL**: You MUST follow brand guidelines exactly:
- **Reference**: `brand_id/Civiqo_Brand_Book_v1.1.pdf` (read before any UI work)
- **Memory**: Load `brand-guidelines-mandatory` memory entry for context
- **Assets**: Use only assets from `civiqo_assets_structured/` folder
- **Checklist**: Complete brand compliance checklist for each UI component

### Technical Standards
- **Zero compilation errors**: `cargo build --workspace` must pass
- **All tests pass**: `SQLX_OFFLINE=true cargo test --workspace` must pass
- **No warnings**: Fix all warnings except documented dead code
- **Security**: Proper authentication/authorization on all endpoints
- **Performance**: Optimized SQL queries with proper indexing

---

## Phase 1: Create Community (START HERE)

### Backend Implementation

#### 1. Database Schema Verification
```bash
# Check current communities table
psql $DATABASE_URL -c "\d communities"

# Verify required columns exist:
- id (UUID, primary key)
- name (TEXT, not null, unique)
- description (TEXT)
- category (TEXT)
- visibility (TEXT: 'public'/'private')
- created_by (UUID, foreign key to users)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)
```

**If columns missing, create migration:**
```bash
# Create new migration
sqlx migrate add add_community_fields

# Add columns in migration file
# Example: ALTER TABLE communities ADD COLUMN category TEXT DEFAULT 'general';
```

#### 2. API Endpoint Implementation
**File**: `src/server/src/handlers/api.rs`

```rust
// Add this function
pub async fn create_community(
    AuthUser(user): AuthUser,  // Must be authenticated
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<CommunityResponse>, AppError>
```

**Required validation:**
- Name: Required, min 3 chars, max 100 chars, unique
- Description: Optional, max 500 chars
- Category: Optional, from predefined list
- Visibility: Optional, default 'public'

**Database insert:**
```sql
INSERT INTO communities (name, description, category, visibility, created_by, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
RETURNING *;
```

#### 3. Business Logic Rules
- Community names must be unique globally
- Creator automatically becomes community admin
- Set default visibility to 'public'
- Validate category against allowed values
- Handle database errors gracefully

### Frontend Implementation

#### 1. Create Community Form
**File**: `src/server/templates/partials/create_community.html`

**Requirements:**
- Follow brand guidelines exactly (colors, typography, spacing)
- Use brand assets from `civiqo_assets_structured/`
- Form validation with real-time feedback
- Loading states during submission
- Success/error message display

**HTML Structure:**
```html
<div class="modal" id="create-community-modal">
  <form hx-post="/api/communities" 
        hx-target="#create-community-result" 
        hx-swap="innerHTML">
    <div class="modal-header">
      <h3 class="text-xl font-bold">Create New Community</h3>
    </div>
    <div class="modal-body">
      <!-- Form fields following brand guidelines -->
    </div>
    <div class="modal-footer">
      <button type="submit" class="btn-primary">Create Community</button>
    </div>
  </form>
</div>
```

#### 2. HTMX Integration
- Form submission via POST to `/api/communities`
- Show loading spinner during submission
- Display success message and redirect
- Handle validation errors with inline feedback
- Close modal on success

#### 3. Dashboard Integration
**Update**: `src/server/templates/dashboard.html`
- Add "Create Community" button (brand-compliant)
- Trigger modal open with HTMX
- Update communities list after creation

### Testing Implementation

#### 1. Unit Tests
**File**: `src/server/tests/communities_test.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_community_success() {
        // Test successful community creation
    }
    
    #[tokio::test]
    async fn test_create_community_validation_errors() {
        // Test validation failures
    }
    
    #[tokio::test]
    async fn test_create_community_duplicate_name() {
        // Test unique constraint violation
    }
}
```

#### 2. Integration Tests
- Full API endpoint testing
- Authentication flow testing
- Database integration testing
- HTMX form submission testing

#### 3. View Interaction Tests (MANDATORY)
**File**: `src/server/tests/view_interactions_test.rs`

Every fullstack feature MUST include tests for all user interactions in the view.
Since views return HTMX fragments, test by:
1. Making HTTP requests to the page/fragment endpoints
2. Parsing the HTML response to find interactive elements
3. Simulating user interactions via HTMX endpoints
4. Verifying the response HTML contains expected content

**Example structure:**
```rust
#[tokio::test]
async fn test_create_community_form_interaction() {
    // 1. GET the page containing the form
    let response = client.get("/communities").send().await;
    assert!(response.status().is_success());
    
    // 2. Verify form exists with correct hx-post
    let html = response.text().await;
    assert!(html.contains("hx-post=\"/api/communities\""));
    
    // 3. POST to the HTMX endpoint (simulating form submit)
    let response = client.post("/api/communities")
        .json(&CreateCommunityRequest { name: "Test", ... })
        .send().await;
    
    // 4. Verify response fragment contains success message
    let html = response.text().await;
    assert!(html.contains("Community created") || response.status().is_success());
}
```

**Required tests per view:**
- All buttons/links with `hx-get`, `hx-post`, `hx-put`, `hx-delete`
- Form submissions and validation feedback
- Modal open/close interactions
- Pagination controls
- Search/filter functionality
- Success/error message display

---

## Phase 2: List/View Communities (NEXT)

### Backend Tasks
1. **GET /api/communities** - List with pagination
2. **GET /api/communities/:id** - Get specific community
3. **Database queries** with member counts and activity
4. **Permission checking** for private communities

### Frontend Tasks
1. **Communities list page** with search/filter
2. **Community detail view** with member list
3. **HTMX pagination** or infinite scroll
4. **Real-time updates** for activity

---

## Phase 3: Edit Communities

### Backend Tasks
1. **PUT /api/communities/:id** - Update endpoint
2. **Owner/admin permission check**
3. **Partial update support**
4. **Audit trail for changes**

### Frontend Tasks
1. **Edit form** (pre-populated)
2. **Save/cancel actions**
3. **Confirmation dialogs**
4. **Success/error feedback**

---

## Phase 4: Delete Communities

### Backend Tasks
1. **DELETE /api/communities/:id** - Delete endpoint
2. **Owner-only permission check**
3. **Cascade delete handling**
4. **Soft delete option**

### Frontend Tasks
1. **Delete confirmation modal**
2. **Warning messages**
3. **Undo functionality**
4. **Redirect after deletion**

---

## Development Workflow

### Daily Process
1. **Morning**: Check current branch status
2. **Implementation**: Work on current phase
3. **Testing**: Continuous testing throughout
4. **End of Day**: Ensure everything compiles and tests pass

### Commands to Run Frequently
```bash
# Build check
cargo build --workspace

# Test check
SQLX_OFFLINE=true cargo test --workspace

# Database operations
sqlx migrate run
cargo sqlx prepare --workspace

# Local testing
cd src && cargo run --bin server
```

### Git Workflow
```bash
# Create feature branch
git checkout -b feature/communities-crud

# Commit frequently with descriptive messages
git add .
git commit -m "feat: implement create community API endpoint"

# Push for review when phase complete
git push origin feature/communities-crud

# Create PR with comprehensive description
gh pr create --title "Communities CRUD Implementation" --body "## Features Implemented\n\n### Phase 1: Create Community\n- [x] API endpoint\n- [x] Frontend form\n- [x] Database integration\n- [x] Tests\n\n## Testing\n- [x] All tests pass\n- [x] Manual testing complete\n- [x] Brand guidelines followed\n\n## Screenshots\n[Add screenshots]"
```

---

## Quality Checklist (Complete before PR)

### Code Quality
- [ ] No compilation errors (`cargo build --workspace`)
- [ ] All tests pass (`SQLX_OFFLINE=true cargo test --workspace`)
- [ ] No warnings (except documented dead code)
- [ ] Proper error handling throughout
- [ ] SQL queries optimized and secure
- [ ] Authentication/authorization implemented
- [ ] Code follows Rust idiomatic patterns

### Brand Compliance
- [ ] Read `brand_id/Civiqo_Brand_Book_v1.1.pdf`
- [ ] Colors match brand hex codes exactly
- [ ] Typography follows brand hierarchy
- [ ] Used assets from `civiqo_assets_structured/`
- [ ] Logo usage respects guidelines
- [ ] Layout follows brand patterns
- [ ] Icons use brand style

### Testing
- [ ] Unit tests for all business logic
- [ ] Integration tests for API endpoints
- [ ] Database operations tested
- [ ] Error scenarios tested
- [ ] Manual browser testing successful
- [ ] HTMX functionality tested
- [ ] **View interaction tests** for all user interactions (MANDATORY)
- [ ] All `hx-*` attributes have corresponding test coverage

### Documentation
- [ ] Code comments for complex logic
- [ ] API documentation in comments
- [ ] README updated if needed
- [ ] PR description comprehensive
- [ ] Screenshots included

---

## Handoff to Verifier

When you complete a phase and create a PR:
1. **Ensure all checklist items are complete**
2. **Create comprehensive PR description**
3. **Include screenshots of UI**
4. **Test manually in browser**
5. **Tag verifier agent for review**

The verifier will review your code, test functionality, and either approve or request changes. Be prepared to address feedback quickly.

---

## Success Metrics

Your success is measured by:
- **Code Quality**: Zero errors, all tests pass
- **Feature Completeness**: All requirements implemented
- **Brand Compliance**: 100% adherence to guidelines
- **User Experience**: Seamless, intuitive interface
- **Security**: Proper authentication and validation
- **Performance**: Optimized queries and fast response

**Remember**: You are responsible for the complete implementation. Take pride in delivering production-ready code that the verifier can approve without hesitation.
