# UX Navigation Audit - Civiqo

**Data**: 2024-12-01  
**Versione**: 1.0  
**Stato**: 🔴 CRITICAL ISSUES FOUND

---

## Executive Summary

L'analisi ha rivelato **problemi strutturali significativi** nella navigazione dell'applicazione. Il problema principale è che **Businesses e Governance sono trattati come entità globali invece che come funzionalità interne alle Community**.

### Problemi Critici Identificati

1. **Businesses** (`/businesses`) - Dovrebbe essere **dentro la Community**
2. **Governance** (`/governance`) - Dovrebbe essere **dentro la Community**  
3. **Chat** (`/chat`) - Ambiguo: globale o per community?
4. **Mancano link di navigazione** tra sezioni correlate

---

## 1. Architettura Informativa Corretta (TO-BE)

### Gerarchia Logica

```
HOME (/)
├── COMMUNITIES (/communities)
│   └── COMMUNITY DETAIL (/communities/{id})
│       ├── Feed (tab)
│       ├── Membri (tab)
│       ├── Info (tab)
│       ├── Governance/Votazioni (tab) ← CORRETTO, già presente
│       ├── Attività Locali (tab) ← MANCANTE!
│       ├── Chat Community (tab) ← MANCANTE!
│       └── Eventi (tab) ← MANCANTE!
│
├── DASHBOARD (/dashboard) - Area personale utente
│   ├── Le mie Community
│   ├── Proposte attive (dalle mie community)
│   ├── Notifiche
│   └── Attività recente
│
├── PROFILO (/users/{id})
│   ├── Info personali
│   ├── Community di appartenenza
│   └── Attività
│
└── ADMIN (/admin) - Solo admin
    ├── Moderazione
    ├── Analytics
    └── Audit Logs
```

### Cosa NON Dovrebbe Essere Globale

| Entità | Stato Attuale | Stato Corretto |
|--------|---------------|----------------|
| Businesses | `/businesses` (globale) | `/communities/{id}?tab=businesses` |
| Governance | `/governance` (globale) | `/communities/{id}?tab=governance` |
| Chat | `/chat` (globale) | `/communities/{id}/chat` o tab |
| POI | `/poi` (globale) | `/communities/{id}?tab=poi` |

---

## 2. Matrice di Navigazione Completa

### 2.1 Navbar Globale

| Link | Priorità | Visibilità | Stato | Note |
|------|----------|------------|-------|------|
| Home | P1 | Tutti | ✅ OK | Logo cliccabile |
| Communities | P1 | Tutti | ✅ OK | Lista community |
| ~~Businesses~~ | - | - | ❌ RIMUOVERE | Spostare dentro community |
| ~~Governance~~ | - | - | ❌ RIMUOVERE | Spostare dentro community |
| Chat | P2 | Logged | ⚠️ RIVEDERE | Dovrebbe mostrare chat delle community |
| Search | P1 | Tutti | ✅ OK | Ricerca globale |
| Notifications | P2 | Logged | ✅ OK | Dropdown |
| Profile | P2 | Logged | ✅ OK | Avatar + dropdown |
| Dashboard | P2 | Logged | ✅ OK | Area personale |
| Login | P1 | Guest | ✅ OK | |

### 2.2 Community Detail - Tabs

| Tab | Priorità | Visibilità | Stato | Note |
|-----|----------|------------|-------|------|
| Feed | P1 | Tutti | ✅ OK | Post della community |
| Membri | P2 | Tutti | ✅ OK | Lista membri |
| Info | P3 | Tutti | ✅ OK | Descrizione, regole |
| Governance | P1 | Tutti | ✅ OK | Proposte e votazioni |
| **Attività** | P1 | Tutti | ❌ MANCANTE | Businesses locali |
| **Chat** | P2 | Membri | ❌ MANCANTE | Chat community |
| **Eventi** | P2 | Tutti | ❌ MANCANTE | Calendario eventi |
| **POI** | P3 | Tutti | ❌ MANCANTE | Punti di interesse |

### 2.3 Dashboard Utente

| Sezione | Priorità | Stato | Note |
|---------|----------|-------|------|
| Welcome | P1 | ✅ OK | |
| Stats | P2 | ✅ OK | |
| Proposte Attive | P1 | ✅ OK | Link a governance |
| Le mie Community | P1 | ✅ OK | |
| Attività Recente | P2 | ✅ OK | |
| **Quick Actions** | P1 | ⚠️ INCOMPLETO | Mancano azioni rapide |

