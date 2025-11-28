# Civiqo - English Translations
# File: posts.ftl - Posts, comments and interactions

# =============================================================================
# POSTS
# =============================================================================

posts-title = Posts
posts-create = New Post
posts-empty = No posts yet
posts-empty-subtitle = Be the first to post something!

posts-filter-all = All
posts-filter-recent = Recent
posts-filter-popular = Popular
posts-filter-following = Following

# =============================================================================
# POST CARD
# =============================================================================

post-by = Posted by { $name }
post-in = in { $community }
post-ago = { $time }
post-edited = (edited)

post-likes = { $count ->
    [one] { $count } like
   *[other] { $count } likes
}
post-comments = { $count ->
    [one] { $count } comment
   *[other] { $count } comments
}
post-shares = { $count ->
    [one] { $count } share
   *[other] { $count } shares
}

# =============================================================================
# POST ACTIONS
# =============================================================================

post-like = Like
post-unlike = Unlike
post-comment = Comment
post-share = Share
post-save = Save
post-unsave = Unsave
post-report = Report
post-edit = Edit
post-delete = Delete

post-delete-confirm = Are you sure you want to delete this post?
post-delete-success = Post deleted
post-report-success = Post reported. Thank you for your feedback.

# =============================================================================
# CREATE POST
# =============================================================================

post-create-title = Create Post
post-create-community-label = Community
post-create-community-placeholder = Select a community
post-create-title-label = Title
post-create-title-placeholder = A catchy title...
post-create-content-label = Content
post-create-content-placeholder = What do you want to share?
post-create-media = Add media
post-create-poll = Add poll
post-create-submit = Publish
post-create-draft = Save draft

post-create-success = Post published!
post-create-error = Error publishing post

# =============================================================================
# EDIT POST
# =============================================================================

post-edit-title = Edit Post
post-edit-submit = Save changes
post-edit-success = Post updated!
post-edit-error = Error updating post

# =============================================================================
# COMMENTS
# =============================================================================

comments-title = Comments
comments-empty = No comments yet
comments-empty-subtitle = Be the first to comment!
comments-load-more = Load more comments
comments-show-replies = Show { $count ->
    [one] { $count } reply
   *[other] { $count } replies
}
comments-hide-replies = Hide replies

comment-placeholder = Write a comment...
comment-submit = Comment
comment-reply = Reply
comment-edit = Edit
comment-delete = Delete
comment-report = Report
comment-like = Like

comment-edited = (edited)
comment-deleted = Comment deleted
comment-by = { $name }
comment-ago = { $time }

comment-delete-confirm = Are you sure you want to delete this comment?
comment-delete-success = Comment deleted
comment-report-success = Comment reported

# =============================================================================
# SHARING
# =============================================================================

share-title = Share
share-copy-link = Copy link
share-link-copied = Link copied!
share-facebook = Share on Facebook
share-twitter = Share on Twitter
share-whatsapp = Share on WhatsApp
share-email = Send via email

# =============================================================================
# REPORTING
# =============================================================================

report-title = Report content
report-reason-label = Reason for reporting
report-reason-spam = Spam
report-reason-harassment = Harassment
report-reason-hate = Hate speech
report-reason-violence = Violence
report-reason-misinformation = Misinformation
report-reason-other = Other
report-details-label = Details (optional)
report-details-placeholder = Provide additional details...
report-submit = Submit report
report-success = Thank you for reporting. We'll review it shortly.
