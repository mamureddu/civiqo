-- ============================================================================
-- Migration 012: Add unique constraint for proposal votes
-- ============================================================================
-- Required for ON CONFLICT (user_id, proposal_id) DO UPDATE in vote handler

-- Drop the existing non-unique index
DROP INDEX IF EXISTS idx_votes_user_proposal;

-- Create unique index (one vote per user per proposal)
CREATE UNIQUE INDEX idx_votes_user_proposal ON votes(user_id, proposal_id) WHERE proposal_id IS NOT NULL;
