# 📊 Civiqo User Flows Master Document

> Documento master per tracciare tutti gli user flows, il loro stato e le dipendenze.

**Ultimo aggiornamento**: 2025-11-27  
**Versione**: 1.0.0  
**Maintainer**: Agente UX

---

## 📋 Indice Flows

| ID | Nome | Categoria | Stato | Priorità |
|----|------|-----------|-------|----------|
| UF-001 | First Visit (Anonymous) | Onboarding | ✅ Completo | - |
| UF-002 | Registration | Onboarding | ✅ Completo | - |
| UF-003 | First Login | Onboarding | ❌ Incompleto | 🔴 P0 |
| UF-004 | Profile Setup | Onboarding | ⚠️ Parziale | 🟠 P1 |
| UF-005 | Community Discovery | Core | ✅ Completo | - |
| UF-006 | Join Community | Core | ✅ Completo | - |
| UF-007 | Create Post | Core | ✅ Completo | - |
| UF-008 | Comment on Post | Core | ✅ Completo | - |
| UF-009 | Create Proposal | Governance | ✅ Completo | - |
| UF-010 | Vote on Proposal | Governance | ✅ Completo | - |
| UF-011 | View Notifications | Engagement | ⚠️ Parziale | 🔴 P0 |
| UF-012 | Search | Discovery | ⚠️ Parziale | 🟡 P2 |
| UF-013 | Follow User | Social | ⚠️ Parziale | 🟡 P2 |
| UF-014 | Chat | Communication | ⚠️ Parziale | 🟡 P2 |
| UF-015 | Create Community | Admin | ✅ Completo | - |
| UF-016 | Manage Community | Admin | ⚠️ Parziale | 🟡 P2 |
| UF-017 | Error Recovery | System | ❌ Mancante | 🟠 P1 |

---

## 🔄 User Flows Dettagliati

### UF-001: First Visit (Anonymous) ✅

**Obiettivo**: Utente anonimo esplora il sito e capisce il valore di Civiqo.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Landing Page                                            │
│ URL: /                                                          │
│ Stato: ✅ Implementato                                          │
├─────────────────────────────────────────────────────────────────┤
│ ELEMENTI:                                                       │
│ ✅ Hero con value proposition                                   │
│ ✅ Feature highlights (3 card)                                  │
│ ✅ CTA "Explore Communities"                                    │
│ ✅ CTA "Sign In"                                                │
│ ✅ Recent communities (HTMX)                                    │
├─────────────────────────────────────────────────────────────────┤
│ AZIONI POSSIBILI:                                               │
│ → Click "Explore" → UF-005 (Community Discovery)               │
│ → Click "Sign In" → UF-002 (Registration)                      │
│ → Click community card → Community Detail (read-only)          │
└─────────────────────────────────────────────────────────────────┘
```

**Metriche**:
- Bounce rate target: < 50%
- Click-through to communities: > 30%
- Click-through to login: > 15%

---

### UF-002: Registration ✅

**Obiettivo**: Utente crea un account.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Click Login/Register                                    │
│ Trigger: CTA su landing o navbar                                │
│ Stato: ✅ Implementato                                          │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Auth0 Universal Login                                   │
│ URL: Auth0 hosted page                                          │
│ Stato: ✅ Implementato                                          │
│ OPZIONI:                                                        │
│ ✅ Email/Password                                               │
│ ✅ Google OAuth                                                 │
│ ⚠️ Altri social (configurabili in Auth0)                       │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Callback                                                │
│ URL: /auth/callback                                             │
│ Stato: ✅ Implementato                                          │
│ AZIONE: Crea/aggiorna utente in DB, crea sessione              │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Redirect                                                │
│ URL: /dashboard                                                 │
│ Stato: ✅ Implementato                                          │
│ PROBLEMA: ❌ Nessun onboarding per nuovi utenti                │
└─────────────────────────────────────────────────────────────────┘
```

**Gap identificati**:
- ❌ Nessun welcome modal
- ❌ Nessuna distinzione primo login vs login successivi
- ❌ Nessun tracking "is_new_user"

---

### UF-003: First Login ❌ INCOMPLETO

