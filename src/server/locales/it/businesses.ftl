# Civiqo - Traduzioni Italiane
# File: businesses.ftl - Attività commerciali locali

# =============================================================================
# LISTA ATTIVITÀ
# =============================================================================

businesses-title = Attività Locali
businesses-subtitle = Scopri le attività della tua zona
businesses-search-placeholder = Cerca attività...
businesses-filter-all = Tutte
businesses-filter-verified = Verificate
businesses-filter-category = Categoria
businesses-sort-rating = Valutazione
businesses-sort-recent = Più recenti
businesses-sort-name = Nome A-Z
businesses-empty = Nessuna attività trovata
businesses-empty-subtitle = Sii il primo a registrare la tua attività!

# =============================================================================
# CARD ATTIVITÀ
# =============================================================================

business-verified = Verificata
business-rating = { $rating } stelle
business-reviews = { $count ->
    [one] { $count } recensione
   *[other] { $count } recensioni
}
business-category-restaurant = Ristorante
business-category-shop = Negozio
business-category-service = Servizi
business-category-health = Salute
business-category-education = Istruzione
business-category-entertainment = Intrattenimento
business-category-other = Altro

# =============================================================================
# DETTAGLIO ATTIVITÀ
# =============================================================================

business-detail-about = Informazioni
business-detail-products = Prodotti
business-detail-reviews = Recensioni
business-detail-contact = Contatti
business-detail-hours = Orari

business-about-description = Descrizione
business-about-address = Indirizzo
business-about-phone = Telefono
business-about-email = Email
business-about-website = Sito web
business-about-owner = Proprietario

business-hours-monday = Lunedì
business-hours-tuesday = Martedì
business-hours-wednesday = Mercoledì
business-hours-thursday = Giovedì
business-hours-friday = Venerdì
business-hours-saturday = Sabato
business-hours-sunday = Domenica
business-hours-closed = Chiuso
business-hours-open-now = Aperto ora
business-hours-closed-now = Chiuso ora

# =============================================================================
# CREA ATTIVITÀ
# =============================================================================

business-create-title = Registra la tua Attività
business-create-subtitle = Aggiungi la tua attività alla community

business-create-name-label = Nome attività
business-create-name-placeholder = Il nome della tua attività
business-create-description-label = Descrizione
business-create-description-placeholder = Descrivi la tua attività...
business-create-category-label = Categoria
business-create-address-label = Indirizzo
business-create-address-placeholder = Via, numero civico, città
business-create-phone-label = Telefono
business-create-email-label = Email
business-create-website-label = Sito web
business-create-submit = Registra Attività

business-create-success = Attività registrata con successo!
business-create-error = Errore nella registrazione

# =============================================================================
# PRODOTTI
# =============================================================================

products-title = Prodotti e Servizi
products-empty = Nessun prodotto disponibile
products-add = Aggiungi prodotto

product-name = Nome
product-description = Descrizione
product-price = Prezzo
product-available = Disponibile
product-unavailable = Non disponibile

product-add-title = Aggiungi Prodotto
product-add-name-label = Nome prodotto
product-add-description-label = Descrizione
product-add-price-label = Prezzo
product-add-submit = Aggiungi

# =============================================================================
# RECENSIONI
# =============================================================================

reviews-title = Recensioni
reviews-empty = Nessuna recensione ancora
reviews-write = Scrivi una recensione
reviews-average = Valutazione media

review-rating-label = Valutazione
review-comment-label = Commento
review-comment-placeholder = Racconta la tua esperienza...
review-submit = Pubblica recensione
review-success = Recensione pubblicata!
review-already = Hai già recensito questa attività

review-helpful = Utile
review-report = Segnala

# Stelle
rating-1 = Pessimo
rating-2 = Scarso
rating-3 = Nella media
rating-4 = Buono
rating-5 = Eccellente

# =============================================================================
# ORDINI
# =============================================================================

orders-title = I miei ordini
orders-empty = Nessun ordine
orders-filter-all = Tutti
orders-filter-pending = In attesa
orders-filter-completed = Completati
orders-filter-cancelled = Annullati

order-status-pending = In attesa
order-status-confirmed = Confermato
order-status-preparing = In preparazione
order-status-ready = Pronto
order-status-delivered = Consegnato
order-status-completed = Completato
order-status-cancelled = Annullato

order-total = Totale
order-items = { $count ->
    [one] { $count } articolo
   *[other] { $count } articoli
}
order-placed = Ordine del { $date }
order-details = Dettagli ordine
