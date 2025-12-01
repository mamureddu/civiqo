# Civiqo - English Translations
# File: dashboard.ftl - User dashboard

# =============================================================================
# DASHBOARD
# =============================================================================

dashboard-title = Dashboard
dashboard-welcome = Welcome back, { $name }!
dashboard-subtitle = Here's what's happening in your communities

# =============================================================================
# STATISTICS
# =============================================================================

dashboard-stats-communities = Your Communities
dashboard-stats-posts = Your Posts
dashboard-stats-notifications = Notifications
dashboard-stats-activity = Recent Activity

# =============================================================================
# SECTIONS
# =============================================================================

dashboard-section-communities = Your Communities
dashboard-section-communities-empty = You're not a member of any community yet
dashboard-section-communities-explore = Explore communities
dashboard-section-communities-create = Create your first community

dashboard-section-activity = Recent Activity
dashboard-section-activity-empty = No recent activity
dashboard-section-activity-view-all = View all activity

dashboard-section-notifications = Notifications
dashboard-section-notifications-empty = No new notifications
dashboard-section-notifications-view-all = View all notifications
dashboard-section-notifications-mark-read = Mark as read

# =============================================================================
# QUICK ACTIONS
# =============================================================================

dashboard-quick-actions = Quick Actions
dashboard-action-create-community = Create Community
dashboard-action-create-post = New Post
dashboard-action-create-proposal = New Proposal
dashboard-action-explore = Explore

# =============================================================================
# COMMUNITY CARD (Dashboard)
# =============================================================================

dashboard-community-members = { $count ->
    [one] { $count } member
   *[other] { $count } members
}
dashboard-community-new-posts = { $count ->
    [one] { $count } new post
   *[other] { $count } new posts
}
dashboard-community-view = View
dashboard-community-settings = Settings

# =============================================================================
# ACTIVITY
# =============================================================================

activity-post-created = You posted in { $community }
activity-comment-added = You commented in { $community }
activity-joined-community = You joined { $community }
activity-proposal-created = You created a proposal in { $community }
activity-vote-cast = You voted in { $community }

# =============================================================================
# NOTIFICATIONS
# =============================================================================

notification-new-member = { $name } joined { $community }
notification-new-post = New post in { $community }
notification-new-comment = { $name } commented on your post
notification-mention = { $name } mentioned you
notification-proposal-approved = Your proposal was approved
notification-proposal-rejected = Your proposal was rejected
notification-role-changed = Your role in { $community } has changed

# =============================================================================
# ONBOARDING
# =============================================================================

onboarding-welcome = Welcome to Civiqo!
onboarding-step-1-title = Complete your profile
onboarding-step-1-description = Add a photo and a short bio
onboarding-step-2-title = Join a community
onboarding-step-2-description = Find and join communities in your area
onboarding-step-3-title = Participate
onboarding-step-3-description = Post, comment and vote in your communities
onboarding-skip = Skip for now
onboarding-next = Next
onboarding-finish = Get Started!

# =============================================================================
# ADMIN
# =============================================================================

admin-title = Admin Dashboard
admin-moderation = Moderation
admin-analytics = Analytics
admin-audit = Audit Log
admin-moderation-queue = Moderation Queue
admin-analytics-summary = Recent Events
admin-audit-logs = Activity Log
