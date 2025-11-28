-- ============================================================================
-- Migration 004: Content (Posts, Comments, Reactions)
-- ============================================================================
-- Tables: posts, comments, reactions
-- 
-- ID Strategy:
-- - posts.id: UUID (app generates via Uuid::now_v7())
-- - comments.id: UUID (app generates via Uuid::now_v7())
-- - reactions.id: BIGINT (DB generates, high volume, app INSERTs without id)
-- ============================================================================

-- ============================================================================
-- POSTS (UUID - app generates)
-- ============================================================================

CREATE TABLE posts (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    content_type VARCHAR(20) DEFAULT 'markdown',  -- 'markdown', 'text', 'html'
    media_url VARCHAR(500),
    is_pinned BOOLEAN DEFAULT false,
    is_locked BOOLEAN DEFAULT false,
    view_count BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_posts_community ON posts(community_id);
CREATE INDEX idx_posts_author ON posts(author_id);
CREATE INDEX idx_posts_created_at ON posts(created_at DESC);
CREATE INDEX idx_posts_community_created ON posts(community_id, created_at DESC);
CREATE INDEX idx_posts_title_lower ON posts(lower(title));
CREATE INDEX idx_posts_is_pinned ON posts(is_pinned) WHERE is_pinned = true;

-- ============================================================================
-- COMMENTS (UUID - app generates)
-- ============================================================================

CREATE TABLE comments (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,  -- For threading
    content TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_comments_post ON comments(post_id);
CREATE INDEX idx_comments_author ON comments(author_id);
CREATE INDEX idx_comments_parent ON comments(parent_id);
CREATE INDEX idx_comments_post_created ON comments(post_id, created_at ASC);

-- ============================================================================
-- REACTIONS (BIGINT - DB generates, high volume)
-- ============================================================================

CREATE TABLE reactions (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL,  -- 'like', 'heart', 'celebrate', etc.
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(post_id, user_id)  -- One reaction per user per post
);

CREATE INDEX idx_reactions_post ON reactions(post_id);
CREATE INDEX idx_reactions_user ON reactions(user_id);
CREATE INDEX idx_reactions_post_type ON reactions(post_id, reaction_type);
