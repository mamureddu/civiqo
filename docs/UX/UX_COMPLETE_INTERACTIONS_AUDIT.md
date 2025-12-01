# UX Complete Interactions Audit - Civiqo

**Data**: 2024-12-01  
**Versione**: 2.0  
**Stato**: AUDIT COMPLETO

---

## Executive Summary

Questo documento elenca **TUTTE** le interazioni di navigazione identificate analizzando i 43 template HTML dell'applicazione. Per ogni pagina sono elencati:
- Link di navigazione presenti
- Link mancanti
- Azioni HTMX
- Policy di visibilità

---

## 1. Architettura Corretta (Principio Fondamentale)

### ⚠️ PROBLEMA CRITICO IDENTIFICATO

**Businesses, Governance e POI sono attualmente globali ma dovrebbero essere INTERNI alle Community.**

```
ARCHITETTURA ERRATA (attuale):
├── /communities
├── /businesses      ← GLOBALE (ERRORE!)
├── /governance      ← GLOBALE (ERRORE!)
├── /poi             ← GLOBALE (ERRORE!)
└── /chat

ARCHITETTURA CORRETTA (target):
├── /communities
│   └── /communities/{id}
│       ├── Tab: Feed
│       ├── Tab: Membri
│       ├── Tab: Info
│       ├── Tab: Governance/Votazioni ✓ (già presente)
│       ├── Tab: Attività Locali (businesses) ← DA AGGIUNGERE
│       ├── Tab: Chat ← DA AGGIUNGERE
│       └── Tab: POI/Mappa ← DA AGGIUNGERE
├── /dashboard (aggregatore personale)
├── /governance (aggregatore proposte dalle mie community) ← OK se aggregatore
└── /chat (lista chat delle mie community)
```

---

## 2. Inventario Completo Template

### 2.1 Template Pagine Principali (15)

| Template | Route | Descrizione |
|----------|-------|-------------|
| `index.html` | `/` | Landing page |
| `dashboard.html` | `/dashboard` | Dashboard utente |
| `communities.html` | `/communities` | Lista community |
| `community_detail.html` | `/communities/{id}` | Dettaglio community |
| `community_posts.html` | `/communities/{id}/posts` | Posts di una community |
| `create_community.html` | `/communities/create` | Crea community |
| `create_post.html` | `/communities/{id}/posts/new` | Crea post |
| `post_detail.html` | `/posts/{id}` | Dettaglio post |
| `governance.html` | `/governance` | Governance globale |
| `proposal_detail.html` | `/governance/{id}` | Dettaglio proposta |
| `businesses.html` | `/businesses` | Lista attività (GLOBALE) |
| `business_detail.html` | `/businesses/{id}` | Dettaglio attività |
| `create_business.html` | `/businesses/new` | Crea attività |
| `chat_list.html` | `/chat` | Lista chat rooms |
| `chat.html` | `/chat/{room_id}` | Chat room |
| `profile.html` | `/users/{id}` | Profilo utente |
| `notifications.html` | `/notifications` | Pagina notifiche |
| `poi.html` | `/poi` | Mappa POI |
| `admin.html` | `/admin` | Pannello admin |
| `404.html` | - | Errore 404 |
| `500.html` | - | Errore 500 |

### 2.2 Template Fragments (22)

| Fragment | Uso |
|----------|-----|
| `business-card.html` | Card attività |
| `comment-form.html` | Form commento |
| `comment-item.html` | Singolo commento |
| `community-card.html` | Card community |
| `empty-state.html` | Stato vuoto |
| `follow-button.html` | Pulsante follow |
| `join-button.html` | Pulsante join |
| `members-list.html` | Lista membri |
| `notifications-list.html` | Lista notifiche |
| `post-card.html` | Card post |
| `post-form.html` | Form post |
| `product-card.html` | Card prodotto |
| `profile-completion-banner.html` | Banner completamento profilo |
| `rating-stars.html` | Stelle rating |
| `reaction-buttons.html` | Pulsanti reazioni |
| `review-card.html` | Card recensione |
| `review-form.html` | Form recensione |
| `search-results.html` | Risultati ricerca |
| `toast.html` | Toast notification |
| `user-card.html` | Card utente |
| `welcome-modal.html` | Modal benvenuto |

