-- Migration: Change community ID from BIGINT to UUID (UUIDv7)
-- Date: November 25, 2025
-- Purpose: Use UUIDv7 for globally unique, time-ordered community IDs
-- CockroachDB Compatible: Uses DROP/CREATE instead of ALTER COLUMN TYPE

-- ============================================================================
-- Why UUIDv7?
-- ============================================================================
-- 1. Globally unique: No collision across federated instances
-- 2. Time-ordered: Sortable, better index performance than UUIDv4
-- 3. Federation-ready: Works as both ID and federation identifier
-- 4. DB-friendly: Sequential nature improves insert performance
-- 5. No separate 'code' field needed: UUID IS the federation identifier

-- ============================================================================
-- Step 1: Drop all dependent tables (fresh DB, no data to preserve)
-- ============================================================================
-- CockroachDB doesn't support ALTER COLUMN TYPE in transactions
-- So we drop and recreate tables with correct types

DROP TABLE IF EXISTS polls CASCADE;
DROP TABLE IF EXISTS decisions CASCADE;
DROP TABLE IF EXISTS proposals CASCADE;
DROP TABLE IF EXISTS chat_rooms CASCADE;
DROP TABLE IF EXISTS businesses CASCADE;
DROP TABLE IF EXISTS community_boundaries CASCADE;
DROP TABLE IF EXISTS community_members CASCADE;
DROP TABLE IF EXISTS communities CASCADE;

-- ============================================================================
-- Step 2: Recreate communities table with UUID primary key
-- ============================================================================
CREATE TABLE communities (
    id UUID PRIMARY KEY,  -- UUIDv7 generated in application
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(100) UNIQUE NOT NULL,
    is_public BOOLEAN DEFAULT true,
    requires_approval BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Step 3: Recreate dependent tables with UUID community_id
-- ============================================================================

-- Community Members
CREATE TABLE community_members (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id),
    status VARCHAR(20) DEFAULT 'active',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(community_id, user_id)
);

-- Community Boundaries (GeoJSON)
CREATE TABLE community_boundaries (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    boundary_type VARCHAR(50) NOT NULL,
    geojson TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Businesses
CREATE TABLE businesses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    address TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    phone VARCHAR(50),
    email VARCHAR(255),
    website VARCHAR(500),
    is_verified BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Chat Rooms
CREATE TABLE chat_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    room_type VARCHAR(50) DEFAULT 'public',
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Proposals
CREATE TABLE proposals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    proposal_type VARCHAR(50) DEFAULT 'general',
    status VARCHAR(50) DEFAULT 'draft',
    voting_starts_at TIMESTAMP WITH TIME ZONE,
    voting_ends_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Decisions
CREATE TABLE decisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    proposal_id UUID REFERENCES proposals(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) DEFAULT 'pending',
    decided_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Polls
CREATE TABLE polls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    question TEXT NOT NULL,
    poll_type VARCHAR(50) DEFAULT 'single_choice',
    status VARCHAR(50) DEFAULT 'active',
    ends_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Step 4: Create indexes
-- ============================================================================
CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_created_by ON communities(created_by);
CREATE INDEX idx_communities_is_public ON communities(is_public);

CREATE INDEX idx_community_members_community ON community_members(community_id);
CREATE INDEX idx_community_members_user ON community_members(user_id);

CREATE INDEX idx_businesses_community ON businesses(community_id);
CREATE INDEX idx_businesses_owner ON businesses(owner_id);

CREATE INDEX idx_chat_rooms_community ON chat_rooms(community_id);

CREATE INDEX idx_proposals_community ON proposals(community_id);
CREATE INDEX idx_proposals_status ON proposals(status);

CREATE INDEX idx_decisions_community ON decisions(community_id);

CREATE INDEX idx_polls_community ON polls(community_id);

-- ============================================================================
-- Notes
-- ============================================================================
-- After this migration:
-- 1. communities.id is UUID (UUIDv7 for new communities)
-- 2. All community_id foreign keys are UUID
-- 3. Application generates UUIDv7 on community creation:
--    let id = Uuid::now_v7();
