-- ============================================================================
-- Migration 002: Communities
-- ============================================================================
-- Tables: communities, community_members, community_boundaries
-- 
-- ID Strategy:
-- - communities.id: UUID (app generates via Uuid::now_v7())
-- - community_members.id: BIGINT (DB generates, app INSERTs without id)
-- - community_boundaries.id: BIGINT (DB generates, not used in app)
-- ============================================================================

-- ============================================================================
-- COMMUNITIES (UUID - app generates)
-- ============================================================================

CREATE TABLE communities (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(100) UNIQUE NOT NULL,
    code VARCHAR(20) UNIQUE,  -- Federation code (e.g., "cvq_abc123")
    is_public BOOLEAN DEFAULT true,
    requires_approval BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_code ON communities(code);
CREATE INDEX idx_communities_created_by ON communities(created_by);
CREATE INDEX idx_communities_is_public ON communities(is_public);
CREATE INDEX idx_communities_created_at ON communities(created_at DESC);
CREATE INDEX idx_communities_name_lower ON communities(lower(name));

-- ============================================================================
-- COMMUNITY MEMBERS (BIGINT - DB generates, junction table)
-- ============================================================================

CREATE TABLE community_members (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id),
    status VARCHAR(50) DEFAULT 'active',  -- 'active', 'pending', 'banned'
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(community_id, user_id)
);

CREATE INDEX idx_community_members_community ON community_members(community_id);
CREATE INDEX idx_community_members_user ON community_members(user_id);
CREATE INDEX idx_community_members_role ON community_members(role_id);
CREATE INDEX idx_community_members_status ON community_members(status);

-- ============================================================================
-- COMMUNITY BOUNDARIES (BIGINT - DB generates, GeoJSON storage)
-- ============================================================================

CREATE TABLE community_boundaries (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    boundary_type VARCHAR(50) NOT NULL,  -- 'polygon', 'circle', etc.
    geojson TEXT NOT NULL,
    coordinates JSONB,  -- Alternative format
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_community_boundaries_community ON community_boundaries(community_id);
CREATE INDEX idx_community_boundaries_type ON community_boundaries(boundary_type);
