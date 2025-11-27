# Civiqo User Flows

> Documentazione dettagliata dei flussi utente per ogni feature principale.

**Ultimo aggiornamento**: 2025-11-27  
**Versione**: 1.0.0  
**Maintainer**: Agente UX

---

## Indice Flussi

| ID | Nome | Stato | Priorità |
|----|------|-------|----------|
| F01 | Onboarding | ✅ Implementato | Alta |
| F02 | Community Discovery | ✅ Implementato | Alta |
| F03 | Community Participation | ✅ Implementato | Alta |
| F04 | Governance & Voting | ✅ Implementato | Alta |
| F05 | Chat & Messaging | 🔄 Parziale | Media |
| F06 | Profile Management | ✅ Implementato | Media |
| F07 | Business Directory | 🔄 Parziale | Bassa |
| F08 | POI & Maps | 📋 Pianificato | Bassa |

---

## F01: Onboarding

### Persona
**Nuovo Cittadino**: Utente che scopre Civiqo per la prima volta.

### Trigger
- Ricerca Google "partecipazione cittadina [città]"
- Condivisione social da amico
- QR code su materiale comunale

### Flusso Principale

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. LANDING PAGE                                                 │
│    ├─ Hero: "Partecipa alla vita della tua comunità"           │
│    ├─ Feature highlights (3 card)                               │
│    ├─ CTA primaria: "Inizia Ora" → Auth0                       │
│    └─ CTA secondaria: "Esplora" → Communities (anon)           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. AUTH0 REGISTRATION                                           │
│    ├─ Email/Password o Social Login                             │
│    ├─ Verifica email (se email/password)                        │
│    └─ Redirect → Dashboard                                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. DASHBOARD (First Visit)                                      │
│    ├─ Welcome banner: "Benvenuto su Civiqo!"                   │
│    ├─ Prompt: "Completa il tuo profilo" → Profile Edit         │
│    ├─ Suggerimenti community (basati su località se nota)      │
│    └─ Widget vuoti con CTA per popolarli                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. PROFILE COMPLETION (Opzionale ma incentivato)               │
│    ├─ Nome visualizzato                                         │
│    ├─ Bio                                                       │
│    ├─ Avatar                                                    │
│    └─ Località (per suggerimenti)                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 5. FIRST COMMUNITY JOIN                                         │
│    ├─ Browse communities                                        │
│    ├─ Click "Unisciti"                                         │
│    ├─ Conferma (community pubbliche) / Richiesta (private)     │
│    └─ Redirect → Community Detail                              │
└─────────────────────────────────────────────────────────────────┘
```

### Stati UI per Step

| Step | Loading | Empty | Error | Success |
|------|---------|-------|-------|---------|
| Landing | Skeleton hero | N/A | Fallback statico | N/A |
| Auth0 | Spinner Auth0 | N/A | Toast errore | Redirect |
| Dashboard | Skeleton widgets | "Nessuna attività" | Toast errore | Populated |
| Profile | Skeleton form | Pre-filled defaults | Inline errors | Toast success |
| Join | Button loading | N/A | Toast errore | Toast + redirect |

### Metriche
- **Conversion Rate**: Landing → Registration > 15%
- **Completion Rate**: Registration → First Join > 60%
- **Time to First Join**: < 5 minuti

---

## F02: Community Discovery

### Persona
**Cittadino Curioso**: Utente che cerca community rilevanti.

### Trigger
- Click "Communities" in navbar
- Ricerca nella search bar
- Suggerimento in dashboard

### Flusso Principale

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. COMMUNITIES LIST                                             │
│    ├─ Search bar con filtri                                     │
│    ├─ Grid/List di community cards                              │
│    ├─ Ordinamento: Popolari / Recenti / Vicine                 │
│    └─ Infinite scroll o pagination                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
┌───────────────────────┐   ┌───────────────────────┐
│ 2a. SEARCH            │   │ 2b. BROWSE            │
│     ├─ Debounced      │   │     ├─ Scroll         │
│     ├─ HTMX update    │   │     ├─ Load more      │
│     └─ Highlight      │   │     └─ Filter apply   │
└───────────────────────┘   └───────────────────────┘
                    │                   │
                    └─────────┬─────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. COMMUNITY CARD INTERACTION                                   │
│    ├─ Hover: shadow + border highlight                          │
│    ├─ Click card → Community Detail                            │
│    └─ Click "Unisciti" → Join flow (inline o modal)            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. COMMUNITY DETAIL (Preview)                                   │
│    ├─ Header con cover + info                                   │
│    ├─ Tab Feed (read-only se non membro)                       │
│    ├─ Tab Members (count visibile)                              │
│    └─ CTA "Unisciti" prominente                                │
└─────────────────────────────────────────────────────────────────┘
```

