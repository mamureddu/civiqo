# Civiqo - Traduzioni Italiane
# File: errors.ftl - Messaggi di errore

# =============================================================================
# ERRORI HTTP
# =============================================================================

error-400 = Richiesta non valida
error-400-description = La richiesta non può essere elaborata. Verifica i dati inseriti.

error-401 = Non autorizzato
error-401-description = Devi effettuare l'accesso per visualizzare questa pagina.

error-403 = Accesso negato
error-403-description = Non hai i permessi per accedere a questa risorsa.

error-404 = Pagina non trovata
error-404-description = La pagina che stai cercando non esiste o è stata spostata.

error-500 = Errore del server
error-500-description = Si è verificato un errore interno. Riprova più tardi.

error-502 = Gateway non valido
error-502-description = Il server ha ricevuto una risposta non valida.

error-503 = Servizio non disponibile
error-503-description = Il servizio è temporaneamente non disponibile. Riprova tra qualche minuto.

error-back-home = Torna alla Home
error-try-again = Riprova
error-contact-support = Contatta il supporto

# =============================================================================
# ERRORI AUTENTICAZIONE
# =============================================================================

error-auth-invalid-credentials = Email o password non corretti
error-auth-email-exists = Questa email è già registrata
error-auth-weak-password = La password deve contenere almeno 8 caratteri
error-auth-session-expired = La tua sessione è scaduta. Effettua nuovamente l'accesso.
error-auth-account-locked = Account bloccato. Contatta il supporto.
error-auth-email-not-verified = Verifica la tua email prima di accedere.
error-auth-oauth-failed = Errore durante l'autenticazione. Riprova.

# =============================================================================
# ERRORI VALIDAZIONE
# =============================================================================

error-validation-required = Questo campo è obbligatorio
error-validation-email = Inserisci un indirizzo email valido
error-validation-min-length = Deve contenere almeno { $min } caratteri
error-validation-max-length = Deve contenere al massimo { $max } caratteri
error-validation-pattern = Formato non valido
error-validation-unique = Questo valore è già in uso
error-validation-mismatch = I valori non coincidono

# =============================================================================
# ERRORI COMMUNITY
# =============================================================================

error-community-not-found = Community non trovata
error-community-name-taken = Questo nome è già in uso
error-community-permission = Non hai i permessi per questa azione
error-community-already-member = Sei già membro di questa community
error-community-not-member = Non sei membro di questa community
error-community-owner-leave = Il proprietario non può abbandonare la community
error-community-last-admin = Devi nominare un altro admin prima di lasciare

# =============================================================================
# ERRORI POST
# =============================================================================

error-post-not-found = Post non trovato
error-post-permission = Non hai i permessi per modificare questo post
error-post-empty = Il contenuto del post non può essere vuoto
error-post-too-long = Il post supera la lunghezza massima consentita

# =============================================================================
# ERRORI COMMENTI
# =============================================================================

error-comment-not-found = Commento non trovato
error-comment-permission = Non hai i permessi per modificare questo commento
error-comment-empty = Il commento non può essere vuoto

# =============================================================================
# ERRORI GOVERNANCE
# =============================================================================

error-proposal-not-found = Proposta non trovata
error-proposal-closed = La votazione per questa proposta è chiusa
error-proposal-already-voted = Hai già votato per questa proposta
error-poll-not-found = Sondaggio non trovato
error-poll-closed = Questo sondaggio è chiuso

# =============================================================================
# ERRORI BUSINESS
# =============================================================================

error-business-not-found = Attività non trovata
error-business-permission = Non hai i permessi per modificare questa attività
error-review-already-exists = Hai già recensito questa attività
error-order-not-found = Ordine non trovato

# =============================================================================
# ERRORI CHAT
# =============================================================================

error-chat-room-not-found = Stanza chat non trovata
error-chat-permission = Non hai accesso a questa chat
error-chat-message-failed = Impossibile inviare il messaggio. Riprova.
error-chat-connection = Errore di connessione alla chat

# =============================================================================
# ERRORI FILE
# =============================================================================

error-file-too-large = Il file è troppo grande. Dimensione massima: { $max }
error-file-type = Tipo di file non supportato
error-file-upload = Errore durante il caricamento del file

# =============================================================================
# ERRORI RETE
# =============================================================================

error-network = Errore di rete. Verifica la tua connessione.
error-timeout = La richiesta ha impiegato troppo tempo. Riprova.
error-offline = Sei offline. Alcune funzionalità potrebbero non essere disponibili.

# =============================================================================
# ERRORI GENERICI
# =============================================================================

error-generic = Si è verificato un errore. Riprova più tardi.
error-unexpected = Errore imprevisto. Se il problema persiste, contatta il supporto.
error-maintenance = Stiamo effettuando manutenzione. Torna tra qualche minuto.
