# UX Audit Report - Single Community Mode

**Data**: 2 Dicembre 2025  
**Contesto**: Refactoring da multi-community a single-community mode  
**Severità**: 🔴 Critico | 🟠 Alto | 🟡 Medio | 🟢 Basso

---

## Executive Summary

L'applicazione è stata progettata per un modello **multi-community** ma ora deve funzionare in **single-community mode**. Questo audit identifica tutte le view, i link e le aree che necessitano di refactoring.

### Statistiche
- **Template analizzati**: 43
- **Problemi critici**: 8
- **Dead links identificati**: 12+
- **Aree non dinamiche**: 15+
- **Template da rimuovere/deprecare**: 5
- **Template da modificare**: 18

---

## 🔴 PROBLEMI CRITICI (Bloccanti)

### 1. `community_home.html` - Errore Runtime
**File**: `src/server/templates/community_home.html`  
**Problema**: Il template usa ancora `| first` filter che causa errore 500
**Linea**: 28 (già fixata, ma potrebbero esserci altri problemi)
**Status**: ⚠️ Parzialmente risolto

**HTMX Endpoints non implementati**:
- `/htmx/community/{{ community.id }}/posts` - **NON ESISTE**
- `/htmx/community/{{ community.id }}/proposals` - **NON ESISTE**

```html
<!-- Linea 112 - DEAD LINK -->
<div hx-get="/htmx/community/{{ community.id }}/posts" hx-trigger="load">

<!-- Linea 128 - DEAD LINK -->
<div hx-get="/htmx/community/{{ community.id }}/proposals" hx-trigger="load">
```

**Fix richiesto**: Creare gli endpoint HTMX o usare quelli esistenti (`/htmx/communities/{id}/...`)

---

### 2. `base.html` - Navbar Multi-Community
**File**: `src/server/templates/base.html`  
**Problema**: La navbar contiene link a `/communities` che in single-community mode non ha senso

**Link problematici**:
- Linea 99: `<a href="/communities">` - Dovrebbe essere `/` o rimosso
- Linea 215-220: Mobile menu link a `/communities`

**Fix richiesto**: 
- Rimuovere link "Communities" dalla navbar
- Sostituire con link alla home della community

---

### 3. `index.html` - Template Multi-Community (OBSOLETO)
**File**: `src/server/templates/index.html`  
**Problema**: Questo template è per landing page multi-community, NON viene usato in single-community mode

**Contenuto obsoleto**:
- Hero section con "Esplora Communities"
- Features grid generico
- Link a `/communities`
- HTMX call a `/htmx/communities/recent`

**Status**: 🗑️ **DA DEPRECARE** - Il route `/` ora usa `community_home.html`

---

### 4. `communities.html` - Pagina Multi-Community (OBSOLETA)
**File**: `src/server/templates/communities.html`  
**Problema**: Intera pagina per listare/creare multiple communities

**Status**: 🗑️ **DA DEPRECARE** - Il route `/communities` ora fa redirect a `/`

---

### 5. `create_community.html` - Form Creazione Community (OBSOLETO)
**File**: `src/server/templates/create_community.html`  
**Problema**: Non serve in single-community mode

**Status**: 🗑️ **DA DEPRECARE** - La community viene creata nel setup wizard

---

## 🟠 PROBLEMI ALTI

### 6. `dashboard.html` vs `user_dashboard.html` - Duplicazione
**Problema**: Esistono DUE template dashboard

| Template | Uso | Contesto |
|----------|-----|----------|
| `dashboard.html` | Multi-community | Mostra "Le tue communities" |
| `user_dashboard.html` | Single-community | Mostra dati della singola community |

**Fix richiesto**: 
- Usare SOLO `user_dashboard.html` per single-community
- Deprecare `dashboard.html`
- Aggiornare handler `pages::dashboard` per usare `user_dashboard.html`

---

### 7. `governance.html` - Manca Community Context
**File**: `src/server/templates/governance.html`  
**Problema**: La pagina governance mostra un dropdown per selezionare community

**Linea 184-189**:
```html
<select name="community_id" required
        hx-get="/htmx/user/communities-options"
        hx-trigger="load">
    <option value="">{{ t.action_filter }}...</option>
</select>
```

**Fix richiesto**: 
- Rimuovere dropdown community
- Usare automaticamente l'unica community esistente
- Passare `community_id` dal backend

---

### 8. `chat_list.html` - Lista Chat Rooms Multi-Community
**File**: `src/server/templates/chat_list.html`  
**Problema**: Mostra lista di chat rooms per multiple communities

**Fix richiesto**:
- In single-community, dovrebbe mostrare direttamente la chat della community
- Oppure mostrare canali/stanze della singola community

---

### 9. `businesses.html` - Manca Community Context
**File**: `src/server/templates/businesses.html`  
**Problema**: Non filtra per community, mostra tutte le attività

**Fix richiesto**:
- Filtrare automaticamente per l'unica community
- Passare `community_id` dal backend

---

## 🟡 PROBLEMI MEDI

### 10. Dead Links nella Navbar
**File**: `base.html`

| Link | Destinazione | Problema |
|------|--------------|----------|
| `/communities` | Lista communities | Obsoleto in single-community |
| `/communities/create` | Crea community | Obsoleto |

---

### 11. Dead HTMX Endpoints

