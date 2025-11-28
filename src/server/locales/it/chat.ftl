# Civiqo - Traduzioni Italiane
# File: chat.ftl - Chat e messaggistica

# =============================================================================
# CHAT GENERALE
# =============================================================================

chat-title = Chat
chat-subtitle = Messaggi delle tue community
chat-empty = Nessuna chat attiva
chat-empty-subtitle = Unisciti a una community per iniziare a chattare

# =============================================================================
# LISTA CHAT
# =============================================================================

chat-rooms = Stanze
chat-direct = Messaggi diretti
chat-search-placeholder = Cerca conversazioni...

chat-room-members = { $count ->
    [one] { $count } membro
   *[other] { $count } membri
}
chat-room-online = { $count ->
    [one] { $count } online
   *[other] { $count } online
}
chat-last-message = Ultimo messaggio: { $time }
chat-unread = { $count ->
    [one] { $count } non letto
   *[other] { $count } non letti
}

# =============================================================================
# STANZA CHAT
# =============================================================================

chat-room-title = { $name }
chat-room-info = Info stanza
chat-room-members-list = Membri
chat-room-settings = Impostazioni
chat-room-leave = Abbandona stanza
chat-room-mute = Silenzia
chat-room-unmute = Riattiva notifiche

# =============================================================================
# MESSAGGI
# =============================================================================

chat-input-placeholder = Scrivi un messaggio...
chat-send = Invia
chat-typing = { $name } sta scrivendo...
chat-typing-multiple = { $count } persone stanno scrivendo...

message-edited = (modificato)
message-deleted = Messaggio eliminato
message-edit = Modifica
message-delete = Elimina
message-reply = Rispondi
message-copy = Copia
message-forward = Inoltra
message-pin = Fissa
message-unpin = Rimuovi fissato

message-delete-confirm = Sei sicuro di voler eliminare questo messaggio?
message-delete-success = Messaggio eliminato

# =============================================================================
# MESSAGGI DIRETTI
# =============================================================================

dm-new = Nuovo messaggio
dm-to = A:
dm-search-users = Cerca utenti...
dm-start-conversation = Inizia conversazione

# =============================================================================
# CREA STANZA
# =============================================================================

chat-create-room = Crea stanza
chat-create-name-label = Nome stanza
chat-create-name-placeholder = Nome della stanza...
chat-create-description-label = Descrizione
chat-create-description-placeholder = Descrizione opzionale...
chat-create-private = Stanza privata
chat-create-private-hint = Solo i membri invitati possono accedere
chat-create-submit = Crea stanza
chat-create-success = Stanza creata!

# =============================================================================
# STATI
# =============================================================================

chat-status-online = Online
chat-status-offline = Offline
chat-status-away = Assente
chat-status-busy = Occupato

chat-connection-lost = Connessione persa
chat-connection-restored = Connessione ripristinata
chat-reconnecting = Riconnessione in corso...

# =============================================================================
# NOTIFICHE CHAT
# =============================================================================

chat-notification-new-message = Nuovo messaggio da { $name }
chat-notification-mention = { $name } ti ha menzionato in { $room }
chat-notification-added = Sei stato aggiunto a { $room }