---

## 3. Analisi Pagina per Pagina

### 3.1 `base.html` - Navbar

**Stato**: ⚠️ DA CORREGGERE

**Link Presenti**:
- ✅ Communities
- ❌ Businesses (da rimuovere dalla navbar globale)
- ❌ Governance (da rimuovere dalla navbar globale)
- ✅ Chat (ma da rivedere logica)
- ✅ Search
- ✅ Notifications
- ✅ Profile/Login

**Azioni Richieste**:
1. Rimuovere link "Attività" dalla navbar globale
2. Rimuovere link "Governance" dalla navbar globale
3. Mantenere Chat ma collegarlo alle chat delle community dell'utente

---

### 3.2 `communities.html` - Lista Community

**Stato**: ✅ OK (dopo fix recenti)

**Link Presenti**:
- ✅ Card community → `/communities/{id}`
- ✅ Crea community → `/communities/create`
- ✅ Filtri funzionanti

**Mancanti**:
- ⚠️ Nessun link rapido a "Le mie community"

---

### 3.3 `community_detail.html` - Dettaglio Community

**Stato**: ⚠️ INCOMPLETO

**Tabs Presenti**:
- ✅ Feed
- ✅ Membri  
- ✅ Info
- ✅ Governance/Votazioni

**Tabs Mancanti**:
- ❌ **Attività Locali** (Businesses)
- ❌ **Chat** (per membri)
- ❌ **Eventi**
- ❌ **POI/Mappa**

**Link Interni**:
- ✅ Nuovo Post → `/communities/{id}/posts/new`
- ✅ Nuova Proposta → Modal
- ❌ Nuova Attività → MANCANTE
- ❌ Chat Community → MANCANTE

---

### 3.4 `businesses.html` - Lista Attività (GLOBALE)

**Stato**: ❌ ARCHITETTURA ERRATA

Questa pagina **non dovrebbe esistere come entità globale**.

**Problema**: Le attività commerciali appartengono a una community specifica, non sono globali.

**Soluzione**: 
1. Rimuovere `/businesses` come route globale
2. Aggiungere tab "Attività" in `community_detail.html`
3. Route: `/communities/{id}?tab=businesses`

---

### 3.5 `governance.html` - Governance (GLOBALE)

**Stato**: ⚠️ PARZIALMENTE CORRETTO

**Uso Corretto**: Aggregatore di proposte dalle community dell'utente
**Uso Errato**: Governance globale senza contesto community

**Soluzione**:
- Mantenere come **aggregatore** per utente loggato
- Mostrare proposte raggruppate per community
- Link diretto alla community per ogni proposta

---

### 3.6 `dashboard.html` - Dashboard Utente

**Stato**: ⚠️ INCOMPLETO

**Sezioni Presenti**:
- ✅ Welcome
- ✅ Stats
- ✅ Proposte Attive
- ✅ Le mie Community
- ✅ Attività Recente

**Mancanti**:
- ❌ **Quick Actions Panel** con:
  - Crea Post (in quale community?)
  - Vai alle Chat
  - Le mie Attività (businesses che gestisco)
- ❌ **Notifiche non lette** (preview)

---

### 3.7 `proposal_detail.html` - Dettaglio Proposta

**Stato**: ✅ OK (dopo fix recenti)

**Link Presenti**:
- ✅ Breadcrumb → Governance → Proposta
- ✅ Link Community
- ✅ Pulsanti voto (condizionali)

**Mancanti**:
- ⚠️ Link "Torna alla Community" più evidente

---

### 3.8 `create_post.html` - Crea Post

**Stato**: ✅ OK (dopo fix recenti)

**Link Presenti**:
- ✅ Breadcrumb completo
- ✅ Info community

---

### 3.9 `chat_list.html` / `chat.html`

**Stato**: ⚠️ DA RIVEDERE

**Problema**: La chat dovrebbe essere contestualizzata per community.

**Soluzione**:
- Chat list mostra le chat delle community di cui sono membro
- Ogni community ha la sua chat room

---

## 4. Piano d'Azione Prioritizzato

### Fase 1: Fix Critici (P0) 🔴

| # | Task | File | Effort |
|---|------|------|--------|
| 1.1 | Rimuovere "Attività" dalla navbar globale | `base.html` | S |
| 1.2 | Rimuovere "Governance" dalla navbar globale | `base.html` | S |
| 1.3 | Aggiungere tab "Attività" in community_detail | `community_detail.html` | M |
| 1.4 | Creare fragment per businesses di community | `fragments/community-businesses.html` | M |

