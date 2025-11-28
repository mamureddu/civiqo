# Civiqo - English Translations
# File: chat.ftl - Chat and messaging

# =============================================================================
# GENERAL CHAT
# =============================================================================

chat-title = Chat
chat-subtitle = Messages from your communities
chat-empty = No active chats
chat-empty-subtitle = Join a community to start chatting

# =============================================================================
# CHAT LIST
# =============================================================================

chat-rooms = Rooms
chat-direct = Direct Messages
chat-search-placeholder = Search conversations...

chat-room-members = { $count ->
    [one] { $count } member
   *[other] { $count } members
}
chat-room-online = { $count ->
    [one] { $count } online
   *[other] { $count } online
}
chat-last-message = Last message: { $time }
chat-unread = { $count ->
    [one] { $count } unread
   *[other] { $count } unread
}

# =============================================================================
# CHAT ROOM
# =============================================================================

chat-room-title = { $name }
chat-room-info = Room info
chat-room-members-list = Members
chat-room-settings = Settings
chat-room-leave = Leave room
chat-room-mute = Mute
chat-room-unmute = Unmute

# =============================================================================
# MESSAGES
# =============================================================================

chat-input-placeholder = Type a message...
chat-send = Send
chat-typing = { $name } is typing...
chat-typing-multiple = { $count } people are typing...

message-edited = (edited)
message-deleted = Message deleted
message-edit = Edit
message-delete = Delete
message-reply = Reply
message-copy = Copy
message-forward = Forward
message-pin = Pin
message-unpin = Unpin

message-delete-confirm = Are you sure you want to delete this message?
message-delete-success = Message deleted

# =============================================================================
# DIRECT MESSAGES
# =============================================================================

dm-new = New message
dm-to = To:
dm-search-users = Search users...
dm-start-conversation = Start conversation

# =============================================================================
# CREATE ROOM
# =============================================================================

chat-create-room = Create room
chat-create-name-label = Room name
chat-create-name-placeholder = Room name...
chat-create-description-label = Description
chat-create-description-placeholder = Optional description...
chat-create-private = Private room
chat-create-private-hint = Only invited members can access
chat-create-submit = Create room
chat-create-success = Room created!

# =============================================================================
# STATUS
# =============================================================================

chat-status-online = Online
chat-status-offline = Offline
chat-status-away = Away
chat-status-busy = Busy

chat-connection-lost = Connection lost
chat-connection-restored = Connection restored
chat-reconnecting = Reconnecting...

# =============================================================================
# CHAT NOTIFICATIONS
# =============================================================================

chat-notification-new-message = New message from { $name }
chat-notification-mention = { $name } mentioned you in { $room }
chat-notification-added = You were added to { $room }