| Endpoint | Usato in | Status |
|----------|----------|--------|
| `/htmx/community/{id}/posts` | `community_home.html` | ❌ Non esiste |
| `/htmx/community/{id}/proposals` | `community_home.html` | ❌ Non esiste |
| `/htmx/user/communities-options` | `governance.html` | ❓ Da verificare |
| `/htmx/stats/proposals` | `user_dashboard.html` | ❌ Non esiste |
| `/htmx/stats/messages` | `user_dashboard.html` | ❌ Non esiste |

---

### 12. Testi Hardcoded (Non i18n)

| File | Linea | Testo |
|------|-------|-------|
| `businesses.html` | 10 | "Attività Locali" |
| `businesses.html` | 11 | "Scopri e supporta le attività..." |
| `businesses.html` | 19 | "Aggiungi Attività" |
| `businesses.html` | 34 | "Cerca attività..." |
| `businesses.html` | 48-56 | Categorie dropdown |
| `chat_list.html` | 8 | "Chat Rooms" |
| `chat_list.html` | 9 | "Seleziona una community..." |
| `governance.html` | 37-66 | "Approvate", "Partecipanti", "In Scadenza" |

---

### 13. `community_detail.html` - Obsoleto?
**File**: `src/server/templates/community_detail.html`  
**Problema**: In single-community mode, la home È il detail della community

**Status**: ❓ Da valutare se serve ancora

---

### 14. Quick Actions Links
**File**: `user_dashboard.html`

| Link | Destinazione | Problema |
|------|--------------|----------|
| `/governance/proposals/create` | Crea proposta | Route non esiste |
| `/profile` | Profilo utente | Dovrebbe essere `/users/{id}` |

---

## 🟢 PROBLEMI BASSI

### 15. Fragments da Verificare

| Fragment | Usato | Status |
|----------|-------|--------|
| `fragments/community-card.html` | Liste communities | Obsoleto in single-community |
| `fragments/join-button.html` | Join community | Obsoleto |
| `fragments/members-list.html` | Lista membri | OK, serve ancora |

---

### 16. Template Admin
**File**: `admin.html`, `admin/instance_settings.html`  
**Status**: ✅ OK - Servono per gestione istanza

---

### 17. Template Errore
**File**: `404.html`, `500.html`  
**Status**: ✅ OK

---

## Piano d'Azione Consigliato

### Fase 1: Fix Critici (Immediato) ✅ COMPLETATO
1. ✅ Fix filtro `| first` in tutti i template
2. ✅ Fix URL HTMX in `community_home.html` (usano ora `/htmx/communities/{id}/feed` e `/htmx/communities/{id}/proposals`)
3. ✅ Handler `pages::index` già passa dati corretti

### Fase 2: Refactoring Navbar (1-2 ore) ✅ COMPLETATO
1. ✅ Sostituito link `/communities` con `/` (Home) in `base.html`
2. ✅ Aggiunto link `/businesses` (Attività) nella navbar
3. ✅ Aggiornato mobile menu con stessa struttura

### Fase 3: Consolidamento Dashboard (1-2 ore) ✅ COMPLETATO
1. ✅ Handler `dashboard` già usa `user_dashboard.html`
2. ✅ `dashboard.html` può essere deprecato
3. 🔲 Creare endpoint `/htmx/stats/proposals` (nice-to-have)
4. 🔲 Creare endpoint `/htmx/stats/messages` (nice-to-have)

### Fase 4: Refactoring Governance (1 ora) ✅ COMPLETATO
1. ✅ Rimosso dropdown community selection da `governance.html`
2. ✅ Handler `governance` ora passa `community` nel context
3. ✅ Form usa hidden input con `community.id`

### Fase 5: Cleanup (30 min) - PENDING
1. 🔲 Deprecare `index.html` (non usato, `/` usa `community_home.html`)
2. 🔲 Deprecare `communities.html` (redirect a `/`)
3. 🔲 Deprecare `create_community.html` (setup wizard)
4. 🔲 Aggiornare i18n per testi hardcoded

---

## Matrice Navigazione Single-Community

```
                    ┌─────────────────┐
                    │   / (Home)      │
                    │ community_home  │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│  /governance  │   │    /chat      │   │  /businesses  │
│   Proposte    │   │  Chat Room    │   │   Attività    │
└───────────────┘   └───────────────┘   └───────────────┘
        │
        ▼
┌───────────────┐
│/governance/:id│
│Dettaglio Prop │
└───────────────┘

                    ┌─────────────────┐
                    │   /dashboard    │
                    │ user_dashboard  │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│   /profile    │   │/admin/settings│   │   /chat       │
│  (se admin)   │   │  (se admin)   │   │               │
└───────────────┘   └───────────────┘   └───────────────┘
```

---

## Routes da Modificare

| Route | Attuale | Nuovo Comportamento |
|-------|---------|---------------------|
| `/` | `index.html` | `community_home.html` ✅ |
| `/communities` | `communities.html` | Redirect a `/` ✅ |
| `/communities/create` | `create_community.html` | Redirect a `/` o 404 |
| `/dashboard` | `dashboard.html` | `user_dashboard.html` |
| `/governance` | Dropdown community | Auto-select unica community |

---

## Conclusioni

Il refactoring a single-community richiede modifiche significative ma gestibili. I problemi critici sono concentrati in:

1. **HTMX endpoints mancanti** per `community_home.html`
2. **Navbar** con link obsoleti
3. **Dashboard** duplicato
4. **Governance** con dropdown inutile

Stima tempo totale: **4-6 ore** di sviluppo per completare il refactoring.
