# 🗺️ Civiqo Navigation Matrix

> Mappa completa di tutte le connessioni tra pagine e componenti dell'applicazione.

**Ultimo aggiornamento**: 2025-11-27  
**Versione**: 1.0.0  
**Maintainer**: Agente UX

---

## 📋 Indice

1. [Matrice di Navigazione](#matrice-di-navigazione)
2. [Grafo Connessioni Dettagliato](#grafo-connessioni-dettagliato)
3. [Entry/Exit Points per Pagina](#entryexit-points-per-pagina)
4. [Azioni e Destinazioni](#azioni-e-destinazioni)
5. [Stati e Transizioni](#stati-e-transizioni)
6. [Gap Analysis](#gap-analysis)

---

## Matrice di Navigazione

### Legenda
- ✅ Collegamento implementato
- ❌ Collegamento mancante (da implementare)
- ➖ Non applicabile
- 🔄 Collegamento bidirezionale

### Matrice Pagine Principali

| DA ↓ / A → | Landing | Dashboard | Communities | Community Detail | Governance | Chat | Profile | Post Detail | Notifications |
|------------|---------|-----------|-------------|------------------|------------|------|---------|-------------|---------------|
| **Landing** | ➖ | ✅ login | ✅ explore | ❌ | ❌ | ❌ | ❌ | ❌ | ➖ |
| **Dashboard** | ✅ logo | ➖ | ✅ navbar | ✅ widget | ✅ widget+navbar | ✅ navbar | ✅ avatar | ❌ | ❌ dropdown |
| **Communities** | ✅ logo | ✅ navbar | ➖ | ✅ card click | ✅ navbar | ✅ navbar | ✅ avatar | ❌ | ❌ |
| **Community Detail** | ✅ logo | ✅ navbar | ✅ breadcrumb | ➖ | ✅ tab | ✅ navbar | ✅ avatar | ✅ post click | ❌ |
| **Governance** | ✅ logo | ✅ navbar | ✅ proposal→community | ✅ proposal | ➖ | ✅ navbar | ✅ avatar | ❌ | ❌ |
| **Chat** | ✅ logo | ✅ navbar | ✅ navbar | ❌ | ✅ navbar | ➖ | ✅ avatar | ❌ | ❌ |
| **Profile** | ✅ logo | ✅ navbar | ✅ user communities | ✅ community link | ✅ navbar | ✅ navbar | ➖ | ✅ user posts | ❌ |
| **Post Detail** | ✅ logo | ✅ navbar | ✅ breadcrumb | ✅ breadcrumb | ✅ navbar | ✅ navbar | ✅ author | ➖ | ❌ |

### Connessioni Mancanti Critiche ❌

| Da | A | Priorità | Note |
|----|---|----------|------|
| Dashboard | Post Detail | 🟡 Media | Activity feed dovrebbe linkare ai post |
| Landing | Governance | 🟢 Bassa | CTA "Partecipa alle decisioni" |
| Community Detail | Notifications | 🔴 Alta | "Attiva notifiche per questa community" |
| Qualsiasi | Notifications Page | 🔴 Alta | Pagina notifiche non esiste |
| Qualsiasi | Search Results Page | 🟡 Media | Solo dropdown, no pagina dedicata |

---

## Grafo Connessioni Dettagliato

### Livello 1: Navigazione Globale (Navbar)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              NAVBAR (sempre visibile)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  [Logo] ──────► Landing (/)                                                 │
│                                                                              │
│  [Communities] ──────► /communities                                         │
│                                                                              │
│  [Votazioni] ──────► /governance                                            │
│                                                                              │
│  [Chat] ──────► /chat                                                       │
│                                                                              │
│  [Search] ──────► Dropdown HTMX ──────► ❌ /search (MANCANTE)              │
│                                                                              │
│  [Notifications] ──────► Dropdown HTMX ──────► ❌ /notifications (MANCANTE)│
│                                                                              │
│  [Avatar] ──────► /users/:id                                                │
│                                                                              │
│  [Dashboard] ──────► /dashboard                                             │
│                                                                              │
│  [Logout] ──────► POST /auth/logout ──────► Landing                        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Livello 2: Flusso Community

```
                                    /communities
                                         │
                    ┌────────────────────┼────────────────────┐
                    │                    │                    │
                    ▼                    ▼                    ▼
              [Card Click]        [Create Button]       [Search]
                    │                    │                    │
                    ▼                    ▼                    ▼
           /communities/:id    /communities/create    HTMX Filter
                    │                    │
        ┌───────────┼───────────┐       │
        │           │           │       │
        ▼           ▼           ▼       │
    [Tab Feed]  [Tab Members] [Tab Votazioni]
        │           │           │
        ▼           ▼           ▼
   Post List    Members List  Proposals List
        │           │           │
        ▼           ▼           ▼
  /posts/:id   /users/:id    [Vote Action]
        │                       │
        ▼                       ▼
   Comments              HTMX Update
```

### Livello 3: Flusso Governance

```
                              /governance
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
                    ▼              ▼              ▼
              [Filter Active] [Filter All]  [Filter Mine]
                    │              │              │
                    └──────────────┼──────────────┘
                                   │
                                   ▼
                           Proposals List
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
                    ▼              ▼              ▼
              [Card Click]   [Vote Button]  [Community Link]
                    │              │              │
                    ▼              ▼              ▼
           Proposal Detail   HTMX Vote    /communities/:id
           (❌ MANCANTE)     Confirmation
```

### Livello 4: Flusso User Profile

```
                              /users/:id
                                   │
                    ┌──────────────┼──────────────┬──────────────┐
                    │              │              │              │
                    ▼              ▼              ▼              ▼
              [Tab Posts]   [Tab Communities] [Tab Followers] [Tab Following]
                    │              │              │              │
                    ▼              ▼              ▼              ▼
               Post List    Community List   User List      User List
                    │              │              │              │
                    ▼              ▼              ▼              ▼
              /posts/:id   /communities/:id  /users/:id    /users/:id
                    
                    
              [Edit Button] (solo owner)
                    │
                    ▼
            /users/:id/edit
                    │
                    ▼
              [Save] ──► /users/:id
```

---

## Entry/Exit Points per Pagina

### Landing (`/`)

| Entry Points | Exit Points |
|--------------|-------------|
| URL diretto | → Dashboard (login) |
| Logout redirect | → Communities (explore) |
| Logo click (da qualsiasi pagina) | → Auth0 (login/register) |

### Dashboard (`/dashboard`)

| Entry Points | Exit Points |
|--------------|-------------|
| Post-login redirect | → Community (widget click) |
| Navbar "Dashboard" | → Governance (widget click) |
| | → Profile (avatar click) |
| | → Communities (navbar) |
| | → Chat (navbar) |

### Community Detail (`/communities/:id`)

| Entry Points | Exit Points |
|--------------|-------------|
| Communities list (card click) | → Post Detail (post click) |
| Dashboard widget | → User Profile (member click) |
| Governance proposal link | → Create Post (/posts/new) |
| Search result | → Governance tab (proposals) |
| Direct URL/deep link | → Leave community (action) |
| | → Edit community (admin) |

### Post Detail (`/posts/:id`)

| Entry Points | Exit Points |
|--------------|-------------|
| Community feed (post click) | → Community (breadcrumb) |
| User profile posts | → Author profile (avatar click) |
| Search result | → Commenter profile (avatar click) |
| Direct URL/deep link | → Edit post (author) |
| Notification click | → Delete post (author/admin) |

### Governance (`/governance`)

| Entry Points | Exit Points |
|--------------|-------------|
| Navbar "Votazioni" | → Community (proposal community link) |
| Dashboard widget | → Proposal Detail (❌ MANCANTE) |
| | → Vote action (HTMX) |

### Profile (`/users/:id`)

| Entry Points | Exit Points |
|--------------|-------------|
| Navbar avatar | → Post Detail (post click) |
| Post author click | → Community (community click) |
| Comment author click | → Follower/Following profile |
| Member list click | → Edit Profile (owner) |
| Search result | |

---

## Azioni e Destinazioni

### Azioni Globali (disponibili ovunque)

| Azione | Trigger | Destinazione | Implementato |
|--------|---------|--------------|--------------|
| Go Home | Logo click | `/` | ✅ |
| Go Dashboard | Navbar link | `/dashboard` | ✅ |
| Go Communities | Navbar link | `/communities` | ✅ |
| Go Governance | Navbar link | `/governance` | ✅ |
| Go Chat | Navbar link | `/chat` | ✅ |
| Go Profile | Avatar click | `/users/:id` | ✅ |
| Search | Search input | Dropdown HTMX | ✅ |
| View Notifications | Bell click | Dropdown HTMX | ✅ |
| Logout | Button click | POST → `/` | ✅ |

### Azioni Contestuali

#### In Community Detail

| Azione | Trigger | Destinazione | Implementato |
|--------|---------|--------------|--------------|
| View Post | Post card click | `/posts/:id` | ✅ |
| Create Post | "Nuovo Post" button | `/communities/:id/posts/new` | ✅ |
| View Member | Member avatar click | `/users/:id` | ✅ |
| Join Community | "Unisciti" button | HTMX update | ✅ |
| Leave Community | "Lascia" button | HTMX update | ✅ |
| Create Proposal | "Nuova Proposta" button | Modal HTMX | ✅ |
| Vote | "Vota" button | HTMX update | ✅ |
| Edit Community | "Modifica" button | Modal/Page | ⚠️ Parziale |
| Delete Community | "Elimina" button | Confirm → redirect | ⚠️ Parziale |

#### In Post Detail

| Azione | Trigger | Destinazione | Implementato |
|--------|---------|--------------|--------------|
| Go to Community | Breadcrumb click | `/communities/:id` | ✅ |
| View Author | Author avatar click | `/users/:id` | ✅ |
| Add Comment | Form submit | HTMX append | ✅ |
| Reply to Comment | "Rispondi" click | HTMX form | ✅ |
| React to Post | Reaction button | HTMX update | ✅ |
| Edit Post | "Modifica" button | Modal/inline | ✅ |
| Delete Post | "Elimina" button | Confirm → redirect | ✅ |

#### In Governance

| Azione | Trigger | Destinazione | Implementato |
|--------|---------|--------------|--------------|
| View Proposal | Card click | Proposal detail | ❌ MANCANTE |
| Vote | "Vota" button | HTMX update | ✅ |
| Go to Community | Community name click | `/communities/:id` | ✅ |
| Filter by Status | Tab click | HTMX filter | ⚠️ Parziale |

---

## Stati e Transizioni

### Diagramma Stati Utente

```
                    ┌─────────────┐
                    │   VISITOR   │
                    │ (anonymous) │
                    └──────┬──────┘
                           │
                    [Login/Register]
                           │
                           ▼
                    ┌─────────────┐
                    │  NEW USER   │
                    │(first login)│
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
        [Skip Onboard] [Complete]  [Partial]
              │         Profile     Profile
              │            │            │
              ▼            ▼            ▼
        ┌─────────┐  ┌─────────┐  ┌─────────┐
        │ BROWSER │  │ ACTIVE  │  │INCOMPLETE│
        │(no comm)│  │  USER   │  │ PROFILE │
        └────┬────┘  └────┬────┘  └────┬────┘
             │            │            │
        [Join Comm]  [Continue]  [Complete]
             │            │            │
             └────────────┼────────────┘
                          │
                          ▼
                    ┌─────────────┐
                    │   MEMBER    │
                    │(in ≥1 comm) │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
        [Post/Comment] [Vote]    [Create Comm]
              │            │            │
              ▼            ▼            ▼
        ┌─────────┐  ┌─────────┐  ┌─────────┐
        │CONTRIBUT│  │  VOTER  │  │  ADMIN  │
        │   OR    │  │         │  │         │
        └─────────┘  └─────────┘  └─────────┘
```

### Transizioni Pagina con Stati

```
┌──────────────────────────────────────────────────────────────────┐
│                    COMMUNITY DETAIL PAGE                          │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  STATO: visitor (non loggato)                                    │
│  ├─ Può vedere: Feed (read-only), Members count                  │
│  ├─ Non può: Postare, Commentare, Votare, Vedere tab Votazioni   │
│  └─ CTA: "Accedi per partecipare"                                │
│                                                                   │
│  STATO: logged_in, non_member                                    │
│  ├─ Può vedere: Feed (read-only), Members, Votazioni (read-only) │
│  ├─ Non può: Postare, Commentare, Votare                         │
│  └─ CTA: "Unisciti alla community"                               │
│                                                                   │
│  STATO: member                                                    │
│  ├─ Può: Postare, Commentare, Votare, Creare proposte           │
│  ├─ Non può: Modificare community, Gestire membri                │
│  └─ CTA: "Nuovo Post", "Nuova Proposta"                          │
│                                                                   │
│  STATO: admin                                                     │
│  ├─ Può: Tutto di member + Modificare, Gestire membri           │
│  └─ CTA: "Modifica Community", "Gestisci Membri"                 │
│                                                                   │
│  STATO: owner                                                     │
│  ├─ Può: Tutto di admin + Eliminare community, Trasferire        │
│  └─ CTA: "Elimina Community", "Trasferisci Proprietà"           │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Gap Analysis

### Connessioni Mancanti per Priorità

#### 🔴 Critiche (Bloccano flussi principali)

| Gap | Impatto | Soluzione |
|-----|---------|-----------|
| Notifications Page | Utenti non possono vedere storico notifiche | Creare `/notifications` |
| Proposal Detail Page | Non si può vedere dettaglio proposta | Creare `/proposals/:id` |
| Onboarding Flow | Nuovi utenti persi | Creare welcome modal + wizard |
| Mobile Navigation | Sito inutilizzabile su mobile | Hamburger menu |

#### 🟡 Importanti (Degradano UX)

| Gap | Impatto | Soluzione |
|-----|---------|-----------|
| Search Results Page | Ricerca limitata a dropdown | Creare `/search?q=` |
| Dashboard → Post | Activity non cliccabile | Aggiungere link ai post |
| Settings Page | Nessuna gestione preferenze | Creare `/settings` |
| Error Pages | Errori mostrano pagina brutta | Creare 404/500 custom |

#### 🟢 Nice to Have

| Gap | Impatto | Soluzione |
|-----|---------|-----------|
| Landing → Governance | CTA mancante | Aggiungere link |
| Community → Chat Room | No chat diretta | Link a chat community |
| Breadcrumbs consistenti | Navigazione confusa | Standardizzare |

### Flussi Interrotti

| Flusso | Punto di Interruzione | Impatto |
|--------|----------------------|---------|
| Notifica → Contenuto | Dropdown non porta a pagina | Alto |
| Ricerca → Risultato | Solo dropdown, no deep link | Medio |
| Governance → Dettaglio | No pagina proposta | Alto |
| Onboarding → Prima Azione | Non esiste | Critico |

---

## Raccomandazioni

### Immediate (Sprint UX-1)
1. Creare pagina Notifications
2. Creare pagina Proposal Detail
3. Aggiungere hamburger menu mobile

### Breve Termine (Sprint UX-2/3)
4. Creare pagina Search Results
5. Implementare onboarding flow
6. Standardizzare breadcrumbs

### Medio Termine
7. Creare Settings page
8. Aggiungere deep links per notifiche
9. Implementare chat per community

---

*Documento mantenuto da Agente UX. Aggiornare dopo ogni modifica alla navigazione.*
