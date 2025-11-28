# Civiqo - English Translations
# File: governance.ftl - Proposals, voting and decisions

# =============================================================================
# GENERAL GOVERNANCE
# =============================================================================

governance-title = Governance
governance-subtitle = Participate in your community's decisions
governance-tab-proposals = Proposals
governance-tab-decisions = Decisions
governance-tab-polls = Polls

# =============================================================================
# PROPOSALS
# =============================================================================

proposals-title = Proposals
proposals-create = New Proposal
proposals-empty = No active proposals
proposals-empty-subtitle = Create the first proposal for your community

proposals-filter-all = All
proposals-filter-active = Active
proposals-filter-approved = Approved
proposals-filter-rejected = Rejected
proposals-filter-pending = Pending

proposal-status-draft = Draft
proposal-status-active = Active
proposal-status-voting = Voting
proposal-status-approved = Approved
proposal-status-rejected = Rejected
proposal-status-implemented = Implemented

# Proposal card
proposal-by = Proposed by { $name }
proposal-created = Created on { $date }
proposal-deadline = Deadline: { $date }
proposal-votes = { $count ->
    [one] { $count } vote
   *[other] { $count } votes
}
proposal-comments = { $count ->
    [one] { $count } comment
   *[other] { $count } comments
}

# =============================================================================
# CREATE PROPOSAL
# =============================================================================

proposal-create-title = Create Proposal
proposal-create-subtitle = Propose an idea or change to the community

proposal-create-title-label = Title
proposal-create-title-placeholder = A clear and concise title
proposal-create-description-label = Description
proposal-create-description-placeholder = Describe your proposal in detail...
proposal-create-category-label = Category
proposal-create-deadline-label = Voting deadline
proposal-create-submit = Publish Proposal
proposal-create-save-draft = Save as draft

proposal-category-general = General
proposal-category-infrastructure = Infrastructure
proposal-category-events = Events
proposal-category-rules = Rules
proposal-category-budget = Budget
proposal-category-other = Other

proposal-create-success = Proposal created successfully!
proposal-create-error = Error creating proposal

# =============================================================================
# PROPOSAL DETAIL
# =============================================================================

proposal-detail-description = Description
proposal-detail-discussion = Discussion
proposal-detail-votes = Voting
proposal-detail-history = History

proposal-vote-for = For
proposal-vote-against = Against
proposal-vote-abstain = Abstain
proposal-vote-submit = Vote
proposal-vote-change = Change vote
proposal-vote-success = Vote recorded!
proposal-vote-already = You've already voted on this proposal

proposal-comment-placeholder = Add a comment...
proposal-comment-submit = Comment
proposal-comment-reply = Reply

# =============================================================================
# DECISIONS
# =============================================================================

decisions-title = Decisions
decisions-empty = No decisions recorded
decisions-filter-all = All
decisions-filter-recent = Recent
decisions-filter-important = Important

decision-status-pending = Pending
decision-status-approved = Approved
decision-status-rejected = Rejected
decision-status-implemented = Implemented

decision-made-on = Decision made on { $date }
decision-participants = { $count ->
    [one] { $count } participant
   *[other] { $count } participants
}

# =============================================================================
# POLLS
# =============================================================================

polls-title = Polls
polls-create = New Poll
polls-empty = No active polls

poll-status-active = Active
poll-status-closed = Closed
poll-status-draft = Draft

poll-votes = { $count ->
    [one] { $count } vote
   *[other] { $count } votes
}
poll-ends = Ends on { $date }
poll-ended = Ended on { $date }

# Create poll
poll-create-title = Create Poll
poll-create-question-label = Question
poll-create-question-placeholder = Your question...
poll-create-options-label = Options
poll-create-option-placeholder = Option { $number }
poll-create-add-option = Add option
poll-create-multiple = Allow multiple answers
poll-create-anonymous = Anonymous votes
poll-create-deadline-label = Deadline
poll-create-submit = Create Poll

poll-vote-submit = Vote
poll-vote-success = Vote recorded!
poll-results = Results
poll-your-vote = Your vote
