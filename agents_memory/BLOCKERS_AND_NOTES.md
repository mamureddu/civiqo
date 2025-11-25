# Blockers and Notes: Community CRUD Routes

**Agent 1**: Document issues, decisions, and questions here  
**Agent 2**: Provide guidance and answers

---

## 🚨 Blockers

(No blockers yet - Agent 1 will document any issues encountered)

---

## 💡 Technical Decisions

### Decision 1: Response Format
**Question**: Should endpoints return JSON or redirect?  
**Decision**: TBD by Agent 1 based on existing patterns  
**Rationale**: Check existing API endpoints for consistency

### Decision 2: Role ID Lookup
**Question**: How to get 'admin' role ID for community_members insert?  
**Options**:
1. Subquery: `(SELECT id FROM roles WHERE name = 'admin')`
2. Cache role IDs in AppState
3. Hardcode role ID (not recommended)

**Recommendation**: Use subquery for simplicity and correctness

### Decision 3: Transaction Handling
**Question**: How to handle transaction errors?  
**Recommendation**: 
- Rollback on any error
- Return appropriate HTTP status
- Log error details for debugging

---

## 📝 Implementation Notes

### Note 1: BIGINT vs UUID
**Important**: communities.id is BIGINT (i64), not UUID  
**Impact**: Use `i64` in Rust structs, not `Uuid`  
**Example**: `Path(id): Path<i64>`

### Note 2: Existing Indexes
**Good News**: Schema already has indexes on:
- `communities.slug` (unique)
- `communities.created_by`
- No need to add new indexes

### Note 3: Cascade Delete
**Schema**: ON DELETE CASCADE already configured for:
- community_members
- community_boundaries
- businesses
- proposals
- chat_rooms

**Action**: Test thoroughly to verify cascade works

---

## ❓ Questions for Agent 2

(Agent 1 will add questions here during implementation)

---

## 🔍 Important Findings

(Agent 1 will document discoveries here)

---

## 🐛 Bugs Found

(Document any bugs discovered during implementation)

---

## ⚡ Performance Notes

(Document any performance observations)

---

## 🔒 Security Notes

(Document security considerations and implementations)

---

## 📚 Useful References

### Existing Code Patterns
- `src/server/src/handlers/pages.rs` - AuthUser usage examples
- `src/server/src/handlers/api.rs` - Existing API handlers
- `src/server/src/auth.rs` - AuthUser extractor implementation

### Database Schema
- `src/migrations/001_initial_schema_with_bigint.sql` - Full schema

### Dependencies
- `validator` docs: https://docs.rs/validator/
- `axum` docs: https://docs.rs/axum/
- `sqlx` docs: https://docs.rs/sqlx/

---

## 📊 Progress Notes

(Agent 1 will update with progress notes during implementation)

---

## ✅ Completed Items

(Move completed items here from other sections)

---

**Last Updated**: November 25, 2025 (Agent 2 Planning Phase)
