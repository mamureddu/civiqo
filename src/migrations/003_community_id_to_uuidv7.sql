-- Migration: Change community ID from BIGINT to UUID (UUIDv7)
-- Date: November 25, 2025
-- Purpose: Use UUIDv7 for globally unique, time-ordered community IDs

-- ============================================================================
-- Why UUIDv7?
-- ============================================================================
-- 1. Globally unique: No collision across federated instances
-- 2. Time-ordered: Sortable, better index performance than UUIDv4
-- 3. Federation-ready: Works as both ID and federation identifier
-- 4. DB-friendly: Sequential nature improves insert performance
-- 5. No separate 'code' field needed: UUID IS the federation identifier

-- ============================================================================
-- Step 1: Truncate related tables (no data currently)
-- ============================================================================
TRUNCATE TABLE community_members CASCADE;
TRUNCATE TABLE community_boundaries CASCADE;
TRUNCATE TABLE businesses CASCADE;
TRUNCATE TABLE chat_rooms CASCADE;
TRUNCATE TABLE proposals CASCADE;
TRUNCATE TABLE decisions CASCADE;
TRUNCATE TABLE polls CASCADE;

-- ============================================================================
-- Step 2: Drop foreign key constraints
-- ============================================================================
ALTER TABLE community_members DROP CONSTRAINT IF EXISTS community_members_community_id_fkey;
ALTER TABLE community_boundaries DROP CONSTRAINT IF EXISTS community_boundaries_community_id_fkey;
ALTER TABLE businesses DROP CONSTRAINT IF EXISTS businesses_community_id_fkey;
ALTER TABLE chat_rooms DROP CONSTRAINT IF EXISTS chat_rooms_community_id_fkey;
ALTER TABLE proposals DROP CONSTRAINT IF EXISTS proposals_community_id_fkey;
ALTER TABLE decisions DROP CONSTRAINT IF EXISTS decisions_community_id_fkey;
ALTER TABLE polls DROP CONSTRAINT IF EXISTS polls_community_id_fkey;

-- ============================================================================
-- Step 3: Change community_id columns to UUID in related tables
-- ============================================================================
ALTER TABLE community_members ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE community_boundaries ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE businesses ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE chat_rooms ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE proposals ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE decisions ALTER COLUMN community_id TYPE UUID USING NULL;
ALTER TABLE polls ALTER COLUMN community_id TYPE UUID USING NULL;

-- ============================================================================
-- Step 4: Backup and recreate communities table with UUID primary key
-- ============================================================================
-- Store existing community data
CREATE TEMP TABLE communities_backup AS SELECT * FROM communities;

-- Drop and recreate communities table
DROP TABLE communities CASCADE;

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

-- Restore data with new UUIDs
INSERT INTO communities (id, name, description, slug, is_public, requires_approval, created_by, created_at, updated_at)
SELECT 
    gen_random_uuid(),  -- Generate UUID for existing data
    name, 
    description, 
    slug, 
    is_public, 
    requires_approval, 
    created_by, 
    created_at, 
    updated_at
FROM communities_backup;

DROP TABLE communities_backup;

-- ============================================================================
-- Step 5: Recreate foreign key constraints
-- ============================================================================
ALTER TABLE community_members 
ADD CONSTRAINT community_members_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE community_boundaries 
ADD CONSTRAINT community_boundaries_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE businesses 
ADD CONSTRAINT businesses_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE chat_rooms 
ADD CONSTRAINT chat_rooms_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE proposals 
ADD CONSTRAINT proposals_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE decisions 
ADD CONSTRAINT decisions_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

ALTER TABLE polls 
ADD CONSTRAINT polls_community_id_fkey 
FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

-- ============================================================================
-- Step 6: Create indexes
-- ============================================================================
CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_created_by ON communities(created_by);
CREATE INDEX idx_communities_is_public ON communities(is_public);

-- ============================================================================
-- Notes
-- ============================================================================
-- After this migration:
-- 1. communities.id is UUID (UUIDv7 for new communities)
-- 2. The UUID serves as both ID and federation identifier
-- 3. Application generates UUIDv7 on community creation:
--    let id = Uuid::now_v7();
-- 4. Existing community gets a random UUID (gen_random_uuid)
