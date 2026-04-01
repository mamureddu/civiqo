# Civiqo - Traduzioni Italiane
# File: communities.ftl - Community e membership

# =============================================================================
# LISTA COMMUNITY
# =============================================================================

communities-title = Community
communities-subtitle = Scopri e unisciti alle community della tua zona
communities-search-placeholder = Cerca community...
communities-filter-all = Tutte
communities-filter-public = Pubbliche
communities-filter-private = Private
communities-filter-my = Le mie
communities-sort-recent = Più recenti
communities-sort-popular = Più popolari
communities-sort-name = Nome A-Z
communities-empty = Nessuna community trovata
communities-empty-subtitle = Crea la prima community della tua zona!

# =============================================================================
# CARD COMMUNITY
# =============================================================================

community-members-label = Membri
community-members = { $count ->
    [one] { $count } membro
   *[other] { $count } membri
}
community-posts = { $count ->
    [one] { $count } post
   *[other] { $count } post
}
community-public = Pubblica
community-private = Privata
community-verified = Verificata
community-created-by = Creata da { $name }
community-created-at = Creata il { $date }

# =============================================================================
# CREA COMMUNITY
# =============================================================================

community-create-title = Crea una Nuova Community
community-create-subtitle = Costruisci uno spazio per la tua comunità locale dove connettersi e collaborare

community-create-name-label = Nome Community
community-create-name-placeholder = es. Associazione di Quartiere Centro
community-create-name-hint = Questo verrà usato per generare un URL unico per la tua community
community-create-name-validation = Deve essere di 3-255 caratteri e contenere almeno una lettera o numero

community-create-description-label = Descrizione
community-create-description-placeholder = Descrivi lo scopo della tua community, gli obiettivi e chi dovrebbe unirsi...
community-create-description-hint = Opzionale: Aiuta le persone a capire di cosa tratta la tua community
community-create-description-max = Massimo 2000 caratteri

community-create-privacy-title = Impostazioni Privacy
community-create-public-label = Community Pubblica
community-create-public-hint = Chiunque può trovare e unirsi a questa community
community-create-approval-label = Richiedi Approvazione
community-create-approval-hint = I nuovi membri devono essere approvati da un admin

community-create-submit = Crea Community
community-create-cancel = Annulla
community-create-creating = Creazione...

community-create-success = Community creata con successo!
community-create-error = Errore nella creazione della community
community-create-redirect = Reindirizzamento alla dashboard...

community-create-guidelines-title = Linee Guida Community
community-create-guideline-1 = Le community devono essere inclusive e accoglienti
community-create-guideline-2 = Concentrati sull'engagement locale e la collaborazione
community-create-guideline-3 = Diventerai automaticamente admin della community
community-create-guideline-4 = Le impostazioni possono essere modificate in seguito

# Validazione
community-name-required = Il nome della community è obbligatorio e non può essere vuoto
community-name-min = Il nome deve essere di almeno 3 caratteri
community-name-max = Il nome è troppo lungo (massimo 255 caratteri)
community-name-invalid = Il nome deve contenere almeno una lettera o numero
community-description-max = La descrizione è troppo lunga (massimo 2000 caratteri)

# =============================================================================
# DETTAGLIO COMMUNITY
# =============================================================================

community-detail-about = Informazioni
community-detail-members = Membri
community-detail-posts = Post
community-detail-events = Eventi
community-detail-governance = Governance
community-detail-settings = Impostazioni

community-about-description = Descrizione
community-about-location = Località
community-about-created = Data creazione
community-about-admin = Amministratore

# =============================================================================
# MEMBERSHIP
# =============================================================================

community-join = Unisciti
community-leave = Abbandona
community-request-join = Richiedi di unirti
community-pending = Richiesta in attesa
community-joined = Sei membro

community-join-success = Ti sei unito alla community!
community-leave-success = Hai abbandonato la community
community-leave-confirm = Sei sicuro di voler abbandonare questa community?
community-request-sent = Richiesta inviata! Attendi l'approvazione.

# =============================================================================
# GESTIONE MEMBRI
# =============================================================================

members-title = Membri
members-search = Cerca membri...
members-role-owner = Proprietario
members-role-admin = Admin
members-role-moderator = Moderatore
members-role-member = Membro
members-joined = Iscritto dal { $date }

members-promote = Promuovi
members-demote = Declassa
members-remove = Rimuovi
members-ban = Blocca

members-promote-admin = Promuovi ad Admin
members-promote-moderator = Promuovi a Moderatore
members-demote-member = Declassa a Membro
members-remove-confirm = Sei sicuro di voler rimuovere questo membro?
members-ban-confirm = Sei sicuro di voler bloccare questo utente?

# =============================================================================
# RICHIESTE DI ADESIONE
# =============================================================================

requests-title = Richieste di adesione
requests-empty = Nessuna richiesta in attesa
requests-approve = Approva
requests-reject = Rifiuta
requests-approve-all = Approva tutte
requests-approved = Richiesta approvata
requests-rejected = Richiesta rifiutata

# =============================================================================
# IMPOSTAZIONI COMMUNITY
# =============================================================================

community-settings-title = Impostazioni Community
community-settings-general = Generali
community-settings-privacy = Privacy
community-settings-moderation = Moderazione
community-settings-danger = Zona pericolosa

community-settings-name = Nome community
community-settings-description = Descrizione
community-settings-cover = Immagine di copertina
community-settings-icon = Icona

community-settings-visibility = Visibilità
community-settings-join-approval = Approvazione iscrizioni
community-settings-post-approval = Approvazione post

community-settings-delete = Elimina community
community-settings-delete-warning = Questa azione è irreversibile. Tutti i dati della community verranno eliminati.
community-settings-delete-confirm = Scrivi il nome della community per confermare

community-settings-transfer = Trasferisci proprietà
community-settings-transfer-hint = Trasferisci la proprietà della community a un altro membro