---

## 3. Analisi Dettagliata per Template

### 3.1 `base.html` - Layout Base

#### Navbar Desktop - Link Presenti
| Link | Target | Visibilità | Stato |
|------|--------|------------|-------|
| Logo | `/` | Tutti | ✅ OK |
| Communities | `/communities` | Tutti | ✅ OK |
| **Attività** | `/businesses` | Tutti | ❌ **RIMUOVERE** |
| **Governance** | `/governance` | Tutti | ⚠️ Rivedere |
| Chat | `/chat` | Tutti | ✅ OK |
| Search | Dropdown HTMX | Tutti | ✅ OK |
| Notifications | Dropdown HTMX | Logged | ✅ OK |
| Dashboard | `/dashboard` | Logged | ⚠️ Non visibile |
| Profile | `/users/{id}` | Logged | ✅ OK |
| Login | `/auth/login` | Guest | ✅ OK |

#### Navbar Mobile - Link Presenti
| Link | Target | Stato |
|------|--------|-------|
| Communities | `/communities` | ✅ OK |
| **Attività** | `/businesses` | ❌ **RIMUOVERE** |
| Governance | `/governance` | ⚠️ Rivedere |
| Chat | `/chat` | ✅ OK |
| Dashboard | `/dashboard` | ✅ OK |
| Profile | `/users/{id}` | ✅ OK |

#### Footer - Link Presenti
| Link | Target | Stato |
|------|--------|-------|
| Language Switcher | POST `/api/set-language` | ✅ OK |
| Privacy | - | ❌ MANCANTE |
| Terms | - | ❌ MANCANTE |
| Contact | - | ❌ MANCANTE |

#### HTMX Endpoints Usati
| Endpoint | Trigger | Target |
|----------|---------|--------|
| `/htmx/search?q=` | Input debounce | Dropdown |
| `/htmx/notifications` | Bell click | Dropdown |

---

### 3.2 `index.html` - Landing Page

#### Link Presenti
| Elemento | Target | Visibilità |
|----------|--------|------------|
| CTA "Esplora" | `/communities` | Tutti |
| CTA "Accedi" | `/auth/login` | Guest |
| "Vedi tutte" | `/communities` | Tutti |
| Community Card | `/communities/{id}` | Tutti |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/communities/recent` | Load |

#### Link Mancanti
| Link | Priorità | Note |
|------|----------|------|
| CTA Governance | Bassa | "Partecipa alle decisioni" |
| CTA Chat | Bassa | "Unisciti alle conversazioni" |

---

### 3.3 `dashboard.html` - Dashboard Utente

#### Link Presenti
| Elemento | Target |
|----------|--------|
| "Vedi tutte" Governance | `/governance` |
| "Crea Community" | `/communities/create` |
| Community Card | `/communities/{id}` |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/dashboard/active-proposals` | Load |
| `/htmx/user/communities` | Load |
| `/htmx/user/activity` | Load |

#### Link Mancanti
| Link | Priorità | Note |
|------|----------|------|
| Post dall'activity feed | Alta | Click su attività → post |
| Notifiche preview | Media | Widget notifiche |
| Quick Actions | Media | Azioni rapide |

---