**Obiettivo**: Guidare nuovo utente verso prima azione significativa.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Post-Auth Redirect                                      │
│ Stato: ❌ DA IMPLEMENTARE                                       │
├─────────────────────────────────────────────────────────────────┤
│ IDEALE:                                                         │
│ 1. Detectare se primo login (flag in DB o session)             │
│ 2. Mostrare Welcome Modal                                       │
│ 3. Offrire scelte:                                              │
│    - "Esplora Community" → /communities                        │
│    - "Completa Profilo" → /users/:id/edit                      │
│    - "Salta" → /dashboard                                       │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Community Suggestions                                   │
│ Stato: ❌ DA IMPLEMENTARE                                       │
├─────────────────────────────────────────────────────────────────┤
│ IDEALE:                                                         │
│ 1. Se utente ha località → suggerisci community vicine         │
│ 2. Altrimenti → suggerisci community popolari                  │
│ 3. Mostrare max 3 suggerimenti con "Unisciti" rapido           │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: First Action Celebration                                │
│ Stato: ❌ DA IMPLEMENTARE                                       │
├─────────────────────────────────────────────────────────────────┤
│ IDEALE:                                                         │
│ 1. Quando utente completa prima azione (join/post/vote)        │
│ 2. Mostrare toast celebrativo                                   │
│ 3. Suggerire prossima azione                                    │
└─────────────────────────────────────────────────────────────────┘
```

**Implementazione richiesta**:
1. Aggiungere campo `first_login_completed` a users
2. Creare endpoint `/htmx/onboarding/welcome-modal`
3. Creare endpoint `/htmx/onboarding/suggested-communities`
4. Creare componente toast celebrativo

---

### UF-004: Profile Setup ⚠️ PARZIALE

**Obiettivo**: Utente completa il proprio profilo.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Accesso a Edit Profile                                  │
│ Stato: ✅ Implementato                                          │
│ TRIGGER:                                                        │
│ ✅ Click su avatar → profile → "Modifica"                      │
│ ❌ Banner "Completa profilo" su dashboard (MANCANTE)           │
│ ❌ Prompt in onboarding (MANCANTE)                             │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Form Edit Profile                                       │
│ URL: /users/:id/edit                                            │
│ Stato: ✅ Implementato                                          │
│ CAMPI:                                                          │
│ ✅ Nome                                                         │
│ ✅ Bio                                                          │
│ ✅ Avatar (URL)                                                 │
│ ⚠️ Location (campo esiste ma non usato per suggerimenti)       │
│ ❌ Upload avatar diretto (solo URL)                            │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Salvataggio                                             │
│ Stato: ✅ Implementato                                          │
│ AZIONE: PUT /api/users/:id → redirect a profile                │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Feedback                                                │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Redirect a profile                                           │
│ ❌ Toast di conferma (MANCANTE)                                │
│ ❌ Progress indicator completamento (MANCANTE)                 │
└─────────────────────────────────────────────────────────────────┘
```

**Gap identificati**:
- ❌ Nessun prompt per completare profilo
- ❌ Nessun progress indicator
- ❌ Upload avatar non implementato

---

### UF-005: Community Discovery ✅

**Obiettivo**: Utente trova community di interesse.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Accesso a Communities                                   │
│ URL: /communities                                               │
│ Stato: ✅ Implementato                                          │
│ TRIGGER:                                                        │
│ ✅ Navbar "Communities"                                         │
│ ✅ Landing "Explore"                                            │
│ ✅ Dashboard widget                                             │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Browse/Search                                           │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Lista community con card                                     │
│ ✅ Search con HTMX                                              │
│ ❌ Filtri per categoria (MANCANTE)                             │
│ ❌ Filtri per località (MANCANTE)                              │
│ ❌ Ordinamento (popolari/recenti/vicine) (MANCANTE)            │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: View Community                                          │
│ URL: /communities/:id                                           │
│ Stato: ✅ Implementato                                          │
│ ELEMENTI:                                                       │
│ ✅ Header con info community                                    │
│ ✅ Tab Feed/Members/Votazioni                                   │
│ ✅ Join button (se non membro)                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-006: Join Community ✅

**Obiettivo**: Utente si unisce a una community.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Click "Unisciti"                                        │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Community detail page, community card                 │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2a: Community Pubblica                                     │
│ Stato: ✅ Implementato                                          │
│ AZIONE: POST /api/communities/:id/join                         │
│ RISULTATO: Membro immediato, HTMX update button                │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2b: Community Privata                                      │
│ Stato: ✅ Implementato                                          │
│ AZIONE: POST /api/communities/:id/request-join                 │
│ RISULTATO: Richiesta pendente, attesa approvazione             │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Feedback                                                │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Button cambia stato                                          │
│ ❌ Toast di conferma (MANCANTE)                                │
│ ❌ Notifica per richiesta pendente (MANCANTE)                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-007: Create Post ✅

**Obiettivo**: Membro crea un post nella community.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Click "Nuovo Post"                                      │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Community detail (tab Feed)                           │
│ VISIBILITÀ: Solo membri                                         │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Form Creazione                                          │
│ URL: /communities/:id/posts/new                                 │
│ Stato: ✅ Implementato                                          │
│ CAMPI:                                                          │
│ ✅ Titolo (required)                                            │
│ ✅ Contenuto (textarea)                                         │
│ ❌ Rich text editor (solo textarea)                            │
│ ❌ Media upload (MANCANTE)                                     │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Submit                                                  │
│ Stato: ✅ Implementato                                          │
│ AZIONE: POST /api/communities/:id/posts                        │
│ REDIRECT: /posts/:id (nuovo post)                              │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Feedback                                                │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Redirect a post                                              │
│ ❌ Toast di conferma (MANCANTE)                                │
│ ❌ Notifica ad altri membri (MANCANTE)                         │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-009: Create Proposal ✅

