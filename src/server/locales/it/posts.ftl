# Civiqo - Traduzioni Italiane
# File: posts.ftl - Post, commenti e interazioni

# =============================================================================
# POST
# =============================================================================

posts-title = Post
posts-create = Nuovo Post
posts-empty = Nessun post ancora
posts-empty-subtitle = Sii il primo a pubblicare qualcosa!

posts-filter-all = Tutti
posts-filter-recent = Recenti
posts-filter-popular = Popolari
posts-filter-following = Seguiti

# =============================================================================
# CARD POST
# =============================================================================

post-by = Pubblicato da { $name }
post-in = in { $community }
post-ago = { $time }
post-edited = (modificato)

post-likes = { $count ->
    [one] { $count } mi piace
   *[other] { $count } mi piace
}
post-comments = { $count ->
    [one] { $count } commento
   *[other] { $count } commenti
}
post-shares = { $count ->
    [one] { $count } condivisione
   *[other] { $count } condivisioni
}

# =============================================================================
# AZIONI POST
# =============================================================================

post-like = Mi piace
post-unlike = Non mi piace più
post-comment = Commenta
post-share = Condividi
post-save = Salva
post-unsave = Rimuovi dai salvati
post-report = Segnala
post-edit = Modifica
post-delete = Elimina

post-delete-confirm = Sei sicuro di voler eliminare questo post?
post-delete-success = Post eliminato
post-report-success = Post segnalato. Grazie per il feedback.

# =============================================================================
# CREA POST
# =============================================================================

post-create-title = Crea Post
post-create-community-label = Community
post-create-community-placeholder = Seleziona una community
post-create-title-label = Titolo
post-create-title-placeholder = Un titolo accattivante...
post-create-content-label = Contenuto
post-create-content-placeholder = Cosa vuoi condividere?
post-create-media = Aggiungi media
post-create-poll = Aggiungi sondaggio
post-create-submit = Pubblica
post-create-draft = Salva bozza

post-create-success = Post pubblicato!
post-create-error = Errore nella pubblicazione

# =============================================================================
# MODIFICA POST
# =============================================================================

post-edit-title = Modifica Post
post-edit-submit = Salva modifiche
post-edit-success = Post aggiornato!
post-edit-error = Errore nell'aggiornamento

# =============================================================================
# COMMENTI
# =============================================================================

comments-title = Commenti
comments-empty = Nessun commento ancora
comments-empty-subtitle = Sii il primo a commentare!
comments-load-more = Carica altri commenti
comments-show-replies = Mostra { $count ->
    [one] { $count } risposta
   *[other] { $count } risposte
}
comments-hide-replies = Nascondi risposte

comment-placeholder = Scrivi un commento...
comment-submit = Commenta
comment-reply = Rispondi
comment-edit = Modifica
comment-delete = Elimina
comment-report = Segnala
comment-like = Mi piace

comment-edited = (modificato)
comment-deleted = Commento eliminato
comment-by = { $name }
comment-ago = { $time }

comment-delete-confirm = Sei sicuro di voler eliminare questo commento?
comment-delete-success = Commento eliminato
comment-report-success = Commento segnalato

# =============================================================================
# CONDIVISIONE
# =============================================================================

share-title = Condividi
share-copy-link = Copia link
share-link-copied = Link copiato!
share-facebook = Condividi su Facebook
share-twitter = Condividi su Twitter
share-whatsapp = Condividi su WhatsApp
share-email = Invia via email

# =============================================================================
# SEGNALAZIONE
# =============================================================================

report-title = Segnala contenuto
report-reason-label = Motivo della segnalazione
report-reason-spam = Spam
report-reason-harassment = Molestie
report-reason-hate = Contenuto d'odio
report-reason-violence = Violenza
report-reason-misinformation = Disinformazione
report-reason-other = Altro
report-details-label = Dettagli (opzionale)
report-details-placeholder = Fornisci ulteriori dettagli...
report-submit = Invia segnalazione
report-success = Grazie per la segnalazione. La esamineremo al più presto.
