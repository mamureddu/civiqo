-- ============================================================================
-- Migration 010: User Profiles Enhancement & Search
-- ============================================================================
-- Adds:
-- 1. Additional profile fields (cover_image, is_public)
-- 2. User follows system
-- 3. Notifications system
-- 4. Full-text search indexes
-- ============================================================================

-- ============================================================================
-- PART 1: Extend user_profiles
-- ============================================================================

-- Add cover image for profile banner
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS cover_image TEXT;

-- Add privacy setting (public/private profile)
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS is_public BOOLEAN DEFAULT true;

-- Add avatar_url as alias for picture (some code uses this)
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS avatar_url TEXT;

-- ============================================================================
-- PART 2: User Follows System
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_follows (
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (follower_id, following_id),
    -- Prevent self-follows
    CONSTRAINT no_self_follow CHECK (follower_id != following_id)
);

-- Indexes for efficient follower/following queries
CREATE INDEX IF NOT EXISTS idx_user_follows_follower ON user_follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_following ON user_follows(following_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_created_at ON user_follows(created_at DESC);

-- ============================================================================
-- PART 3: Notifications System
-- ============================================================================

CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,  -- 'follow', 'comment', 'reaction', 'mention', 'join_request', 'post'
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    target_type VARCHAR(50),    -- 'post', 'comment', 'community', 'user'
    target_id VARCHAR(100),     -- UUID or BIGINT as string
    message TEXT,
    is_read BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for notification queries
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_user_unread ON notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notifications_actor ON notifications(actor_id);

-- ============================================================================
-- PART 4: Full-Text Search Support
-- ============================================================================

-- Note: CockroachDB supports GIN indexes for full-text search
-- Using ILIKE for simple search, can upgrade to full-text later

-- User search index (on email and profile name)
CREATE INDEX IF NOT EXISTS idx_user_profiles_name ON user_profiles(name);

-- Community search index
CREATE INDEX IF NOT EXISTS idx_communities_name_lower ON communities(lower(name));
CREATE INDEX IF NOT EXISTS idx_communities_description ON communities(description) WHERE description IS NOT NULL;

-- Post search index
CREATE INDEX IF NOT EXISTS idx_posts_title_lower ON posts(lower(title));

-- ============================================================================
-- PART 5: Follower/Following Count Cache (Optional Optimization)
-- ============================================================================

-- Add cached counts to user_profiles for performance
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS follower_count INT DEFAULT 0;
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS following_count INT DEFAULT 0;

-- ============================================================================
-- PART 6: Seed Default Roles if not exists
-- ============================================================================

INSERT INTO roles (name, description, is_default) 
VALUES ('member', 'Regular community member', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles (name, description, is_default) 
VALUES ('admin', 'Community administrator', false)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles (name, description, is_default) 
VALUES ('moderator', 'Community moderator', false)
ON CONFLICT (name) DO NOTHING;
