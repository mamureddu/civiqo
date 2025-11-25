# Testing Checklist: Community CRUD Routes

**Agent 1**: Track testing progress here  
**Update**: Check off items as tests pass

---

## Unit Tests

### Validation Tests
- [ ] `CreateCommunityRequest` - Valid data
- [ ] `CreateCommunityRequest` - Name too short (< 3 chars)
- [ ] `CreateCommunityRequest` - Name too long (> 100 chars)
- [ ] `CreateCommunityRequest` - Description too long (> 1000 chars)
- [ ] `CreateCommunityRequest` - Invalid slug (uppercase)
- [ ] `CreateCommunityRequest` - Invalid slug (special chars)
- [ ] `CreateCommunityRequest` - Slug too short (< 3 chars)
- [ ] `CreateCommunityRequest` - Slug too long (> 50 chars)
- [ ] `UpdateCommunityRequest` - Valid data
- [ ] `UpdateCommunityRequest` - All fields None (should fail)
- [ ] `UpdateCommunityRequest` - Name too short
- [ ] `UpdateCommunityRequest` - Name too long

---

## Integration Tests - POST /api/communities

### Success Cases
- [ ] Create community with all fields
- [ ] Create community with minimal fields (name + slug)
- [ ] Create community with is_public = false
- [ ] Verify creator added as admin member
- [ ] Verify response contains correct data

### Error Cases
- [ ] Duplicate slug returns 409 Conflict
- [ ] Invalid name (too short) returns 400
- [ ] Invalid name (too long) returns 400
- [ ] Invalid slug format returns 400
- [ ] Missing required fields returns 400
- [ ] Unauthenticated request returns 401

### Database Verification
- [ ] Community inserted with correct data
- [ ] Creator added to community_members
- [ ] Creator has 'admin' role
- [ ] created_at and updated_at set correctly

---

## Integration Tests - PUT /api/communities/:id

### Success Cases
- [ ] Update name only
- [ ] Update description only
- [ ] Update is_public only
- [ ] Update multiple fields
- [ ] Verify updated_at changed
- [ ] Verify response contains updated data

### Error Cases
- [ ] Non-owner update returns 403 Forbidden
- [ ] Non-existent community returns 404 Not Found
- [ ] Invalid name (too short) returns 400
- [ ] Invalid name (too long) returns 400
- [ ] All fields None returns 400
- [ ] Unauthenticated request returns 401

### Database Verification
- [ ] Only specified fields updated
- [ ] updated_at timestamp changed
- [ ] Other fields unchanged

---

## Integration Tests - DELETE /api/communities/:id

### Success Cases
- [ ] Delete own community returns 200/204
- [ ] Verify community deleted from database
- [ ] Verify cascade delete worked (members deleted)
- [ ] Verify cascade delete worked (boundaries deleted)

### Error Cases
- [ ] Non-owner delete returns 403 Forbidden
- [ ] Non-existent community returns 404 Not Found
- [ ] Unauthenticated request returns 401

### Database Verification
- [ ] Community removed from communities table
- [ ] Related community_members removed
- [ ] Related community_boundaries removed
- [ ] Related businesses removed (if any)

---

## Manual Testing

### Create Community
- [ ] Test via UI form (if exists)
- [ ] Test via curl/Postman
- [ ] Test with valid data
- [ ] Test with invalid data
- [ ] Test duplicate slug
- [ ] Test without authentication

### Update Community
- [ ] Test via UI form (if exists)
- [ ] Test via curl/Postman
- [ ] Test as owner
- [ ] Test as non-owner
- [ ] Test with valid data
- [ ] Test with invalid data

### Delete Community
- [ ] Test via UI button (if exists)
- [ ] Test via curl/Postman
- [ ] Test as owner
- [ ] Test as non-owner
- [ ] Verify cascade delete
- [ ] Verify redirect after delete

---

## Performance Testing

- [ ] Create community < 200ms
- [ ] Update community < 150ms
- [ ] Delete community < 150ms
- [ ] No N+1 queries
- [ ] Proper index usage

---

## Security Testing

### SQL Injection
- [ ] Test with SQL in name field
- [ ] Test with SQL in description field
- [ ] Test with SQL in slug field
- [ ] Verify parameterized queries used

### XSS Prevention
- [ ] Test with script tags in name
- [ ] Test with script tags in description
- [ ] Verify template escaping works

### Authorization
- [ ] Verify owner check for update
- [ ] Verify owner check for delete
- [ ] Verify AuthUser extractor works
- [ ] Test with different users

---

## Edge Cases

- [ ] Create with empty description
- [ ] Create with very long valid name (100 chars)
- [ ] Create with very long valid slug (50 chars)
- [ ] Update with same values (should succeed)
- [ ] Delete already deleted community (404)
- [ ] Concurrent creates with same slug (one should fail)

---

## Test Coverage

**Target**: 80%+ coverage for new code

- [ ] All handlers have tests
- [ ] All validation logic tested
- [ ] All error cases tested
- [ ] All success cases tested

---

## Test Execution

### Commands
```bash
# Run all tests
cd src && cargo test --workspace

# Run specific test file
cd src && cargo test --test community_crud_test

# Run with output
cd src && cargo test --workspace -- --nocapture

# Run with coverage (if tool installed)
cd src && cargo tarpaulin --workspace
```

### Results
- Total Tests: ___ (to be filled)
- Passing: ___ (to be filled)
- Failing: ___ (to be filled)
- Coverage: ___% (to be filled)

---

## 🚨 Test Failures

(Document any test failures and fixes here)

---

## ✅ Sign-off

- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] All manual tests completed
- [ ] Performance targets met
- [ ] Security tests passed
- [ ] Edge cases handled
- [ ] Coverage target met (80%+)

**Ready for Agent 2 Review**: ⏳ Not Yet
