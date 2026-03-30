-- ============================================================================
-- Migration 005: Governance (Proposals, Decisions, Polls, Votes)
-- ============================================================================
-- Tables: proposals, proposal_options, proposal_comments, decisions, 
--         decision_votes, polls, votes
-- 
-- ID Strategy:
-- - proposals.id: UUID (app generates via Uuid::now_v7())
-- - proposal_options.id: BIGINT (DB generates, app INSERTs without id)
-- - proposal_comments.id: BIGINT (DB generates, not used in app)
-- - decisions.id: UUID (app generates via Uuid::now_v7())
-- - decision_votes: Composite PK (no separate id)
-- - polls.id: UUID (app generates via Uuid::now_v7())
-- - votes.id: BIGINT (DB generates, high volume, app INSERTs without id)
-- ============================================================================

-- ============================================================================
-- PROPOSALS (UUID - app generates)
-- ============================================================================

CREATE TABLE proposals (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    proposal_type VARCHAR(50) DEFAULT 'general',  -- 'general', 'budget', 'policy', etc.
    proposal_data JSONB,
    status VARCHAR(50) DEFAULT 'draft',  -- 'draft', 'active', 'passed', 'rejected', 'cancelled'
    quorum_required INT DEFAULT 0,  -- Percentage 0-100
    voting_starts_at TIMESTAMPTZ,
    voting_ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_proposals_community ON proposals(community_id);
CREATE INDEX idx_proposals_created_by ON proposals(created_by);
CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposals_community_status ON proposals(community_id, status);
CREATE INDEX idx_proposals_created_by_status ON proposals(created_by, status);
CREATE INDEX idx_proposals_voting_dates ON proposals(voting_starts_at, voting_ends_at);

-- ============================================================================
-- PROPOSAL OPTIONS (BIGINT - DB generates, for multi-choice)
-- ============================================================================

CREATE TABLE proposal_options (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    option_text VARCHAR(255) NOT NULL,
    vote_count INT DEFAULT 0,
    display_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_proposal_options_proposal ON proposal_options(proposal_id);

-- ============================================================================
-- PROPOSAL COMMENTS (BIGINT - DB generates, not used in app)
-- ============================================================================

CREATE TABLE proposal_comments (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id BIGINT REFERENCES proposal_comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_proposal_comments_proposal ON proposal_comments(proposal_id);
CREATE INDEX idx_proposal_comments_user ON proposal_comments(user_id);
CREATE INDEX idx_proposal_comments_parent ON proposal_comments(parent_id);

-- ============================================================================
-- DECISIONS (UUID - app generates)
-- ============================================================================

CREATE TABLE decisions (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    proposal_id UUID REFERENCES proposals(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    decision_type VARCHAR(100),
    status VARCHAR(50) DEFAULT 'pending',  -- 'pending', 'approved', 'rejected'
    decision_makers JSONB,
    deadline TIMESTAMPTZ,
    result JSONB,
    decided_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_decisions_community ON decisions(community_id);
CREATE INDEX idx_decisions_proposal ON decisions(proposal_id);
CREATE INDEX idx_decisions_created_by ON decisions(created_by);
CREATE INDEX idx_decisions_status ON decisions(status);
CREATE INDEX idx_decisions_created_by_status ON decisions(created_by, status);

-- ============================================================================
-- DECISION VOTES (Composite PK - junction table)
-- ============================================================================

CREATE TABLE decision_votes (
    decision_id UUID NOT NULL REFERENCES decisions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    vote VARCHAR(50),  -- 'approve', 'reject', 'abstain'
    comment TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (decision_id, user_id)
);

CREATE INDEX idx_decision_votes_decision ON decision_votes(decision_id);
CREATE INDEX idx_decision_votes_user ON decision_votes(user_id);

-- ============================================================================
-- POLLS (UUID - app generates)
-- ============================================================================

CREATE TABLE polls (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    question TEXT NOT NULL,
    description TEXT,
    poll_type VARCHAR(50) DEFAULT 'single_choice',  -- 'single_choice', 'multiple_choice'
    options JSONB,
    settings JSONB,
    status VARCHAR(50) DEFAULT 'active',  -- 'draft', 'active', 'closed'
    starts_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_polls_community ON polls(community_id);
CREATE INDEX idx_polls_created_by ON polls(created_by);
CREATE INDEX idx_polls_status ON polls(status);
CREATE INDEX idx_polls_created_by_status ON polls(created_by, status);

-- ============================================================================
-- VOTES (BIGINT - DB generates, high volume)
-- ============================================================================

CREATE TABLE votes (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    poll_id UUID REFERENCES polls(id) ON DELETE CASCADE,
    proposal_id UUID REFERENCES proposals(id) ON DELETE CASCADE,
    option_id BIGINT REFERENCES proposal_options(id) ON DELETE SET NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    vote_data JSONB,
    vote_value VARCHAR(20),  -- 'yes', 'no', 'abstain' for simple votes
    vote_hash VARCHAR(255),  -- For anonymous voting verification
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_votes_poll ON votes(poll_id);
CREATE INDEX idx_votes_proposal ON votes(proposal_id) WHERE proposal_id IS NOT NULL;
CREATE INDEX idx_votes_user ON votes(user_id);
CREATE INDEX idx_votes_user_proposal ON votes(user_id, proposal_id) WHERE proposal_id IS NOT NULL;