### 3.4 `communities.html` - Lista Community

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Community Card | `/communities/{id}` |
| "Crea Community" | Form inline |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/communities/list` | Load |
| `/htmx/communities/search?q=` | Input |
| `POST /api/communities` | Form submit |

#### Link Mancanti
| Link | Priorità |
|------|----------|
| "Le mie community" filter | Media |

---

### 3.5 `community_detail.html` - Dettaglio Community

#### Tabs Presenti
| Tab | Contenuto | Stato |
|-----|-----------|-------|
| Feed | Posts | ✅ OK |
| Membri | Lista membri | ✅ OK |
| Info | Descrizione | ✅ OK |
| Governance/Votazioni | Proposte | ✅ OK |
| **Attività** | Businesses | ❌ **MANCANTE** |
| **Chat** | Chat community | ❌ **MANCANTE** |
| **Eventi** | Calendario | ❌ **MANCANTE** |
| **POI** | Mappa | ❌ **MANCANTE** |

#### Link Presenti
| Elemento | Target | Visibilità |
|----------|--------|------------|
| "Nuovo Post" | `/communities/{id}/posts/new` | Membri |
| "Nuova Proposta" | Modal | Membri |
| Post Card | `/posts/{id}` | Tutti |
| Member Avatar | `/users/{id}` | Tutti |
| Join/Leave | HTMX | Logged |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/communities/{id}/feed` | Tab click |
| `/htmx/communities/{id}/members` | Tab click |
| `/htmx/communities/{id}/proposals` | Tab click |
| `/htmx/communities/{id}/proposals/count` | Load |
| `POST /api/communities/{id}/join` | Button |
| `POST /api/communities/{id}/leave` | Button |
| `POST /htmx/communities/{id}/proposals` | Form |

#### HTMX Endpoints MANCANTI
| Endpoint | Priorità |
|----------|----------|
| `/htmx/communities/{id}/businesses` | **ALTA** |
| `/htmx/communities/{id}/chat` | **ALTA** |
| `/htmx/communities/{id}/events` | Media |
| `/htmx/communities/{id}/poi` | Bassa |

---

### 3.6 `post_detail.html` - Dettaglio Post

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Breadcrumb Community | `/communities` |
| Breadcrumb Community Name | `/communities/{id}` |
| Breadcrumb Posts | `/communities/{id}/posts` |
| Author Avatar | `/users/{id}` |
| "Torna ai post" | `/communities/{id}/posts` |
| Edit Post | `/posts/{id}/edit` |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `DELETE /api/posts/{id}` | Delete button |
| `POST /api/posts/{id}/comments` | Comment form |
| Reaction buttons | Click |

#### Link Mancanti
| Link | Priorità |
|------|----------|
| Commenter Avatar → Profile | Media |

---

### 3.7 `governance.html` - Governance Globale

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Proposal Card | ❌ **MANCANTE** (no detail page link) |
| Community Name | `/communities/{id}` |
| "Crea Proposta" | Modal |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/governance/proposals` | Load |
| `/htmx/governance/proposals?status=` | Tab click |
| `/htmx/user/communities-options` | Load (select) |
| `POST /api/proposals` | Form |

#### Link Mancanti
| Link | Priorità |
|------|----------|
| Proposal → Detail Page | **ALTA** |
| Filter by Community | Media |

---

### 3.8 `proposal_detail.html` - Dettaglio Proposta

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Breadcrumb Governance | `/governance` |
| Community Link | `/communities/{id}` |
| Author Avatar | `/users/{id}` |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `POST /api/proposals/{id}/vote` | Vote button |

#### Link Mancanti
| Link | Priorità |
|------|----------|
| "Torna alla Community" evidente | Media |

---

### 3.9 `profile.html` - Profilo Utente

#### Tabs Presenti
| Tab | Contenuto |
|-----|-----------|
| Post | Posts utente |
| Community | Community utente |
| Follower | Lista follower |
| Seguiti | Lista following |

#### Link Presenti
| Elemento | Target |
|----------|--------|
| "Modifica Profilo" | `/users/{id}/edit` |
| Post Card | `/posts/{id}` |
| Community Card | `/communities/{id}` |
| User Card | `/users/{id}` |
| Website | External |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/users/{id}/follow-button` | Load |
| `/htmx/users/{id}/posts` | Tab intersect |
| `/htmx/users/{id}/communities` | Tab intersect |
| `/htmx/users/{id}/followers` | Tab intersect |
| `/htmx/users/{id}/following` | Tab intersect |

---

### 3.10 `notifications.html` - Notifiche

