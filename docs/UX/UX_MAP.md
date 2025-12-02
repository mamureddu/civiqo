# Civiqo UX Map

> Documento principale della User Experience. Mantiene la mappa completa delle pagine, flussi e interazioni.

**Ultimo aggiornamento**: 2025-12-02  
**Versione**: 1.3.0 (Single-Community Refactor)  
**Maintainer**: Agente UX

> ✅ **STATO**: Single-Community Refactor Complete - Vedi `UX_AUDIT_SINGLE_COMMUNITY_2025-12-02.md` per analisi dettagliata

> 📁 **Cartella di lavoro**: `docs/UX/`

---

## 📍 Indice

1. [Grafo Navigazione](#grafo-navigazione)
2. [Pagine e Stati](#pagine-e-stati)
3. [Flussi Utente](#flussi-utente)
4. [Componenti Condivisi](#componenti-condivisi)
5. [Punti di Ingresso](#punti-di-ingresso)

---

## Grafo Navigazione

```
                                    ┌─────────────┐
                                    │   Landing   │
                                    │   (index)   │
                                    └──────┬──────┘
                                           │
                              ┌────────────┼────────────┐
                              │            │            │
                              ▼            ▼            ▼
                        ┌─────────┐  ┌──────────┐  ┌─────────┐
                        │  Login  │  │ Register │  │ Explore │
                        │ (Auth0) │  │ (Auth0)  │  │  (anon) │
                        └────┬────┘  └────┬─────┘  └────┬────┘
                             │            │             │
                             └─────┬──────┘             │
                                   │                    │
                                   ▼                    │
                            ┌────────────┐              │
                            │  Dashboard │◄─────────────┘
                            │   (auth)   │
                            └─────┬──────┘
                                  │
            ┌─────────────────────┼─────────────────────┐
            │                     │                     │
            ▼                     ▼                     ▼
    ┌───────────────┐     ┌─────────────┐      ┌────────────┐
    │  Communities  │     │    Chat     │      │ Governance │
    │    (list)     │     │   (list)    │      │  (global)  │
    └───────┬───────┘     └──────┬──────┘      └─────┬──────┘
            │                    │                   │
            ▼                    ▼                   │
    ┌───────────────┐     ┌─────────────┐            │
    │   Community   │     │  Chat Room  │            │
    │   (detail)    │     │  (realtime) │            │
    └───────┬───────┘     └─────────────┘            │
            │                                        │
    ┌───────┴───────────────────┬───────────────────┐
    │                           │                   │
    ▼                           ▼                   ▼
┌─────────┐              ┌────────────┐      ┌────────────┐
│  Posts  │              │  Members   │      │ Votazioni  │
│ (feed)  │              │  (list)    │      │ (tab/page) │
└────┬────┘              └────────────┘      └─────┬──────┘
     │                                             │
     ▼                                             ▼
┌─────────┐                                 ┌────────────┐
│  Post   │                                 │  Proposal  │
│(detail) │                                 │  (detail)  │
└─────────┘                                 └────────────┘


    ┌─────────────┐          ┌─────────────┐          ┌─────────────┐
    │  Businesses │          │     POI     │          │  Create    │
    │   (list)    │          │   (map)     │          │  Business   │
    └──────┬──────┘          └─────────────┘          └─────────────┘
           │
    ┌──────┴──────┐
    │             │
    ▼             ▼
┌─────────────┐ ┌─────────────┐
│  Business   │ │   Business   │
│  (detail)   │ │  (create)    │
└─────────────┘ └─────────────┘


    ┌─────────────┐
    │   Profile   │◄──── Accessible from navbar (user menu)
    │   (view)    │
    └──────┬──────┘
           │
           ▼
    ┌─────────────┐
    │   Profile   │
    │   (edit)    │
    └─────────────┘


    ┌─────────────┐
    │    Admin    │◄──── Accessible for admin users
    │  Dashboard  │
    └─────────────┘
        │
    ┌───┴───┬───────┐
    │       │       │
    ▼       ▼       ▼
Moderation Analytics Audit
```

---

## Pagine e Stati

### Legenda Stati Implementazione
- ✅ Completo e funzionante
- ⚠️ Parziale o con problemi
- ❌ Mancante
- 🔄 In sviluppo

### Pubbliche (No Auth)

| Pagina | Route | Stati | Implementazione | Note |
|--------|-------|-------|-----------------|------|
| Landing | `/` | default | ✅ | Hero + feature highlights |
| Communities (explore) | `/communities` | default, loading, empty | ✅ | Manca filtri avanzati |
| Community Detail | `/communities/:id` | default, loading, not_found | ✅ | Solo view per non-membri |
| 404 Error | `*` | default | ✅ | Pagina custom |
| 500 Error | `*` | default | ✅ | Pagina custom |

### Autenticate

| Pagina | Route | Stati | Implementazione | Note |
|--------|-------|-------|-----------------|------|
| Dashboard | `/dashboard` | default, loading | ✅ | Hub personale con widget votazioni |
| Communities | `/communities` | default, loading, empty | ✅ | Con azioni membro |
| Community Detail | `/communities/:id` | default, loading, not_found | ✅ | Tab: Feed, Members, Votazioni |
| Create Community | `/communities/create` | form, submitting, success, error | ✅ | Pagina dedicata |
| Post Detail | `/posts/:id` | default, loading, not_found | ✅ | Con commenti threading |
| Create Post | `/communities/:id/posts/new` | form, submitting, success, error | ✅ | Form semplice |
| Chat List | `/chat` | default, loading, empty | ✅ | UI completa |
| Chat Room | `/chat/:room_id` | default, loading, connecting | ✅ | WebSocket real-time |
| Governance | `/governance` | default, loading, empty | ✅ | Lista globale proposals |
| Proposal Detail | `/governance/:id` | default, loading, not_found | ✅ | Con votazione |
| Profile View | `/users/:id` | default, loading, not_found | ✅ | Pubblico |
| Profile Edit | `/users/:id/edit` | form, submitting, success, error | ✅ | Solo owner |
| Businesses | `/businesses` | default, loading, empty | ✅ | Lista + CRUD completo |
| Business Detail | `/businesses/:id` | default, loading, not_found | ✅ | Dettaglio completo |
| Create Business | `/businesses/new` | form, submitting, success, error | ✅ | Form completo |
| Admin Dashboard | `/admin` | default, loading | ✅ | Analytics, Moderation, Audit |
| Instance Settings | `/admin/settings` | form, saving, success, error | ✅ | Solo admin istanza |
| Notifications | `/notifications` | default, loading, empty | ✅ | Pagina dedicata |
| POI | `/poi` | default, loading | ⚠️ | Placeholder, non funzionale |
| Setup Wizard | `/setup` | wizard, submitting, success, error | ✅ | Solo se no community |
| **Search Results** | `/search` | default, loading, empty | ❌ | **DA CREARE** (solo dropdown) |

---

## Flussi Utente

### 🔐 F1: Onboarding Nuovo Utente

```
Landing → [CTA "Inizia"] → Auth0 Register → Dashboard → [Prompt "Unisciti a una community"]
                                                              │
                                                              ▼
                                                    Communities List → Join → Community Detail
```

**Touchpoints UX**:
- Welcome message personalizzato in dashboard
- Suggerimenti community basati su località
- Onboarding tooltip per prime azioni

### 🏘️ F2: Partecipazione Community

```
Community Detail → [Tab Feed] → Read Posts
                              → [CTA "Nuovo Post"] → Create Post → Success → Feed Updated
                              
Community Detail → [Tab Votazioni] → View Proposals
                                   → [CTA "Nuova Proposta"] → Create Proposal → Success → List Updated
                                   → [CTA "Vota"] → Vote Confirmation → Vote Recorded
```

**Touchpoints UX**:
- Badge notifica per nuove votazioni
- Countdown scadenza votazioni
- Feedback immediato post-voto

### 💬 F3: Comunicazione

```
Dashboard → [Widget Chat] → Chat List → Select Room → Chat Room → Send Message → Real-time Update
                                                                → Receive Message → Notification
```

**Touchpoints UX**:
- Indicatore online/offline
- Typing indicator
- Read receipts (opzionale)

### 🗳️ F4: Governance

```
Dashboard → [Widget Votazioni Attive] → Click Proposal → Community Detail (tab Votazioni)
                                                       → Vote → Confirmation → Updated Count

Navbar → [Link Votazioni] → Governance Page → Filter by Status → View All Proposals
```

**Touchpoints UX**:
- Badge contatore votazioni attive
- Progress bar quorum
- Risultati in tempo reale (post-chiusura)

---

## Componenti Condivisi

### Navbar
```
┌─────────────────────────────────────────────────────────────────────┐
│ [Logo]  Communities  Votazioni  Chat  │  [Search]  │ [Notif] [User]│
└─────────────────────────────────────────────────────────────────────┘
```

**Stati**:
- Logged out: Logo + Explore + Login/Register
- Logged in: Full nav + notifications + user menu

### Card Community
```
┌─────────────────────────────────────┐
│ [Cover Image]                       │
│ ┌─────┐                             │
│ │Icon │ Community Name              │
│ └─────┘ Description excerpt...      │
│         👥 123 members  📍 Location │
│                        [Join/View]  │
└─────────────────────────────────────┘
```

### Card Proposal
```
┌─────────────────────────────────────┐
│ [Type Icon] [Status Badge]          │
│ Proposal Title                      │
│ Description excerpt...              │
│ 👤 Author  🗳️ 45 votes  ⏱️ 2 days  │
│                            [Vote]   │
└─────────────────────────────────────┘
```

### Modal Standard
```
┌─────────────────────────────────────┐
│ Title                          [X]  │
├─────────────────────────────────────┤
│                                     │
│ Content                             │
│                                     │
├─────────────────────────────────────┤
│              [Cancel] [Primary CTA] │
└─────────────────────────────────────┘
```

### Toast Notifications
```
┌─────────────────────────────────────┐
│ ✓ Success message                   │  ← Green bg
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ ⚠ Warning message                   │  ← Yellow bg
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ ✕ Error message                     │  ← Red bg
└─────────────────────────────────────┘
```

---

## Punti di Ingresso

### Entry Points Esterni
| Sorgente | Landing | Azione Attesa |
|----------|---------|---------------|
| Google Search | `/` o `/communities/:id` | Explore/Join |
| Social Share | `/posts/:id` | Read/Engage |
| Email Notification | `/communities/:id?tab=governance` | Vote |
| Deep Link App | Qualsiasi | Context-aware |

### Entry Points Interni (Cross-linking)
| Da | A | Trigger |
|----|---|---------|
| Dashboard | Community | Click widget |
| Dashboard | Governance | Click votazione attiva |
| Community | Post | Click card |
| Community | Proposal | Click tab + card |
| Navbar | Qualsiasi | Click link |
| Search | Qualsiasi | Select result |
| Notification | Context | Click notification |

---

## Metriche UX da Tracciare

| Metrica | Target | Note |
|---------|--------|------|
| Time to First Action | < 30s | Dalla landing al primo click significativo |
| Onboarding Completion | > 80% | Utenti che completano setup profilo |
| Community Join Rate | > 40% | Visitatori che si uniscono |
| Proposal Participation | > 60% | Membri che votano almeno 1 proposta |
| Session Duration | > 5 min | Tempo medio per sessione |
| Return Rate (7d) | > 50% | Utenti che tornano entro 7 giorni |

---

## Changelog

| Data | Versione | Modifiche |
|------|----------|-----------|
| 2025-11-28 | 1.2.0 | Aggiornamento post-Phase 7: Admin, Businesses, Proposal Detail, Chat complete |
| 2025-11-27 | 1.1.0 | Post-Audit: identificate lacune |
| 2025-11-27 | 1.0.0 | Creazione iniziale con grafo navigazione, pagine, flussi |

---

*Documento mantenuto da Agente UX. Per modifiche, invocare `@Agente UX` con contesto.*