### Interazioni HTMX

```html
<!-- Search con debounce -->
<input type="text" 
       hx-get="/htmx/communities/search"
       hx-trigger="keyup changed delay:300ms"
       hx-target="#communities-grid">

<!-- Load more -->
<button hx-get="/htmx/communities/list?page=2"
        hx-target="#communities-grid"
        hx-swap="beforeend">
    Carica altre
</button>
```

---

## F03: Community Participation

### Persona
**Membro Attivo**: Utente membro di una community.

### Trigger
- Click su community in dashboard
- Notifica di nuova attività
- Deep link da email

### Flusso: Creazione Post

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. COMMUNITY DETAIL - TAB FEED                                  │
│    ├─ Lista post ordinati per data                              │
│    ├─ CTA "Nuovo Post" (solo membri)                           │
│    └─ Infinite scroll                                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. CREATE POST (Modal o Page)                                   │
│    ├─ Titolo (required)                                         │
│    ├─ Contenuto (rich text)                                     │
│    ├─ Allegati (opzionale)                                      │
│    └─ [Annulla] [Pubblica]                                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. SUBMIT                                                       │
│    ├─ Button → loading state                                    │
│    ├─ Success → Toast + Feed refresh                           │
│    └─ Error → Inline message + retry                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. POST VISIBLE IN FEED                                         │
│    ├─ Nuovo post in cima                                        │
│    ├─ Highlight temporaneo (2s)                                 │
│    └─ Notifica ad altri membri (async)                         │
└─────────────────────────────────────────────────────────────────┘
```

### Flusso: Interazione Post

```
Post Card
    │
    ├─ Click titolo → Post Detail
    │
    ├─ Click Like → Toggle (optimistic UI)
    │       └─ HTMX swap counter
    │
    ├─ Click Comment → Expand/Focus comment form
    │       └─ Submit → HTMX append comment
    │
    └─ Click Share → Share modal/native share
