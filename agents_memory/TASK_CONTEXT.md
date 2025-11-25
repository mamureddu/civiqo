# Task Context: Community CRUD Routes Implementation

**Date**: November 25, 2025  
**Agent 2 Planning**: Complete  
**Status**: Ready for Agent 1 Implementation

---

## 🎯 Objectives

Implement complete CRUD operations for Communities:
1. **POST /api/communities** - Create new community
2. **PUT /api/communities/:id** - Update existing community
3. **DELETE /api/communities/:id** - Delete community

---

## ✅ Acceptance Criteria

### Functional
- [ ] Users can create communities (authenticated only)
- [ ] Users can update their own communities (owner only)
- [ ] Users can delete their own communities (owner only)
- [ ] All operations have proper validation
- [ ] All operations have proper error handling
- [ ] Database transactions ensure consistency

### Non-Functional
- [ ] Zero compilation errors
- [ ] All tests passing (189+)
- [ ] Response time < 200ms per operation
- [ ] Proper logging for debugging
- [ ] SQL injection prevention
- [ ] XSS prevention

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

## 📝 Notes

- Schema already optimized with BIGINT
- Indexes already exist on slug and created_by
- Auth system already working
- Database already connected and tested