**Obiettivo**: Membro crea una proposta/votazione.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Click "Nuova Proposta"                                  │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Community detail (tab Votazioni)                      │
│ VISIBILITÀ: Solo membri                                         │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Modal Form                                              │
│ Stato: ✅ Implementato                                          │
│ CAMPI:                                                          │
│ ✅ Titolo (required)                                            │
│ ✅ Descrizione                                                  │
│ ✅ Tipo (Discussione/Votazione/Sondaggio)                      │
│ ✅ Data inizio/fine votazione                                   │
│ ❌ Opzioni per sondaggio (MANCANTE)                            │
│ ❌ Quorum configurabile (campo esiste ma non in UI)            │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Submit (HTMX)                                           │
│ Stato: ✅ Implementato                                          │
│ AZIONE: POST /htmx/communities/:id/proposals                   │
│ RISULTATO: Lista aggiornata, modal chiuso                      │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Attivazione                                             │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Proposta creata come "bozza"                                 │
│ ❌ UI per attivare proposta (MANCANTE)                         │
│ ❌ Notifica membri quando attivata (MANCANTE)                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-010: Vote on Proposal ✅

**Obiettivo**: Membro vota su una proposta attiva.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Visualizza Proposta Attiva                              │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Community tab Votazioni, Dashboard widget, /governance│
│ ELEMENTI:                                                       │
│ ✅ Badge "Votazione Aperta"                                     │
│ ✅ Countdown scadenza                                           │
│ ✅ Counter voti                                                 │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Click "Vota"                                            │
│ Stato: ✅ Implementato                                          │
│ AZIONE: POST /api/proposals/:id/vote                           │
│ RISULTATO: HTMX update                                          │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Conferma                                                │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Counter incrementato                                         │
│ ❌ Toast conferma (MANCANTE)                                   │
│ ❌ Mostra scelta utente (MANCANTE)                             │
│ ❌ Opzione modifica voto (MANCANTE)                            │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Risultati                                               │
│ Stato: ❌ DA IMPLEMENTARE                                       │
│ ❌ Visualizzazione risultati post-chiusura                     │
│ ❌ Grafici/percentuali                                         │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-011: View Notifications ⚠️ PARZIALE

**Obiettivo**: Utente vede e gestisce le notifiche.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Click Bell Icon                                         │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Navbar                                                │
│ AZIONE: Apre dropdown HTMX                                      │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Dropdown Notifiche                                      │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Lista ultime notifiche                                       │
│ ❌ Mark as read singola (MANCANTE)                             │
│ ❌ Link "Vedi tutte" (MANCANTE)                                │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Pagina Notifiche                                        │
│ URL: /notifications                                             │
│ Stato: ❌ MANCANTE                                              │
│ IDEALE:                                                         │
│ - Lista completa paginata                                       │
│ - Filtri (tutte/non lette/per tipo)                            │
│ - Mark all as read                                              │
│ - Click → vai al contenuto                                      │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Click Notifica                                          │
│ Stato: ❌ MANCANTE                                              │
│ IDEALE:                                                         │
│ - Mark as read automatico                                       │
│ - Redirect al contenuto (post, proposta, community)            │
└─────────────────────────────────────────────────────────────────┘
```

**Implementazione richiesta**:
1. Creare pagina `/notifications`
2. Aggiungere link "Vedi tutte" in dropdown
3. Implementare mark as read
4. Implementare deep link da notifica a contenuto

---

### UF-012: Search ⚠️ PARZIALE

**Obiettivo**: Utente cerca contenuti nell'app.

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Focus Search Bar                                        │
│ Stato: ✅ Implementato                                          │
│ LOCATION: Navbar (desktop only)                                 │
│ ❌ Non disponibile su mobile                                   │
├─────────────────────────────────────────────────────────────────┤
│ STEP 2: Digitazione                                             │
│ Stato: ✅ Implementato                                          │
│ AZIONE: Debounced HTMX call                                     │
│ RISULTATO: Dropdown con risultati                               │
├─────────────────────────────────────────────────────────────────┤
│ STEP 3: Risultati Dropdown                                      │
│ Stato: ✅ Implementato                                          │
│ MOSTRA:                                                         │
│ ✅ Community matching                                           │
│ ✅ Users matching                                               │
│ ⚠️ Posts matching (limitato)                                   │
├─────────────────────────────────────────────────────────────────┤
│ STEP 4: Pagina Risultati                                        │
│ URL: /search?q=                                                 │
│ Stato: ❌ MANCANTE                                              │
│ IDEALE:                                                         │
│ - Risultati completi paginati                                   │
│ - Filtri per tipo (community/users/posts)                      │
│ - Ordinamento (rilevanza/data)                                  │
│ - URL condivisibile                                             │
└─────────────────────────────────────────────────────────────────┘
```

