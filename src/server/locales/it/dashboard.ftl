# Civiqo - Traduzioni Italiane
# File: dashboard.ftl - Dashboard utente

# =============================================================================
# DASHBOARD
# =============================================================================

dashboard-title = Dashboard
dashboard-welcome = Bentornato, { $name }!
dashboard-subtitle = Ecco cosa succede nelle tue community

# =============================================================================
# STATISTICHE
# =============================================================================

dashboard-stats-communities = Le tue Community
dashboard-stats-posts = I tuoi Post
dashboard-stats-notifications = Notifiche
dashboard-stats-activity = Attività recente

# =============================================================================
# SEZIONI
# =============================================================================

dashboard-section-communities = Le tue Community
dashboard-section-communities-empty = Non sei ancora membro di nessuna community
dashboard-section-communities-explore = Esplora le community
dashboard-section-communities-create = Crea la tua prima community

dashboard-section-activity = Attività Recente
dashboard-section-activity-empty = Nessuna attività recente
dashboard-section-activity-view-all = Vedi tutta l'attività

dashboard-section-notifications = Notifiche
dashboard-section-notifications-empty = Nessuna nuova notifica
dashboard-section-notifications-view-all = Vedi tutte le notifiche
dashboard-section-notifications-mark-read = Segna come lette

# =============================================================================
# AZIONI RAPIDE
# =============================================================================

dashboard-quick-actions = Azioni rapide
dashboard-action-create-community = Crea Community
dashboard-action-create-post = Nuovo Post
dashboard-action-create-proposal = Nuova Proposta
dashboard-action-explore = Esplora

# =============================================================================
# CARD COMMUNITY (Dashboard)
# =============================================================================

dashboard-community-members = { $count ->
    [one] { $count } membro
   *[other] { $count } membri
}
dashboard-community-new-posts = { $count ->
    [one] { $count } nuovo post
   *[other] { $count } nuovi post
}
dashboard-community-view = Visualizza
dashboard-community-settings = Impostazioni

# =============================================================================
# ATTIVITÀ
# =============================================================================

activity-post-created = Hai pubblicato un post in { $community }
activity-comment-added = Hai commentato in { $community }
activity-joined-community = Ti sei unito a { $community }
activity-proposal-created = Hai creato una proposta in { $community }
activity-vote-cast = Hai votato in { $community }

# =============================================================================
# NOTIFICHE
# =============================================================================

notification-new-member = { $name } si è unito a { $community }
notification-new-post = Nuovo post in { $community }
notification-new-comment = { $name } ha commentato il tuo post
notification-mention = { $name } ti ha menzionato
notification-proposal-approved = La tua proposta è stata approvata
notification-proposal-rejected = La tua proposta è stata respinta
notification-role-changed = Il tuo ruolo in { $community } è cambiato

# =============================================================================
# ONBOARDING
# =============================================================================

onboarding-welcome = Benvenuto su Civiqo!
onboarding-step-1-title = Completa il tuo profilo
onboarding-step-1-description = Aggiungi una foto e una breve bio
onboarding-step-2-title = Unisciti a una community
onboarding-step-2-description = Trova e unisciti alle community della tua zona
onboarding-step-3-title = Partecipa
onboarding-step-3-description = Pubblica, commenta e vota nelle tue community
onboarding-skip = Salta per ora
onboarding-next = Avanti
onboarding-finish = Inizia!