#### Link Presenti
| Elemento | Target |
|----------|--------|
| "Segna tutte come lette" | HTMX |
| Notification Item | Target contenuto |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/notifications/list` | Load |
| `/htmx/notifications/list?filter=` | Filter click |
| `POST /htmx/notifications/mark-all-read` | Button |

---

### 3.11 `chat_list.html` - Lista Chat

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Room Card | `/chat/{room_id}` |
| "Esplora Community" | `/communities` |

#### Link Mancanti
| Link | Priorità |
|------|----------|
| Indicatore community di appartenenza | Media |

---

### 3.12 `businesses.html` - Lista Attività (GLOBALE)

**⚠️ QUESTA PAGINA NON DOVREBBE ESISTERE COME GLOBALE**

#### Link Presenti
| Elemento | Target |
|----------|--------|
| Business Card | `/businesses/{id}` |
| "Nuova Attività" | `/businesses/new` |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/businesses/list` | Load |
| `/htmx/businesses/search?q=` | Input |

---

### 3.13 `admin.html` - Pannello Admin

#### Tabs Presenti
| Tab | Contenuto |
|-----|-----------|
| Moderation | Coda moderazione |
| Analytics | Statistiche |
| Audit | Log audit |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/admin/dashboard` | Load |
| `/htmx/admin/moderation` | Load/Filter |
| `/htmx/admin/analytics` | Tab |
| `/htmx/admin/audit-logs` | Tab |

---

### 3.14 `poi.html` - Mappa POI (GLOBALE)

**⚠️ QUESTA PAGINA DOVREBBE ESSERE INTERNA ALLE COMMUNITY**

#### Link Presenti
| Elemento | Target |
|----------|--------|
| "Add Place" | Modal/Form |

#### HTMX Endpoints
| Endpoint | Trigger |
|----------|---------|
| `/htmx/poi/nearby` | Load |

---

## 4. Matrice Completa Test di Navigazione

### 4.1 Test Navbar Globale

| ID | Test | Stato Attuale | Target |
|----|------|---------------|--------|
| NAV-01 | Navbar ha link Communities | ✅ Pass | ✅ |
| NAV-02 | Navbar ha link Chat | ✅ Pass | ✅ |
| NAV-03 | Navbar ha link Dashboard (logged) | ⚠️ Fail | ✅ |
| NAV-04 | Navbar ha Search | ✅ Pass | ✅ |
| NAV-05 | Navbar ha Notifications (logged) | ✅ Pass | ✅ |
| NAV-06 | Navbar ha Auth section | ✅ Pass | ✅ |
| NAV-07 | Navbar NO global Businesses | ❌ Fail | ✅ |
| NAV-08 | Navbar NO global Governance | ⚠️ Discutibile | - |

### 4.2 Test Community Detail Tabs

| ID | Test | Stato Attuale | Target |
|----|------|---------------|--------|
| COM-01 | Community ha tab Feed | ✅ Pass | ✅ |
| COM-02 | Community ha tab Membri | ✅ Pass | ✅ |
| COM-03 | Community ha tab Info | ✅ Pass | ✅ |
| COM-04 | Community ha tab Governance | ✅ Pass | ✅ |
| COM-05 | Community ha tab Attività | ❌ Fail | ✅ |
| COM-06 | Community ha tab Chat | ❌ Fail | ✅ |
| COM-07 | Community ha tab Eventi | ❌ Fail | ⚠️ P2 |
| COM-08 | Community ha tab POI | ❌ Fail | ⚠️ P3 |

### 4.3 Test HTMX Endpoints

| ID | Test | Stato Attuale | Target |
|----|------|---------------|--------|
| HTX-01 | `/htmx/communities/{id}/feed` | ✅ Pass | ✅ |
| HTX-02 | `/htmx/communities/{id}/members` | ✅ Pass | ✅ |
| HTX-03 | `/htmx/communities/{id}/proposals` | ✅ Pass | ✅ |
| HTX-04 | `/htmx/communities/{id}/businesses` | ❌ 404 | ✅ |
| HTX-05 | `/htmx/communities/{id}/chat` | ❌ 404 | ✅ |
| HTX-06 | `/htmx/communities/{id}/events` | ❌ 404 | ⚠️ P2 |
| HTX-07 | `/htmx/communities/{id}/poi` | ❌ 404 | ⚠️ P3 |

### 4.4 Test Breadcrumb e Back Links

| ID | Test | Stato Attuale | Target |
|----|------|---------------|--------|
| BRD-01 | Post detail ha breadcrumb community | ✅ Pass | ✅ |
| BRD-02 | Proposal detail ha breadcrumb | ✅ Pass | ✅ |
| BRD-03 | Proposal detail ha back to community | ⚠️ Weak | ✅ |
| BRD-04 | Create post ha breadcrumb | ✅ Pass | ✅ |

### 4.5 Test Footer

| ID | Test | Stato Attuale | Target |
|----|------|---------------|--------|
| FTR-01 | Footer ha language switcher | ✅ Pass | ✅ |
| FTR-02 | Footer ha link Privacy | ❌ Fail | ✅ |
| FTR-03 | Footer ha link Terms | ❌ Fail | ✅ |
| FTR-04 | Footer ha copyright | ✅ Pass | ✅ |

---

## 5. Piano Implementazione Prioritizzato

### Fase 0: Fix Critici (Bloccanti)

| # | Task | File | Test ID |
|---|------|------|---------|
| 0.1 | Rimuovere `/businesses` dalla navbar | `base.html` | NAV-07 |
| 0.2 | Creare endpoint `/htmx/communities/{id}/businesses` | `htmx.rs` | HTX-04 |
| 0.3 | Creare endpoint `/htmx/communities/{id}/chat` | `htmx.rs` | HTX-05 |
| 0.4 | Aggiungere tab Attività in community_detail | `community_detail.html` | COM-05 |
| 0.5 | Aggiungere tab Chat in community_detail | `community_detail.html` | COM-06 |

### Fase 1: Miglioramenti Navigazione

| # | Task | File | Test ID |
|---|------|------|---------|
| 1.1 | Aggiungere Dashboard link visibile in navbar | `base.html` | NAV-03 |
| 1.2 | Migliorare back link in proposal_detail | `proposal_detail.html` | BRD-03 |
| 1.3 | Aggiungere link Privacy/Terms in footer | `base.html` | FTR-02, FTR-03 |

### Fase 2: Completamento

| # | Task | File | Test ID |
|---|------|------|---------|
| 2.1 | Aggiungere tab Eventi | `community_detail.html` | COM-07 |
| 2.2 | Aggiungere tab POI | `community_detail.html` | COM-08 |
| 2.3 | Creare endpoint eventi | `htmx.rs` | HTX-06 |
| 2.4 | Creare endpoint POI | `htmx.rs` | HTX-07 |

---

## 6. Aggiornamento Test File

I seguenti test devono essere aggiunti a `view_interactions_test.rs`:

```rust
// Sezione 15: NAVIGATION UX TESTS
// Già aggiunti:
// - test_nav_navbar_has_communities_link
// - test_nav_navbar_has_chat_link
// - test_nav_navbar_no_global_businesses
// - test_nav_navbar_has_auth_section
// - test_nav_community_has_feed_tab
// - test_nav_community_has_members_tab
// - test_nav_community_has_governance_tab
// - test_nav_community_has_businesses_tab
// - test_nav_community_has_chat_tab
// - test_nav_htmx_community_businesses_endpoint
// - test_nav_htmx_community_chat_endpoint
// - test_nav_footer_has_language_switcher
// - test_nav_community_has_join_button

// DA AGGIUNGERE:
// - test_nav_navbar_has_dashboard_link
// - test_nav_footer_has_privacy_link
// - test_nav_footer_has_terms_link
// - test_nav_proposal_has_back_to_community
// - test_nav_post_has_breadcrumb
```

---

## 7. Conclusioni

### Stato Attuale
- **Test Passati**: 10/13 (77%)
- **Test Falliti**: 3/13 (23%)
- **Funzionalità Mancanti**: 4 critiche

### Azioni Immediate Richieste
1. ❌ Rimuovere link `/businesses` dalla navbar
2. ❌ Creare endpoint `/htmx/communities/{id}/businesses`
3. ❌ Creare endpoint `/htmx/communities/{id}/chat`
4. ❌ Aggiungere tab Attività e Chat in community_detail

### Verdict
**❌ NEEDS IMPLEMENTATION** - L'architettura di navigazione richiede modifiche strutturali per allinearsi al principio "Businesses appartengono alle Community".

---

*Documento generato da analisi completa dei 43 template HTML - Agent UX*
