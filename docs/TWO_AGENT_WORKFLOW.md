# Two-Agent Development Workflow

## Overview

This document defines the collaborative workflow between two specialized agents for implementing Communities CRUD functionality in the Community Manager project.

### Architecture Context
- **Backend**: Rust with Axum framework
- **Frontend**: HTMX + TailwindCSS + Alpine.js
- **Database**: CockroachDB Cloud (PostgreSQL-compatible)
- **Authentication**: Auth0 OAuth2 with session management
- **Testing**: SQLx offline mode + cargo test
- **Deployment**: AWS Lambda via cargo-lambda

### Brand Guidelines
**MANDATORY**: All UI/UX work must comply with `brand_id/Civiqo_Brand_Book_v1.1.pdf`
- Reference: `docs/BRAND_GUIDELINES.md` memory entry
- Assets location: `civiqo_assets_structured/`
- Compliance checklist required for all PRs

---

## Agent 1: Fullstack Executor

### Role Definition
**Primary responsibility**: Implement complete features from backend to frontend, ensuring all components work together seamlessly.

### Branch Workflow
```bash
# Create feature branch
git checkout -b feature/communities-crud

# Work in isolation
# ... implement features ...

# Ensure everything works
cargo build --workspace
SQLX_OFFLINE=true cargo test --workspace

# Create PR when complete
gh pr create --title "Communities CRUD Implementation" --body "See checklist"
```

### Definition of Done
A feature is complete when:
- [ ] Code compiles without errors (`cargo build --workspace`)
- [ ] All tests pass (`SQLX_OFFLINE=true cargo test --workspace`)
- [ ] Database migrations work locally
- [ ] UI follows brand guidelines exactly
- [ ] All endpoints work via browser testing
- [ ] Error handling is comprehensive
- [ ] Code is properly documented
- [ ] No dead code warnings (except documented futures)

---

## Agent 2: Tech Lead Verifier

### Role Definition
**Primary responsibility**: Review, validate, and ensure code quality, security, and brand compliance before merge.

### Review Process
```bash
# Review PR
gh pr view [PR_NUMBER]

# Checkout and test locally
git checkout pr/[PR_NUMBER]
cargo build --workspace
SQLX_OFFLINE=true cargo test --workspace

# Test functionality manually
# ... browser testing ...

# Approve or request changes
gh pr review [PR_NUMBER] --approve
# OR
gh pr review [PR_NUMBER] --comment "Issues found..."
```

### Review Checklist
#### Code Quality
- [ ] Rust code follows idiomatic patterns
- [ ] SQL queries are optimized and secure
- [ ] Error handling is comprehensive
- [ ] No hardcoded values (use env vars)
- [ ] Proper logging implemented

#### Security
- [ ] Authentication/authorization checks
- [ ] SQL injection prevention
- [ ] XSS prevention in templates
- [ ] Proper session management
- [ ] Input validation

#### Brand Compliance
- [ ] Colors match brand hex codes
- [ ] Typography follows hierarchy
- [ ] Logo usage respects guidelines
- [ ] Icons use brand style
- [ ] Layouts follow brand patterns
- [ ] Assets from structured folders used

#### Testing
- [ ] Unit tests cover critical paths
- [ ] Integration tests work
- [ ] Database queries tested
- [ ] Error scenarios tested
- [ ] Manual browser testing successful

---

## Feature Implementation: Communities CRUD

### Phase 1: Create Community (Priority: HIGH)

#### Backend Tasks
1. **Database Schema**
   - Verify `communities` table has all required fields
   - Add missing columns if needed (category, visibility, etc.)
   - Create migration file if schema changes

2. **API Endpoints**
   - `POST /api/communities` - Create new community
   - Input validation (name, description, category)
   - Authentication check (must be logged in)
   - Database insert with proper error handling
   - Return community data or error

3. **Business Logic**
   - Community name uniqueness check
   - Default values for optional fields
   - Creator automatically becomes admin
   - Audit fields (created_at, updated_at)

#### Frontend Tasks
1. **UI Components**
   - Create community form (brand-compliant)
   - Modal or dedicated page for creation
   - Form validation feedback
   - Success/error message display

2. **HTMX Integration**
   - Form submission via HTMX
   - Dynamic validation feedback
   - Redirect to community after creation
   - Loading states during submission

3. **Template Updates**
   - Update dashboard with "Create Community" button
   - Follow brand guidelines for styling
   - Use structured assets where appropriate

