# Civiqo - English Translations
# File: main.ftl - General application strings

# =============================================================================
# NAVIGATION AND LAYOUT
# =============================================================================

nav-home = Home
nav-communities = Communities
nav-governance = Governance
nav-businesses = Businesses
nav-chat = Chat
nav-profile = Profile
nav-search = Search
nav-notifications = Notifications

# Header
header-login = Login
header-logout = Logout
header-register = Sign Up
header-welcome = Welcome, { $name }

# Footer
footer-copyright = © { $year } Civiqo. All rights reserved.
footer-privacy = Privacy
footer-terms = Terms of Service
footer-contact = Contact
footer-about = About Us

# =============================================================================
# COMMON ACTIONS
# =============================================================================

action-save = Save
action-cancel = Cancel
action-delete = Delete
action-edit = Edit
action-create = Create
action-submit = Submit
action-confirm = Confirm
action-back = Back
action-next = Next
action-close = Close
action-search = Search
action-filter = Filter
action-sort = Sort
action-refresh = Refresh
action-load-more = Load more
action-view-all = View all
action-share = Share
action-copy = Copy
action-download = Download

# =============================================================================
# UI STATES
# =============================================================================

state-loading = Loading...
state-saving = Saving...
state-deleting = Deleting...
state-empty = No items found
state-error = An error occurred
state-success = Operation completed successfully
state-no-results = No results found
state-offline = You are offline

# =============================================================================
# GENERIC MESSAGES
# =============================================================================

message-confirm-delete = Are you sure you want to delete this item?
message-unsaved-changes = You have unsaved changes. Do you want to leave?
message-session-expired = Your session has expired. Please log in again.
message-unauthorized = You don't have permission for this action
message-not-found = Page not found

# =============================================================================
# FORMATTING
# =============================================================================

format-date = { $date }
format-time = { $time }
format-datetime = { $date } at { $time }
format-relative-now = just now
format-relative-minutes = { $count ->
    [one] { $count } minute ago
   *[other] { $count } minutes ago
}
format-relative-hours = { $count ->
    [one] { $count } hour ago
   *[other] { $count } hours ago
}
format-relative-days = { $count ->
    [one] { $count } day ago
   *[other] { $count } days ago
}

# =============================================================================
# VALIDATION
# =============================================================================

validation-required = This field is required
validation-email = Please enter a valid email address
validation-min-length = Must be at least { $min } characters
validation-max-length = Must be at most { $max } characters
validation-invalid-format = Invalid format
validation-passwords-mismatch = Passwords do not match

# =============================================================================
# ACCESSIBILITY
# =============================================================================

a11y-menu-toggle = Toggle menu
a11y-language-select = Select language
a11y-close-modal = Close dialog
a11y-expand = Expand
a11y-collapse = Collapse
a11y-required-field = Required field