---

### UF-017: Error Recovery ❌ MANCANTE

**Obiettivo**: Gestire errori in modo user-friendly.

```
┌─────────────────────────────────────────────────────────────────┐
│ SCENARIO 1: 404 Not Found                                       │
│ Stato: ❌ MANCANTE                                              │
│ ATTUALE: Pagina browser default                                 │
│ IDEALE:                                                         │
│ - Pagina custom brand-compliant                                 │
│ - Suggerimenti (home, search, popular)                         │
│ - Link a supporto                                               │
├─────────────────────────────────────────────────────────────────┤
│ SCENARIO 2: 500 Server Error                                    │
│ Stato: ❌ MANCANTE                                              │
│ ATTUALE: Pagina browser default                                 │
│ IDEALE:                                                         │
│ - Pagina custom con messaggio amichevole                       │
│ - "Stiamo lavorando per risolvere"                             │
│ - Retry button                                                  │
├─────────────────────────────────────────────────────────────────┤
│ SCENARIO 3: Network Error                                       │
│ Stato: ❌ MANCANTE                                              │
│ ATTUALE: Nessun feedback                                        │
│ IDEALE:                                                         │
│ - Toast "Connessione persa"                                    │
│ - Retry automatico per HTMX                                     │
│ - Offline indicator                                             │
├─────────────────────────────────────────────────────────────────┤
│ SCENARIO 4: Form Validation Error                               │
│ Stato: ⚠️ Parziale                                             │
│ ✅ Inline errors per alcuni form                                │
│ ❌ Stile inconsistente                                         │
│ ❌ Focus automatico su primo errore                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📊 Matrice Dipendenze Flows

```
UF-001 (First Visit)
    │
    ├──► UF-002 (Registration)
    │        │
    │        └──► UF-003 (First Login) ❌
    │                 │
    │                 ├──► UF-004 (Profile Setup)
    │                 │
    │                 └──► UF-005 (Community Discovery)
    │                          │
    │                          └──► UF-006 (Join Community)
    │                                   │
    │                                   ├──► UF-007 (Create Post)
    │                                   │        │
    │                                   │        └──► UF-008 (Comment)
    │                                   │
    │                                   ├──► UF-009 (Create Proposal)
    │                                   │        │
    │                                   │        └──► UF-010 (Vote)
    │                                   │
    │                                   └──► UF-014 (Chat)
    │
    └──► UF-005 (Community Discovery) [anonymous browse]

UF-011 (Notifications) ◄── Triggered by: UF-006, UF-007, UF-008, UF-009, UF-010

UF-012 (Search) ◄── Entry point to: UF-005, UF-004 (user profiles)

UF-017 (Error Recovery) ◄── Can occur in: ANY FLOW
```

---

## 🎯 Piano Implementazione Flows

### Priorità 0 (Bloccanti)

| Flow | Gap | Azione | Effort |
|------|-----|--------|--------|
| UF-003 | Onboarding assente | Creare welcome modal + suggestions | 3 giorni |
| UF-011 | Notifications page | Creare /notifications | 2 giorni |
| UF-017 | Error pages | Creare 404/500 | 1 giorno |

### Priorità 1 (Importanti)

| Flow | Gap | Azione | Effort |
|------|-----|--------|--------|
| UF-004 | Profile prompt | Aggiungere banner dashboard | 0.5 giorni |
| UF-012 | Search page | Creare /search | 2 giorni |
| UF-010 | Vote results | Visualizzare risultati | 1 giorno |

### Priorità 2 (Miglioramenti)

| Flow | Gap | Azione | Effort |
|------|-----|--------|--------|
| UF-005 | Filtri community | Aggiungere categoria/località | 2 giorni |
| UF-007 | Rich text | Implementare editor | 3 giorni |
| UF-009 | Poll options | UI per opzioni sondaggio | 1 giorno |

---

## ✅ Checklist per Nuovo Flow

Quando si aggiunge un nuovo flow:

- [ ] Definire obiettivo utente
- [ ] Mappare tutti gli step
- [ ] Identificare trigger/entry points
- [ ] Definire stati UI per ogni step
- [ ] Identificare dipendenze da altri flows
- [ ] Definire metriche di successo
- [ ] Documentare in questo file
- [ ] Aggiornare Navigation Matrix
- [ ] Aggiornare UX_MAP.md

---

*Documento mantenuto da Agente UX. Aggiornare dopo ogni modifica ai flows.*