### Fase 2: Miglioramenti Navigazione (P1) 🟡

| # | Task | File | Effort |
|---|------|------|--------|
| 2.1 | Aggiungere tab "Chat" in community_detail | `community_detail.html` | M |
| 2.2 | Collegare chat globale alle community | `chat_list.html` | M |
| 2.3 | Migliorare dashboard con quick actions | `dashboard.html` | M |
| 2.4 | Aggiungere "Torna alla Community" in proposal_detail | `proposal_detail.html` | S |

### Fase 3: Completamento (P2) 🟢

| # | Task | File | Effort |
|---|------|------|--------|
| 3.1 | Aggiungere tab "Eventi" in community_detail | `community_detail.html` | L |
| 3.2 | Aggiungere tab "POI/Mappa" in community_detail | `community_detail.html` | L |
| 3.3 | Rivedere governance.html come aggregatore | `governance.html` | M |

---

## 5. Specifiche UI per Nuovi Tab Community

### Tab "Attività Locali"

```
┌─────────────────────────────────────────────────────────────┐
│ [Feed] [Membri] [Info] [Votazioni] [🏪 Attività] [Chat]    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 🔍 Cerca attività...          [Categoria ▼] [+ Nuova]│   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ 🍕 Pizzeria │ │ 🛒 Market   │ │ 💇 Parrucch │          │
│  │ Da Mario    │ │ Bio Local   │ │ Style       │          │
│  │ ⭐⭐⭐⭐⭐    │ │ ⭐⭐⭐⭐☆    │ │ ⭐⭐⭐⭐⭐    │          │
│  │ Via Roma 12 │ │ Via Dante 5 │ │ P.zza Unità │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Tab "Chat"

```
┌─────────────────────────────────────────────────────────────┐
│ [Feed] [Membri] [Info] [Votazioni] [Attività] [💬 Chat]    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Chat della Community "Nome Community"                      │
│  ─────────────────────────────────────                      │
│                                                             │
│  [Mario] Ciao a tutti! 🎉                         14:32    │
│  [Lucia] Benvenuto Mario!                         14:33    │
│  [Admin] Ricordo che domani c'è l'assemblea       14:35    │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Scrivi un messaggio...                    [Invia]   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ⚠️ Solo i membri possono partecipare alla chat            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Policy di Visibilità

### Legenda

| Simbolo | Significato |
|---------|-------------|
| 👁️ | Visibile a tutti |
| 🔐 | Solo utenti loggati |
| 👥 | Solo membri community |
| 👑 | Solo admin/owner community |
| 🛡️ | Solo admin sistema |

### Matrice Visibilità

| Elemento | Guest | Logged | Membro | Owner | Admin |
|----------|-------|--------|--------|-------|-------|
| Lista Community | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Dettaglio Community | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Feed Community | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Crea Post | ❌ | ❌ | 👥 | 👑 | 🛡️ |
| Votazioni (view) | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Votazioni (vote) | ❌ | ❌ | 👥 | 👑 | 🛡️ |
| Crea Proposta | ❌ | ❌ | 👥 | 👑 | 🛡️ |
| Attività (view) | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Crea Attività | ❌ | 🔐 | 🔐 | 🔐 | 🛡️ |
| Chat Community | ❌ | ❌ | 👥 | 👑 | 🛡️ |
| Membri (view) | 👁️ | 👁️ | 👁️ | 👁️ | 👁️ |
| Gestione Membri | ❌ | ❌ | ❌ | 👑 | 🛡️ |
| Dashboard | ❌ | 🔐 | 🔐 | 🔐 | 🔐 |
| Admin Panel | ❌ | ❌ | ❌ | ❌ | 🛡️ |

---

## 7. Conclusioni

### Verdict: ❌ NEEDS MAJOR REVISION

L'architettura attuale presenta **problemi strutturali** che compromettono l'esperienza utente:

1. **Businesses e Governance come entità globali** violano il principio di "Identità Locale" del brand
2. **Mancano tab essenziali** nella community detail
3. **La navigazione è frammentata** tra pagine che dovrebbero essere contestuali

### Raccomandazione

Procedere con la **Fase 1** immediatamente per correggere l'architettura informativa prima di aggiungere nuove funzionalità.

---

*Documento generato da Agent UX - Civiqo*
