# 🧪 Testing Checklist: Phase 2 - Posts & Comments

**Created by**: Agent 2 (Tech Lead)  
**Date**: November 25, 2025

---

## 📋 Integration Tests Required

### Posts Tests (`posts_integration.rs`)

#### Create Post
- [ ] `test_create_post_as_member` - Member can create post
- [ ] `test_create_post_as_non_member` - Non-member cannot create post (403)
- [ ] `test_create_post_unauthenticated` - Unauthenticated returns 401
- [ ] `test_create_post_empty_title` - Empty title returns 400
- [ ] `test_create_post_empty_content` - Empty content returns 400

#### List Posts
- [ ] `test_list_posts_public_community` - Anyone can list posts
- [ ] `test_list_posts_private_community_member` - Member can list posts
- [ ] `test_list_posts_private_community_non_member` - Non-member cannot list (403)
- [ ] `test_list_posts_pagination` - Pagination works correctly
- [ ] `test_list_posts_empty` - Empty list returns correctly

#### Get Post
- [ ] `test_get_post_exists` - Returns post with details
- [ ] `test_get_post_not_found` - Returns 404
- [ ] `test_get_post_increments_view_count` - View count increases

#### Update Post
- [ ] `test_update_post_as_author` - Author can update
- [ ] `test_update_post_as_non_author` - Non-author cannot update (403)
- [ ] `test_update_post_not_found` - Returns 404

#### Delete Post
- [ ] `test_delete_post_as_author` - Author can delete
- [ ] `test_delete_post_as_admin` - Admin can delete
- [ ] `test_delete_post_as_non_author` - Non-author cannot delete (403)
- [ ] `test_delete_post_not_found` - Returns 404

---

### Comments Tests (`comments_integration.rs`)

#### Create Comment
- [ ] `test_create_comment_on_post` - Member can comment
- [ ] `test_create_comment_reply` - Can reply to comment
- [ ] `test_create_comment_invalid_parent` - Invalid parent returns 404
- [ ] `test_create_comment_locked_post` - Locked post returns 403
- [ ] `test_create_comment_unauthenticated` - Returns 401

#### List Comments
- [ ] `test_list_comments_flat` - Returns flat list
- [ ] `test_list_comments_threaded` - Returns nested structure
- [ ] `test_list_comments_empty` - Empty list returns correctly

#### Update Comment
- [ ] `test_update_comment_as_author` - Author can update
- [ ] `test_update_comment_sets_edited_flag` - is_edited becomes true
- [ ] `test_update_comment_as_non_author` - Non-author cannot update (403)

#### Delete Comment
- [ ] `test_delete_comment_as_author` - Author can delete
- [ ] `test_delete_comment_as_admin` - Admin can delete
- [ ] `test_delete_comment_cascades_replies` - Replies are deleted

---

### Reactions Tests

#### Add Reaction
- [ ] `test_add_reaction_to_post` - Can add reaction
- [ ] `test_add_reaction_changes_type` - Can change reaction type
- [ ] `test_add_reaction_unauthenticated` - Returns 401

#### Remove Reaction
- [ ] `test_remove_reaction` - Can remove reaction
- [ ] `test_remove_reaction_not_exists` - Returns 404

#### List Reactions
- [ ] `test_list_reactions_grouped` - Returns grouped counts
- [ ] `test_list_reactions_includes_user` - Includes current user's reaction

---

## 🔒 Security Tests

- [ ] All endpoints validate authentication
- [ ] All endpoints validate authorization
- [ ] SQL injection attempts are blocked
- [ ] Input validation rejects malicious content

---

## 📊 Performance Tests (Optional)

- [ ] List posts with 1000+ posts performs well
- [ ] Nested comments (10 levels) performs well
- [ ] Reaction counts aggregate efficiently

---

## ✅ Test Execution Commands

```bash
# Run all Phase 2 tests
cargo test -p server --test posts_integration
cargo test -p server --test comments_integration

# Run specific test
cargo test -p server --test posts_integration test_create_post_as_member

# Run with output
cargo test -p server --test posts_integration -- --nocapture
```

---

## 📈 Coverage Requirements

- **Minimum**: 80% code coverage for new handlers
- **Target**: 90% code coverage
- **Critical paths**: 100% coverage

---

## 🎯 Test Progress

| Category | Total | Passed | Failed | Pending |
|----------|-------|--------|--------|---------|
| Posts Create | 5 | 0 | 0 | 5 |
| Posts List | 5 | 0 | 0 | 5 |
| Posts Get | 3 | 0 | 0 | 3 |
| Posts Update | 3 | 0 | 0 | 3 |
| Posts Delete | 4 | 0 | 0 | 4 |
| Comments Create | 5 | 0 | 0 | 5 |
| Comments List | 3 | 0 | 0 | 3 |
| Comments Update | 3 | 0 | 0 | 3 |
| Comments Delete | 3 | 0 | 0 | 3 |
| Reactions | 6 | 0 | 0 | 6 |
| **TOTAL** | **40** | **0** | **0** | **40** |