```

---

## F04: Governance & Voting

### Persona
**Cittadino Partecipe**: Utente che vuole influenzare decisioni.

### Entry Points
1. **Dashboard** → Widget "Votazioni Attive" → Click → Community tab Governance
2. **Navbar** → "Votazioni" → Governance page globale
3. **Community Detail** → Tab "Votazioni"
4. **Notifica** → "Nuova votazione" → Direct link

### Flusso: Visualizzazione Proposte

```
┌─────────────────────────────────────────────────────────────────┐
│ GOVERNANCE PAGE (Globale)                                       │
│    ├─ Filtri: Tutte / Attive / Concluse / Le mie              │
│    ├─ Lista proposte da tutte le community dell'utente         │
│    └─ Ordinamento: Scadenza / Recenti / Popolari              │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│ COMMUNITY DETAIL - TAB VOTAZIONI                                │
│    ├─ Badge counter proposte attive                             │
│    ├─ Lista proposte della community                            │
│    ├─ CTA "Nuova Proposta" (solo membri)                       │
│    └─ Filtri: Attive / Bozze / Concluse                        │
└─────────────────────────────────────────────────────────────────┘
```

### Flusso: Creazione Proposta

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. CLICK "NUOVA PROPOSTA"                                       │
│    └─ Apre modal dialog                                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. FORM PROPOSTA                                                │
│    ├─ Titolo (required)                                         │
│    ├─ Descrizione                                               │
│    ├─ Tipo: Discussione / Votazione Sì-No / Sondaggio         │
│    ├─ Data inizio votazione                                     │
│    ├─ Data fine votazione                                       │
│    └─ [Annulla] [Crea Proposta]                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. SUBMIT (HTMX)                                                │
│    ├─ POST /htmx/communities/:id/proposals                     │
│    ├─ Success → Lista aggiornata + modal close                 │
│    └─ Error → Messaggio inline nel modal                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. PROPOSTA CREATA (Status: Bozza)                             │
│    ├─ Visibile nella lista                                      │
│    ├─ Badge "📝 Bozza"                                         │
│    └─ Azioni: Modifica / Attiva / Elimina                      │
└─────────────────────────────────────────────────────────────────┘
```

### Flusso: Votazione

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. PROPOSAL CARD (Status: Attiva)                               │
│    ├─ Badge "🗳️ Votazione Aperta"                              │
│    ├─ Countdown scadenza                                        │
│    ├─ Counter voti attuali                                      │
│    └─ CTA "Vota"                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. VOTE ACTION                                                  │
│    ├─ Tipo "vote": Sì / No buttons                             │
│    ├─ Tipo "poll": Radio options                                │
│    └─ Conferma inline o modal                                   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. VOTE SUBMITTED                                               │
│    ├─ Optimistic UI: button disabled + "Votato ✓"             │
│    ├─ Counter incrementato                                      │
│    └─ Toast conferma                                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. POST-VOTE STATE                                              │
│    ├─ Mostra scelta utente                                      │
│    ├─ Opzione modifica voto (se permesso)                      │
│    └─ Risultati parziali (se configurato)                      │
└─────────────────────────────────────────────────────────────────┘
```

### Stati Proposta

| Stato | Badge | Azioni Disponibili | Visibilità Risultati |
|-------|-------|-------------------|---------------------|
| `draft` | 📝 Bozza | Modifica, Attiva, Elimina | Solo autore |
| `active` | 🗳️ Votazione Aperta | Vota (membri) | Counter voti |
| `closed` | ✓ Conclusa | Nessuna | Risultati completi |

---

## F05: Chat & Messaging

### Stato: 🔄 Parziale

### Flusso Pianificato

```
Dashboard → Chat Widget → Chat List → Select Room → Chat Room
                                                        │
                                                        ├─ Send Message
                                                        ├─ Receive (WebSocket)
                                                        └─ Typing Indicator
```

### TODO UX
- [ ] Definire UI per typing indicator
- [ ] Definire UI per read receipts
- [ ] Definire comportamento offline
- [ ] Definire notifiche push

---

## F06: Profile Management

### Flusso: View Profile

```
Navbar User Menu → "Profilo" → Profile View
                                    │
                                    ├─ Info utente
                                    ├─ Tab: Post / Communities / Followers / Following
                                    └─ CTA "Modifica" (solo owner)
```

### Flusso: Edit Profile

```
Profile View → "Modifica" → Profile Edit Form
                                    │
                                    ├─ Avatar upload
                                    ├─ Nome, Bio, Location
                                    ├─ Privacy settings
                                    └─ [Annulla] [Salva]
```

---

## Legenda Stati

| Simbolo | Significato |
|---------|-------------|
| ✅ | Implementato e testato |
| 🔄 | Parzialmente implementato |
| 📋 | Pianificato, non iniziato |
| ❌ | Deprecato o rimosso |

---

*Documento mantenuto da Agente UX. Per aggiornamenti, invocare `@Agente UX`.*
