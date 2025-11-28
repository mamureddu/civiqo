# Civiqo - Traduzioni Italiane
# File: main.ftl - Stringhe generali dell'applicazione

# =============================================================================
# NAVIGAZIONE E LAYOUT
# =============================================================================

nav-home = Home
nav-communities = Community
nav-governance = Governance
nav-businesses = Attività
nav-chat = Chat
nav-profile = Profilo
nav-search = Cerca
nav-notifications = Notifiche

# Header
header-login = Accedi
header-logout = Esci
header-register = Registrati
header-welcome = Benvenuto, { $name }

# Footer
footer-copyright = © { $year } Civiqo. Tutti i diritti riservati.
footer-privacy = Privacy
footer-terms = Termini di Servizio
footer-contact = Contatti
footer-about = Chi Siamo

# =============================================================================
# AZIONI COMUNI
# =============================================================================

action-save = Salva
action-cancel = Annulla
action-delete = Elimina
action-edit = Modifica
action-create = Crea
action-submit = Invia
action-confirm = Conferma
action-back = Indietro
action-next = Avanti
action-close = Chiudi
action-search = Cerca
action-filter = Filtra
action-sort = Ordina
action-refresh = Aggiorna
action-load-more = Carica altri
action-view-all = Vedi tutti
action-share = Condividi
action-copy = Copia
action-download = Scarica

# =============================================================================
# STATI UI
# =============================================================================

state-loading = Caricamento...
state-saving = Salvataggio...
state-deleting = Eliminazione...
state-empty = Nessun elemento trovato
state-error = Si è verificato un errore
state-success = Operazione completata con successo
state-no-results = Nessun risultato trovato
state-offline = Sei offline

# =============================================================================
# MESSAGGI GENERICI
# =============================================================================

message-confirm-delete = Sei sicuro di voler eliminare questo elemento?
message-unsaved-changes = Hai modifiche non salvate. Vuoi uscire?
message-session-expired = La tua sessione è scaduta. Effettua nuovamente l'accesso.
message-unauthorized = Non hai i permessi per questa azione
message-not-found = Pagina non trovata

# =============================================================================
# FORMATTAZIONE
# =============================================================================

format-date = { $date }
format-time = { $time }
format-datetime = { $date } alle { $time }
format-relative-now = adesso
format-relative-minutes = { $count ->
    [one] { $count } minuto fa
   *[other] { $count } minuti fa
}
format-relative-hours = { $count ->
    [one] { $count } ora fa
   *[other] { $count } ore fa
}
format-relative-days = { $count ->
    [one] { $count } giorno fa
   *[other] { $count } giorni fa
}

# =============================================================================
# VALIDAZIONE
# =============================================================================

validation-required = Questo campo è obbligatorio
validation-email = Inserisci un indirizzo email valido
validation-min-length = Deve contenere almeno { $min } caratteri
validation-max-length = Deve contenere al massimo { $max } caratteri
validation-invalid-format = Formato non valido
validation-passwords-mismatch = Le password non coincidono

# =============================================================================
# ACCESSIBILITÀ
# =============================================================================

a11y-menu-toggle = Apri/chiudi menu
a11y-language-select = Seleziona lingua
a11y-close-modal = Chiudi finestra
a11y-expand = Espandi
a11y-collapse = Comprimi
a11y-required-field = Campo obbligatorio
