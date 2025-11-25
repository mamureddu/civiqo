# 📋 Task Context: Phase 2 - Posts & Comments System

**Date**: November 25, 2025  
**Phase**: 2 of 9  
**Estimated Duration**: 2-3 weeks  
**Status**: 🚧 IN PROGRESS

---

## 🎯 Objective

Implement a complete Posts & Comments system for communities, including:
- Posts CRUD (create, read, update, delete)
- Comments with threading support
- Reactions system (likes, upvotes)
- Full integration with existing community system

---

## ✅ Acceptance Criteria

### Posts
- [ ] Users can create posts in communities they are members of
- [ ] Posts have title, content (rich text), and optional media
- [ ] Posts can be edited by author only
- [ ] Posts can be deleted by author or community admin
- [ ] Posts are paginated and sortable
- [ ] Posts show author info and timestamps

### Comments
- [ ] Users can comment on posts
- [ ] Comments support threading (replies to comments)
- [ ] Comments can be edited by author only
- [ ] Comments can be deleted by author or community admin
- [ ] Comments are paginated

### Reactions
- [ ] Users can react to posts (like, upvote, etc.)
- [ ] Users can remove their reactions
- [ ] Reaction counts are aggregated and displayed
- [ ] One reaction type per user per post

### Security
- [ ] All endpoints require authentication
- [ ] Authorization checks for edit/delete operations
- [ ] SQL injection prevention (parameterized queries)
- [ ] Input validation on all fields

### Testing
- [ ] Integration tests for all endpoints
- [ ] Test authorization scenarios
- [ ] Test edge cases (empty content, long content, etc.)

---

## 📊 Database Schema

### Posts Table
```sql
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    content_type VARCHAR(20) DEFAULT 'markdown',
    media_url VARCHAR(500),
    is_pinned BOOLEAN DEFAULT false,
    is_locked BOOLEAN DEFAULT false,
    view_count BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_posts_community ON posts(community_id);
CREATE INDEX idx_posts_author ON posts(author_id);
CREATE INDEX idx_posts_created ON posts(created_at DESC);
```

### Comments Table
```sql
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_comments_post ON comments(post_id);
CREATE INDEX idx_comments_author ON comments(author_id);
CREATE INDEX idx_comments_parent ON comments(parent_id);
```

### Reactions Table
```sql
CREATE TABLE reactions (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(post_id, user_id)
);

CREATE INDEX idx_reactions_post ON reactions(post_id);
CREATE INDEX idx_reactions_user ON reactions(user_id);
```

---

## 🔗 API Endpoints

### Posts
| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/api/communities/:id/posts` | Create post | Required |
| GET | `/api/communities/:id/posts` | List posts | Optional |
| GET | `/api/posts/:id` | Get post detail | Optional |
| PUT | `/api/posts/:id` | Update post | Author only |
| DELETE | `/api/posts/:id` | Delete post | Author/Admin |

### Comments
| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/api/posts/:id/comments` | Create comment | Required |
| GET | `/api/posts/:id/comments` | List comments | Optional |
| PUT | `/api/comments/:id` | Update comment | Author only |
| DELETE | `/api/comments/:id` | Delete comment | Author/Admin |

### Reactions
| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/api/posts/:id/reactions` | Add reaction | Required |
| DELETE | `/api/posts/:id/reactions` | Remove reaction | Required |
| GET | `/api/posts/:id/reactions` | List reactions | Optional |

---

## 🏗️ Technical Approach

1. **Database First**: Create migration with all tables
2. **Posts CRUD**: Implement posts endpoints
3. **Comments System**: Implement comments with threading
4. **Reactions**: Implement reaction system
5. **Testing**: Comprehensive integration tests
6. **HTMX Fragments**: Create UI components

---

## ⚠️ Risk Assessment

| Risk | Mitigation |
|------|------------|
| Complex threading | Use recursive CTE for nested comments |
| Performance | Add proper indexes, pagination |
| Authorization | Reuse existing AuthUser extractor |
| Data integrity | Use ON DELETE CASCADE |

---

## 📁 Files to Create/Modify

### New Files
- `src/migrations/008_posts_comments_reactions.sql`
- `src/server/tests/posts_integration.rs`
- `src/server/tests/comments_integration.rs`
- `src/server/templates/fragments/post-card.html`
- `src/server/templates/fragments/comment-thread.html`

### Modified Files
- `src/server/src/handlers/api.rs` - Add posts/comments handlers
- `src/server/src/main.rs` - Add routes

---

## 🎯 Success Metrics

- [ ] All 13 endpoints implemented
- [ ] All tests passing
- [ ] Zero compilation errors
- [ ] Authorization working correctly
- [ ] Pagination working
- [ ] Threading working for comments

