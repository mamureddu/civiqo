# 📋 Implementation Plan: Phase 2 - Posts & Comments

**Created by**: Agent 2 (Tech Lead)  
**Date**: November 25, 2025  
**Status**: Ready for Agent 1

---

## 🎯 Implementation Steps

### Step 1: Database Migration ⏳
**Priority**: HIGH  
**Estimated Time**: 30 minutes

- [ ] Create migration file `008_posts_comments_reactions.sql`
- [ ] Add posts table with all fields
- [ ] Add comments table with parent_id for threading
- [ ] Add reactions table with unique constraint
- [ ] Add all indexes
- [ ] Run migration: `sqlx migrate run`
- [ ] Regenerate SQLx cache: `cargo sqlx prepare`

### Step 2: Posts CRUD Endpoints ⏳
**Priority**: HIGH  
**Estimated Time**: 2 hours

- [ ] Add request/response types for posts
- [ ] Implement `create_post` handler
  - Verify user is community member
  - Validate input (title, content)
  - Insert into database
  - Return created post
- [ ] Implement `list_posts` handler
  - Pagination support
  - Sort by created_at DESC
  - Include author info
  - Include reaction counts
- [ ] Implement `get_post` handler
  - Include author info
  - Include reaction counts
  - Increment view count
- [ ] Implement `update_post` handler
  - Verify user is author
  - Update title/content
  - Set updated_at
- [ ] Implement `delete_post` handler
  - Verify user is author OR community admin
  - Delete post (cascade deletes comments/reactions)

### Step 3: Comments System ⏳
**Priority**: HIGH  
**Estimated Time**: 2 hours

- [ ] Add request/response types for comments
- [ ] Implement `create_comment` handler
  - Verify post exists
  - Verify user is community member
  - Support parent_id for replies
  - Insert into database
- [ ] Implement `list_comments` handler
  - Pagination support
  - Include author info
  - Support nested structure (threading)
- [ ] Implement `update_comment` handler
  - Verify user is author
  - Update content
  - Set is_edited = true
- [ ] Implement `delete_comment` handler
  - Verify user is author OR community admin
  - Delete comment (cascade deletes replies)

### Step 4: Reactions System ⏳
**Priority**: MEDIUM  
**Estimated Time**: 1 hour

- [ ] Add request/response types for reactions
- [ ] Implement `add_reaction` handler
  - Verify post exists
  - Use UPSERT for idempotency
  - Return updated reaction count
- [ ] Implement `remove_reaction` handler
  - Delete user's reaction
  - Return updated reaction count
- [ ] Implement `list_reactions` handler
  - Group by reaction_type
  - Return counts

### Step 5: Routes Configuration ⏳
**Priority**: HIGH  
**Estimated Time**: 15 minutes

- [ ] Add posts routes to main.rs
- [ ] Add comments routes to main.rs
- [ ] Add reactions routes to main.rs
- [ ] Verify all routes are correctly wired

### Step 6: Integration Tests ⏳
**Priority**: HIGH  
**Estimated Time**: 2 hours

- [ ] Create `posts_integration.rs`
  - Test create post
  - Test list posts
  - Test get post
  - Test update post (author only)
  - Test delete post (author/admin)
  - Test unauthorized access
- [ ] Create `comments_integration.rs`
  - Test create comment
  - Test reply to comment
  - Test list comments with threading
  - Test update comment
  - Test delete comment
  - Test unauthorized access
- [ ] Test reactions
  - Test add reaction
  - Test remove reaction
  - Test unique constraint

### Step 7: HTMX Fragments (Optional) ⏳
**Priority**: LOW  
**Estimated Time**: 1 hour

- [ ] Create `post-card.html` fragment
- [ ] Create `comment-thread.html` fragment
- [ ] Create `reaction-buttons.html` fragment

---

## 📊 Request/Response Types

### Posts

```rust
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub content_type: Option<String>, // "markdown" or "text"
    pub media_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub community_id: String,
    pub author_id: String,
    pub author_email: String,
    pub title: String,
    pub content: String,
    pub content_type: String,
    pub media_url: Option<String>,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub view_count: i64,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct PostsListResponse {
    pub posts: Vec<PostResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub has_next: bool,
    pub has_prev: bool,
}
```

### Comments

```rust
#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub post_id: String,
    pub author_id: String,
    pub author_email: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub is_edited: bool,
    pub created_at: String,
    pub updated_at: String,
    pub replies: Vec<CommentResponse>, // For nested structure
}

#[derive(Debug, Serialize)]
pub struct CommentsListResponse {
    pub comments: Vec<CommentResponse>,
    pub total: i64,
}
```

### Reactions

```rust
#[derive(Debug, Deserialize)]
pub struct AddReactionRequest {
    pub reaction_type: String, // "like", "upvote", "heart", etc.
}

#[derive(Debug, Serialize)]
pub struct ReactionResponse {
    pub reaction_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ReactionsListResponse {
    pub reactions: Vec<ReactionResponse>,
    pub user_reaction: Option<String>, // Current user's reaction
}
```

---

## 🔒 Authorization Rules

### Posts
- **Create**: User must be community member
- **Read**: Public communities = anyone, Private = members only
- **Update**: Author only
- **Delete**: Author OR community admin

### Comments
- **Create**: User must be community member
- **Read**: Same as post access
- **Update**: Author only
- **Delete**: Author OR community admin

### Reactions
- **Add/Remove**: Authenticated users only
- **Read**: Same as post access

---

## ⚠️ Edge Cases to Handle

1. **Empty content**: Reject with 400 Bad Request
2. **Very long content**: Limit to 50,000 characters
3. **Invalid parent_id**: Return 404 for comment replies
4. **Deleted post**: Return 404 for all operations
5. **Locked post**: Prevent new comments if is_locked = true
6. **Duplicate reaction**: Use UPSERT to handle gracefully

---

## 🧪 Test Scenarios

### Posts
1. Create post as member ✓
2. Create post as non-member ✗
3. Update post as author ✓
4. Update post as non-author ✗
5. Delete post as author ✓
6. Delete post as admin ✓
7. Delete post as non-author ✗
8. List posts with pagination ✓
9. Get post detail ✓

### Comments
1. Create comment on post ✓
2. Reply to comment ✓
3. Update comment as author ✓
4. Delete comment as author ✓
5. Delete comment as admin ✓
6. List comments with threading ✓

### Reactions
1. Add reaction ✓
2. Change reaction type ✓
3. Remove reaction ✓
4. Get reaction counts ✓

---

## 📈 Progress Tracking

| Step | Status | Notes |
|------|--------|-------|
| 1. Database Migration | ⏳ | |
| 2. Posts CRUD | ⏳ | |
| 3. Comments System | ⏳ | |
| 4. Reactions System | ⏳ | |
| 5. Routes | ⏳ | |
| 6. Tests | ⏳ | |
| 7. HTMX Fragments | ⏳ | Optional |

---

**Ready for Agent 1 to begin implementation!**

