# Civiqo - English Translations
# File: errors.ftl - Error messages

# =============================================================================
# HTTP ERRORS
# =============================================================================

error-400 = Bad Request
error-400-description = The request could not be processed. Please check your input.

error-401 = Unauthorized
error-401-description = You must be logged in to view this page.

error-403 = Access Denied
error-403-description = You don't have permission to access this resource.

error-404 = Page Not Found
error-404-description = The page you're looking for doesn't exist or has been moved.

error-500 = Server Error
error-500-description = An internal error occurred. Please try again later.

error-502 = Bad Gateway
error-502-description = The server received an invalid response.

error-503 = Service Unavailable
error-503-description = The service is temporarily unavailable. Please try again in a few minutes.

error-back-home = Back to Home
error-try-again = Try Again
error-contact-support = Contact Support

# =============================================================================
# AUTHENTICATION ERRORS
# =============================================================================

error-auth-invalid-credentials = Invalid email or password
error-auth-email-exists = This email is already registered
error-auth-weak-password = Password must be at least 8 characters
error-auth-session-expired = Your session has expired. Please log in again.
error-auth-account-locked = Account locked. Please contact support.
error-auth-email-not-verified = Please verify your email before signing in.
error-auth-oauth-failed = Authentication error. Please try again.

# =============================================================================
# VALIDATION ERRORS
# =============================================================================

error-validation-required = This field is required
error-validation-email = Please enter a valid email address
error-validation-min-length = Must be at least { $min } characters
error-validation-max-length = Must be at most { $max } characters
error-validation-pattern = Invalid format
error-validation-unique = This value is already in use
error-validation-mismatch = Values do not match

# =============================================================================
# COMMUNITY ERRORS
# =============================================================================

error-community-not-found = Community not found
error-community-name-taken = This name is already taken
error-community-permission = You don't have permission for this action
error-community-already-member = You're already a member of this community
error-community-not-member = You're not a member of this community
error-community-owner-leave = The owner cannot leave the community
error-community-last-admin = You must appoint another admin before leaving

# =============================================================================
# POST ERRORS
# =============================================================================

error-post-not-found = Post not found
error-post-permission = You don't have permission to edit this post
error-post-empty = Post content cannot be empty
error-post-too-long = Post exceeds maximum length

# =============================================================================
# COMMENT ERRORS
# =============================================================================

error-comment-not-found = Comment not found
error-comment-permission = You don't have permission to edit this comment
error-comment-empty = Comment cannot be empty

# =============================================================================
# GOVERNANCE ERRORS
# =============================================================================

error-proposal-not-found = Proposal not found
error-proposal-closed = Voting for this proposal is closed
error-proposal-already-voted = You've already voted on this proposal
error-poll-not-found = Poll not found
error-poll-closed = This poll is closed

# =============================================================================
# BUSINESS ERRORS
# =============================================================================

error-business-not-found = Business not found
error-business-permission = You don't have permission to edit this business
error-review-already-exists = You've already reviewed this business
error-order-not-found = Order not found

# =============================================================================
# CHAT ERRORS
# =============================================================================

error-chat-room-not-found = Chat room not found
error-chat-permission = You don't have access to this chat
error-chat-message-failed = Unable to send message. Please try again.
error-chat-connection = Chat connection error

# =============================================================================
# FILE ERRORS
# =============================================================================

error-file-too-large = File is too large. Maximum size: { $max }
error-file-type = File type not supported
error-file-upload = Error uploading file

# =============================================================================
# NETWORK ERRORS
# =============================================================================

error-network = Network error. Please check your connection.
error-timeout = Request timed out. Please try again.
error-offline = You're offline. Some features may not be available.

# =============================================================================
# GENERIC ERRORS
# =============================================================================

error-generic = An error occurred. Please try again later.
error-unexpected = Unexpected error. If the problem persists, please contact support.
error-maintenance = We're performing maintenance. Please check back in a few minutes.
