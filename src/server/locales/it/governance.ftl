# Civiqo - Traduzioni Italiane
# File: governance.ftl - Proposte, votazioni e decisioni

# =============================================================================
# GOVERNANCE GENERALE
# =============================================================================

governance-title = Governance
governance-subtitle = Partecipa alle decisioni della tua community
governance-tab-proposals = Proposte
governance-tab-decisions = Decisioni
governance-tab-polls = Sondaggi

# =============================================================================
# PROPOSTE
# =============================================================================

proposals-title = Proposte
proposals-create = Nuova Proposta
proposals-empty = Nessuna proposta attiva
proposals-empty-subtitle = Crea la prima proposta per la tua community

proposals-filter-all = Tutte
proposals-filter-active = Attive
proposals-filter-approved = Approvate
proposals-filter-rejected = Respinte
proposals-filter-pending = In attesa

proposal-status-draft = Bozza
proposal-status-active = Attiva
proposal-status-voting = In votazione
proposal-status-approved = Approvata
proposal-status-rejected = Respinta
proposal-status-implemented = Implementata

# Card proposta
proposal-by = Proposta da { $name }
proposal-created = Creata il { $date }
proposal-deadline = Scadenza: { $date }
proposal-votes = { $count ->
    [one] { $count } voto
   *[other] { $count } voti
}
proposal-comments = { $count ->
    [one] { $count } commento
   *[other] { $count } commenti
}

# =============================================================================
# CREA PROPOSTA
# =============================================================================

proposal-create-title = Crea Proposta
proposal-create-subtitle = Proponi un'idea o un cambiamento alla community

proposal-create-title-label = Titolo
proposal-create-title-placeholder = Un titolo chiaro e conciso
proposal-create-description-label = Descrizione
proposal-create-description-placeholder = Descrivi la tua proposta in dettaglio...
proposal-create-category-label = Categoria
proposal-create-deadline-label = Scadenza votazione
proposal-create-submit = Pubblica Proposta
proposal-create-save-draft = Salva come bozza

proposal-category-general = Generale
proposal-category-infrastructure = Infrastrutture
proposal-category-events = Eventi
proposal-category-rules = Regolamento
proposal-category-budget = Budget
proposal-category-other = Altro

proposal-create-success = Proposta creata con successo!
proposal-create-error = Errore nella creazione della proposta

# =============================================================================
# DETTAGLIO PROPOSTA
# =============================================================================

proposal-detail-description = Descrizione
proposal-detail-discussion = Discussione
proposal-detail-votes = Votazione
proposal-detail-history = Cronologia

proposal-vote-for = A favore
proposal-vote-against = Contrario
proposal-vote-abstain = Astenuto
proposal-vote-submit = Vota
proposal-vote-change = Cambia voto
proposal-vote-success = Voto registrato!
proposal-vote-already = Hai già votato per questa proposta

proposal-comment-placeholder = Aggiungi un commento...
proposal-comment-submit = Commenta
proposal-comment-reply = Rispondi

# =============================================================================
# DECISIONI
# =============================================================================

decisions-title = Decisioni
decisions-empty = Nessuna decisione registrata
decisions-filter-all = Tutte
decisions-filter-recent = Recenti
decisions-filter-important = Importanti

decision-status-pending = In attesa
decision-status-approved = Approvata
decision-status-rejected = Respinta
decision-status-implemented = Implementata

decision-made-on = Decisione del { $date }
decision-participants = { $count ->
    [one] { $count } partecipante
   *[other] { $count } partecipanti
}

# =============================================================================
# SONDAGGI
# =============================================================================

polls-title = Sondaggi
polls-create = Nuovo Sondaggio
polls-empty = Nessun sondaggio attivo

poll-status-active = Attivo
poll-status-closed = Chiuso
poll-status-draft = Bozza

poll-votes = { $count ->
    [one] { $count } voto
   *[other] { $count } voti
}
poll-ends = Termina il { $date }
poll-ended = Terminato il { $date }

# Crea sondaggio
poll-create-title = Crea Sondaggio
poll-create-question-label = Domanda
poll-create-question-placeholder = La tua domanda...
poll-create-options-label = Opzioni
poll-create-option-placeholder = Opzione { $number }
poll-create-add-option = Aggiungi opzione
poll-create-multiple = Consenti risposte multiple
poll-create-anonymous = Voti anonimi
poll-create-deadline-label = Scadenza
poll-create-submit = Crea Sondaggio

poll-vote-submit = Vota
poll-vote-success = Voto registrato!
poll-results = Risultati
poll-your-vote = Il tuo voto