#### Testing Requirements
1. **Unit Tests**
   - Community creation logic
   - Input validation
   - Database insert operations
   - Error scenarios

2. **Integration Tests**
   - Full API endpoint testing
   - Authentication flow
   - Database integration
   - HTMX form submission

---

### Phase 2: List/View Communities (Priority: HIGH)

#### Backend Tasks
1. **API Endpoints**
   - `GET /api/communities` - List all communities
   - `GET /api/communities/:id` - Get specific community
   - Pagination support
   - Filtering and sorting options

2. **Database Queries**
   - Optimized queries with JOINs
   - Member count aggregation
   - Recent activity per community
   - Permission checking

#### Frontend Tasks
1. **UI Components**
   - Communities list page
   - Community detail view
   - Search and filter interface
   - Pagination controls

2. **HTMX Integration**
   - Dynamic filtering
   - Infinite scroll or pagination
   - Real-time updates
   - Navigation between views

---

### Phase 3: Edit/Update Communities (Priority: MEDIUM)

#### Backend Tasks
1. **API Endpoints**
   - `PUT /api/communities/:id` - Update community
   - Permission checking (owner/admin only)
   - Partial update support
   - Validation of updated data

2. **Business Logic**
   - Ownership verification
   - Audit trail for changes
   - Validation of unique constraints
   - Rollback capability

#### Frontend Tasks
1. **UI Components**
   - Edit form (pre-populated)
   - Save/cancel actions
   - Confirmation dialogs
   - Success/error feedback

---

### Phase 4: Delete Communities (Priority: MEDIUM)

#### Backend Tasks
1. **API Endpoints**
   - `DELETE /api/communities/:id` - Delete community
   - Owner-only permission check
   - Cascade delete handling
   - Soft delete option

2. **Business Logic**
   - Verify no active dependencies
   - Handle member relationships
   - Content cleanup or archiving
   - Confirmation requirements

#### Frontend Tasks
1. **UI Components**
   - Delete confirmation modal
   - Warning messages
   - Undo functionality (if soft delete)
   - Redirect after deletion

---

## Technical Requirements

### Environment Setup
```bash
# Required environment variables
DATABASE_URL=postgresql://...
AUTH0_DOMAIN=...
AUTH0_CLIENT_ID=...
AUTH0_CLIENT_SECRET=...
AUTH0_CALLBACK_URL=...
SESSION_SECRET=...
JWT_SECRET=...
RUST_LOG=info
```

### Development Commands
```bash
# Build and test
cargo build --workspace
SQLX_OFFLINE=true cargo test --workspace

# Database operations
sqlx migrate run
cargo sqlx prepare --workspace

# Local testing
cd src && cargo run --bin server
```

### Code Standards
- Use SQLx for all database operations
- Follow Rust idiomatic patterns
- Implement proper error handling
- Use structured logging
- Document all public functions
- Follow brand guidelines for UI

---

## Collaboration Protocol

### PR Lifecycle State Diagram
```
[Executor Work] → [Draft PR] → [Verifier Review] → [Changes Requested] → [Re-review] → [Approved] → [Merge]
       ↑                ↓              ↓                   ↓              ↑           ↓
       └────────────────┘              └───────────────────┘              └───────────┘
```

### Handoff Criteria
**Executor → Verifier:**
- All Definition of Done items complete
- Comprehensive PR description with screenshots
- Manual testing completed successfully
- Brand compliance self-verified

**Verifier → Executor:**
- Clear, actionable feedback with file:line references
- Priority classification (Security/High/Medium/Low)
- Expected re-review timeframe
- Resources for implementation guidance

### Communication Channels
1. **PR Comments**: Technical feedback and requests
2. **PR Description**: Status updates and implementation notes
3. **Commit Messages**: Detailed change descriptions
4. **Branch Names**: Clear feature identification

---

## Quality Gates

### Phase Gates (must pass before proceeding)
1. **Phase 1 Gate**: Community creation fully functional
2. **Phase 2 Gate**: Community listing and viewing working
3. **Phase 3 Gate**: Edit functionality with proper permissions
4. **Phase 4 Gate**: Delete with safety measures

### Merge Gates (final approval criteria)
- Zero compilation errors and warnings
- 100% test coverage for critical paths
- Complete brand guideline compliance
- Security audit passed
- Performance benchmarks met
- Documentation complete

---

## Success Metrics

