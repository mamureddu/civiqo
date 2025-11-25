-- Performance optimization indexes for communities API
-- Created as part of Agent 2 technical review recommendations

-- Index for public communities filtering
CREATE INDEX IF NOT EXISTS idx_communities_is_public 
ON communities(is_public);

-- Index for sorting by creation date
CREATE INDEX IF NOT EXISTS idx_communities_created_at 
ON communities(created_at DESC);

-- Index for slug-based lookups
CREATE INDEX IF NOT EXISTS idx_communities_slug 
ON communities(slug);

-- Index for community members status filtering
CREATE INDEX IF NOT EXISTS idx_community_members_status 
ON community_members(community_id, status);

-- Index for user's communities lookup
CREATE INDEX IF NOT EXISTS idx_communities_created_by 
ON communities(created_by);

-- Index for member count queries
CREATE INDEX IF NOT EXISTS idx_community_members_community_id 
ON community_members(community_id);
