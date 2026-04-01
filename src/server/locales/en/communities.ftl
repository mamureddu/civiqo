# Civiqo - English Translations
# File: communities.ftl - Communities and membership

# =============================================================================
# COMMUNITY LIST
# =============================================================================

communities-title = Communities
communities-subtitle = Discover and join communities in your area
communities-search-placeholder = Search communities...
communities-filter-all = All
communities-filter-public = Public
communities-filter-private = Private
communities-filter-my = My communities
communities-sort-recent = Most recent
communities-sort-popular = Most popular
communities-sort-name = Name A-Z
communities-empty = No communities found
communities-empty-subtitle = Create the first community in your area!

# =============================================================================
# COMMUNITY CARD
# =============================================================================

community-members-label = Members
community-members = { $count ->
    [one] { $count } member
   *[other] { $count } members
}
community-posts = { $count ->
    [one] { $count } post
   *[other] { $count } posts
}
community-public = Public
community-private = Private
community-verified = Verified
community-created-by = Created by { $name }
community-created-at = Created on { $date }

# =============================================================================
# CREATE COMMUNITY
# =============================================================================

community-create-title = Create a New Community
community-create-subtitle = Build a space for your local community to connect and collaborate

community-create-name-label = Community Name
community-create-name-placeholder = e.g., Downtown Neighborhood Association
community-create-name-hint = This will be used to generate a unique URL for your community
community-create-name-validation = Must be 3-255 characters and contain at least one letter or number

community-create-description-label = Description
community-create-description-placeholder = Describe your community's purpose, goals, and who should join...
community-create-description-hint = Optional: Help people understand what your community is about
community-create-description-max = Maximum 2000 characters

community-create-privacy-title = Privacy Settings
community-create-public-label = Public Community
community-create-public-hint = Anyone can find and join this community
community-create-approval-label = Require Approval
community-create-approval-hint = New members must be approved by an admin

community-create-submit = Create Community
community-create-cancel = Cancel
community-create-creating = Creating...

community-create-success = Community created successfully!
community-create-error = Error creating community
community-create-redirect = Redirecting to dashboard...

community-create-guidelines-title = Community Guidelines
community-create-guideline-1 = Communities should be inclusive and welcoming
community-create-guideline-2 = Focus on local engagement and collaboration
community-create-guideline-3 = You'll automatically become the community admin
community-create-guideline-4 = Community settings can be changed later

# Validation
community-name-required = Community name is required and cannot be empty
community-name-min = Name must be at least 3 characters
community-name-max = Name is too long (maximum 255 characters)
community-name-invalid = Name must contain at least one letter or number
community-description-max = Description is too long (maximum 2000 characters)

# =============================================================================
# COMMUNITY DETAIL
# =============================================================================

community-detail-about = About
community-detail-members = Members
community-detail-posts = Posts
community-detail-events = Events
community-detail-governance = Governance
community-detail-settings = Settings

community-about-description = Description
community-about-location = Location
community-about-created = Created
community-about-admin = Administrator

# =============================================================================
# MEMBERSHIP
# =============================================================================

community-join = Join
community-leave = Leave
community-request-join = Request to join
community-pending = Request pending
community-joined = You're a member

community-join-success = You've joined the community!
community-leave-success = You've left the community
community-leave-confirm = Are you sure you want to leave this community?
community-request-sent = Request sent! Waiting for approval.

# =============================================================================
# MEMBER MANAGEMENT
# =============================================================================

members-title = Members
members-search = Search members...
members-role-owner = Owner
members-role-admin = Admin
members-role-moderator = Moderator
members-role-member = Member
members-joined = Member since { $date }

members-promote = Promote
members-demote = Demote
members-remove = Remove
members-ban = Ban

members-promote-admin = Promote to Admin
members-promote-moderator = Promote to Moderator
members-demote-member = Demote to Member
members-remove-confirm = Are you sure you want to remove this member?
members-ban-confirm = Are you sure you want to ban this user?

# =============================================================================
# JOIN REQUESTS
# =============================================================================

requests-title = Join Requests
requests-empty = No pending requests
requests-approve = Approve
requests-reject = Reject
requests-approve-all = Approve all
requests-approved = Request approved
requests-rejected = Request rejected

# =============================================================================
# COMMUNITY SETTINGS
# =============================================================================

community-settings-title = Community Settings
community-settings-general = General
community-settings-privacy = Privacy
community-settings-moderation = Moderation
community-settings-danger = Danger Zone

community-settings-name = Community name
community-settings-description = Description
community-settings-cover = Cover image
community-settings-icon = Icon

community-settings-visibility = Visibility
community-settings-join-approval = Join approval
community-settings-post-approval = Post approval

community-settings-delete = Delete community
community-settings-delete-warning = This action is irreversible. All community data will be deleted.
community-settings-delete-confirm = Type the community name to confirm

community-settings-transfer = Transfer ownership
community-settings-transfer-hint = Transfer community ownership to another member