### Velocity Targets
- **Phase 1** (Create): 2-3 days
- **Phase 2** (List/View): 2-3 days
- **Phase 3** (Edit): 1-2 days
- **Phase 4** (Delete): 1-2 days
- **Total**: 6-10 days for complete CRUD

### Quality Metrics
- **Code Quality**: Zero critical issues in static analysis
- **Security**: Zero vulnerabilities in security scan
- **Brand Compliance**: 100% adherence verification
- **Test Coverage**: Minimum 80% for new code
- **Performance**: < 200ms response time for API calls

### Collaboration Metrics
- **Review Time**: < 24 hours for standard PRs
- **Re-work Rate**: < 20% changes requested after initial review
- **Merge Rate**: > 80% of PRs merged within 48 hours
- **Communication**: Clear, documented feedback on all reviews

---

## Risk Management

### Technical Risks
- **Database Schema Changes**: Mitigate with backward-compatible migrations
- **Authentication Issues**: Test with multiple Auth0 scenarios
- **Brand Compliance**: Continuous verification against guidelines

### Collaboration Risks
- **Communication Gaps**: Use structured feedback templates
- **Quality Inconsistency**: Clear Definition of Done criteria
- **Timeline Delays**: Daily standups and progress tracking

---

## Emergency Procedures

### Critical Bug Found Post-Merge
1. **Immediate Response**: Create hotfix branch within 1 hour
2. **Assessment**: Verifier leads impact analysis
3. **Fix**: Executor implements fix with priority review
4. **Deployment**: Fast-track deployment after verification

### Brand Compliance Issue
1. **Stop Work**: Immediately halt all UI development
2. **Review**: Verifier assesses compliance gap
3. **Guidance**: Update brand guidelines memory if needed
4. **Resume**: Continue with clarified requirements

---

## Documentation Requirements

### PR Documentation
- **Problem Statement**: What issue does this solve?
- **Implementation Summary**: How was it solved?
- **Testing Summary**: What was tested and how?
- **Brand Compliance**: How guidelines were followed?
- **Screenshots**: Visual evidence of UI implementation
- **Deployment Notes**: Any special deployment considerations

### Code Documentation
- **Public Functions**: Must have rustdoc comments
- **Complex Logic**: Inline comments explaining approach
- **Database Queries**: Comments on optimization choices
- **Security Measures**: Comments on protection mechanisms

---

## Continuous Improvement

### Retrospective Process
After each major phase:
1. **What Went Well**: Document successful practices
2. **What Could Improve**: Identify pain points
3. **Action Items**: Create improvement tasks
4. **Process Updates**: Modify workflow as needed

### Knowledge Sharing
- **Code Reviews**: Learning opportunities for both agents
- **Technical Decisions**: Document rationale in comments
- **Brand Guidelines**: Update memory with clarifications
- **Best Practices**: Create reusable patterns

---

## Final Deliverables

### Complete Communities CRUD System
- **Create**: Full community creation workflow
- **Read**: Comprehensive listing and detail views
- **Update**: Edit functionality with proper permissions
- **Delete**: Safe deletion with confirmation

### Production-Ready Codebase
- **Zero Critical Issues**: No security or performance problems
- **Full Test Coverage**: Comprehensive automated testing
- **Brand Compliant UI**: Perfect adherence to guidelines
- **Complete Documentation**: Ready for maintenance and scaling

### Workflow Documentation
- **Process Refinement**: Lessons learned documented
- **Templates Reusable**: Ready for future features
- **Quality Standards**: Established for team growth
- **Success Metrics**: Proven effectiveness of collaboration

---

## Quick Reference

### Agent 1 (Executor) Commands
```bash
# Start work
git checkout -b feature/communities-crud

# Daily checks
cargo build --workspace
SQLX_OFFLINE=true cargo test --workspace

# Submit for review
git push origin feature/communities-crud
gh pr create --title "Feature: Communities CRUD" --body "Implementation complete"
```

### Agent 2 (Verifier) Commands
```bash
# Review process
gh pr checkout [PR_NUMBER]
cargo build --workspace
SQLX_OFFLINE=true cargo test --workspace

# Manual testing
cd src && cargo run --bin server

# Provide feedback
gh pr review [PR_NUMBER] --approve  # OR --comment
```

### Success Checklist
- [ ] All phases implemented
- [ ] 100% test coverage
- [ ] Brand compliant UI
- [ ] Security verified
- [ ] Documentation complete
- [ ] Production ready

**This workflow ensures high-quality, brand-compliant feature development through clear responsibilities, systematic processes, and continuous collaboration between specialized agents.**
