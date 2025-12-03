-- ============================================================================
-- Migration 010: Membership System
-- ============================================================================
-- Adds membership request system for private communities
-- ============================================================================

-- ============================================================================
-- ALTER COMMUNITIES TABLE
-- ============================================================================

-- Add membership_type column if not exists
ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  membership_type VARCHAR(20) DEFAULT 'public';

-- Add default_member_role column if not exists  
ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  default_member_role VARCHAR(20) DEFAULT 'member';

-- ============================================================================
-- MEMBERSHIP REQUESTS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS membership_requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status VARCHAR(20) DEFAULT 'pending',
  requested_role VARCHAR(20) DEFAULT 'member',
  message TEXT,
  reviewed_by UUID REFERENCES users(id),
  reviewed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(community_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_membership_requests_community ON membership_requests(community_id);
CREATE INDEX IF NOT EXISTS idx_membership_requests_user ON membership_requests(user_id);
CREATE INDEX IF NOT EXISTS idx_membership_requests_status ON membership_requests(status);
CREATE INDEX IF NOT EXISTS idx_membership_requests_created ON membership_requests(created_at DESC);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE membership_requests IS 'Stores join requests for private communities';
COMMENT ON COLUMN membership_requests.status IS 'pending, approved, rejected';
COMMENT ON COLUMN membership_requests.requested_role IS 'Role requested by user (usually member)';
COMMENT ON COLUMN membership_requests.message IS 'Optional message from user explaining why they want to join';
COMMENT ON COLUMN communities.membership_type IS 'public, private, or hybrid';
