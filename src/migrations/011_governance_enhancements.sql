-- ======================================================================
-- Migration 011: Governance System Enhancements
-- Extends proposals and votes tables for full governance functionality
-- ======================================================================

-- 1. Add quorum_required to proposals (percentage 0-100)
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS quorum_required INT DEFAULT 0;

-- 2. Create proposal_options for multi-choice polls
CREATE TABLE IF NOT EXISTS proposal_options (
    id INT8 PRIMARY KEY DEFAULT unique_rowid(),
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    option_text VARCHAR(255) NOT NULL,
    vote_count INT DEFAULT 0,
    display_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Add proposal_id to votes table (votes currently uses poll_id which is different)
ALTER TABLE votes ADD COLUMN IF NOT EXISTS proposal_id UUID REFERENCES proposals(id) ON DELETE CASCADE;

-- 4. Add option_id to votes for multi-choice voting
ALTER TABLE votes ADD COLUMN IF NOT EXISTS option_id INT8 REFERENCES proposal_options(id) ON DELETE SET NULL;

-- 5. Add vote_value for simple yes/no/abstain votes
ALTER TABLE votes ADD COLUMN IF NOT EXISTS vote_value VARCHAR(20);

-- 6. Indexes for performance
CREATE INDEX IF NOT EXISTS idx_proposals_community_status ON proposals(community_id, status);
CREATE INDEX IF NOT EXISTS idx_proposals_voting_dates ON proposals(voting_starts_at, voting_ends_at);
CREATE INDEX IF NOT EXISTS idx_proposal_options_proposal ON proposal_options(proposal_id);
CREATE INDEX IF NOT EXISTS idx_votes_proposal ON votes(proposal_id) WHERE proposal_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_votes_user_proposal ON votes(user_id, proposal_id) WHERE proposal_id IS NOT NULL;
