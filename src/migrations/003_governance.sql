-- Governance and decision-making tables

-- Polls for community voting
CREATE TABLE polls (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    poll_type poll_type NOT NULL,
    options JSONB NOT NULL DEFAULT '[]', -- Array of poll options
    settings JSONB NOT NULL DEFAULT '{}', -- Poll configuration (anonymous, multiple choice, etc.)
    status poll_status DEFAULT 'draft',
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CHECK (ends_at > starts_at)
);

-- Poll votes (encrypted for privacy)
CREATE TABLE votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    poll_id UUID REFERENCES polls(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    vote_data JSONB NOT NULL, -- Encrypted vote choices
    vote_hash VARCHAR(255) NOT NULL, -- Hash for verification without revealing vote
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(poll_id, user_id)
);

-- Community decisions (formal decision-making process)
CREATE TABLE decisions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    decision_type decision_type NOT NULL,
    status decision_status DEFAULT 'pending',
    decision_makers JSONB NOT NULL DEFAULT '[]', -- Array of user IDs or roles
    deadline TIMESTAMPTZ,
    result JSONB, -- Final decision result
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Decision votes
CREATE TABLE decision_votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    decision_id UUID REFERENCES decisions(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    vote decision_vote_type NOT NULL,
    comment TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(decision_id, user_id)
);

-- Proposal system for community changes
CREATE TABLE proposals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    proposal_type VARCHAR(100) NOT NULL, -- 'rule_change', 'budget', 'member_action', etc.
    proposal_data JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(50) DEFAULT 'draft',
    voting_starts_at TIMESTAMPTZ,
    voting_ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Proposal comments/discussion
CREATE TABLE proposal_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposal_id UUID REFERENCES proposals(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES proposal_comments(id) ON DELETE CASCADE, -- For threaded comments
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for governance queries
CREATE INDEX idx_polls_community_id ON polls(community_id);
CREATE INDEX idx_polls_created_by ON polls(created_by);
CREATE INDEX idx_polls_status ON polls(status);
CREATE INDEX idx_polls_dates ON polls(starts_at, ends_at);
CREATE INDEX idx_votes_poll_id ON votes(poll_id);
CREATE INDEX idx_votes_user_id ON votes(user_id);
CREATE INDEX idx_decisions_community_id ON decisions(community_id);
CREATE INDEX idx_decisions_status ON decisions(status);
CREATE INDEX idx_decision_votes_decision_id ON decision_votes(decision_id);
CREATE INDEX idx_decision_votes_user_id ON decision_votes(user_id);
CREATE INDEX idx_proposals_community_id ON proposals(community_id);
CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposal_comments_proposal_id ON proposal_comments(proposal_id);
CREATE INDEX idx_proposal_comments_parent_id ON proposal_comments(parent_id);

-- Update triggers
CREATE TRIGGER update_polls_updated_at BEFORE UPDATE ON polls FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_decisions_updated_at BEFORE UPDATE ON decisions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_proposals_updated_at BEFORE UPDATE ON proposals FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_proposal_comments_updated_at BEFORE UPDATE ON proposal_comments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();