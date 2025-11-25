# 📝 Blockers & Notes: Phase 2 - Posts & Comments

**Created by**: Agent 2 (Tech Lead)  
**Date**: November 25, 2025

---

## 🚧 Current Blockers

*None at start of implementation*

---

## 📝 Implementation Notes

### Database Considerations
- Use UUID for posts and comments (federation-ready)
- Use BIGINT for reactions (high volume, no federation need)
- CASCADE deletes for referential integrity
- Indexes on foreign keys for performance

### Authorization Pattern
- Reuse existing `AuthUser` extractor
- Reuse existing `OptionalAuthUser` for public reads
- Check community membership for write operations
- Check author/admin status for edit/delete

### Threading Strategy
- Use `parent_id` for comment replies
- Fetch comments flat, build tree in Rust
- Limit nesting depth to 10 levels (optional)

### Reaction Types
- Start with: "like", "upvote", "heart", "celebrate"
- Store as VARCHAR(20) for flexibility
- Use UNIQUE constraint on (post_id, user_id)

---

## ❓ Questions for Agent 2

*Agent 1 can add questions here during implementation*

---

## ✅ Decisions Made

1. **UUID vs BIGINT**: UUID for posts/comments (federation-ready), BIGINT for reactions
2. **Threading**: Flat storage with parent_id, tree built in application
3. **Reactions**: One reaction per user per post (UNIQUE constraint)
4. **Content Type**: Support markdown and plain text
5. **Soft Delete**: Not implemented (hard delete with CASCADE)

---

## 🐛 Issues Encountered

*Agent 1 will document issues here*

---

## 💡 Improvements for Future

- [ ] Add post categories/tags
- [ ] Add post scheduling
- [ ] Add comment mentions (@user)
- [ ] Add rich media embeds
- [ ] Add post bookmarks
- [ ] Add comment reactions

---

## 📊 Progress Log

| Time | Action | Status |
|------|--------|--------|
| 23:24 | Phase 0 Planning Complete | ✅ |
| 23:26 | Step 1: Database Migration | ✅ |
| 23:30 | Step 2: Posts CRUD | ✅ |
| 23:32 | Step 3: Comments System | ✅ |
| 23:33 | Step 4: Reactions System | ✅ |
| 23:34 | Step 5: Routes | ✅ |
| 23:38 | Step 6: Tests (12 tests) | ✅ |

