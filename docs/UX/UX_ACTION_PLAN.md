# 🎯 Civiqo UX Action Plan

> Piano d'azione strutturato per allineare il progetto alle linee guida UX e Brand.

**Creato da**: Agente UX  
**Data**: 2025-11-27  
**Basato su**: UX_AUDIT_REPORT.md, UX_NAVIGATION_MATRIX.md, UX_USER_FLOWS_MASTER.md

---

## 📚 Documenti di Riferimento

> **Cartella**: `docs/UX/`

| Documento | Scopo | Quando Consultare |
|-----------|-------|-------------------|
| `UX_AUDIT_REPORT.md` | Analisi stato attuale | Prima di ogni sprint |
| `UX_NAVIGATION_MATRIX.md` | Mappa connessioni pagine | Quando si aggiunge navigazione |
| `UX_USER_FLOWS_MASTER.md` | Tutti i flussi utente | Quando si implementa un flow |
| `UX_MAP.md` | Overview pagine e stati | Reference generale |
| `UX_COMPONENTS.md` | Design system | Quando si crea UI |
| `UX_IMPLEMENTATION_TRACKER.md` | Tracker task UX | Durante implementazione |
| `../BRAND.md` | Linee guida brand | Sempre |

---

## 📋 Overview

Questo piano organizza le azioni UX in sprint gestibili, allineati con le fasi di sviluppo esistenti.

### Priorità
- 🔴 **P0**: Bloccante - Da fare immediatamente
- 🟠 **P1**: Critico - Prossimo sprint
- 🟡 **P2**: Importante - Entro 2 settimane
- 🟢 **P3**: Nice to have - Backlog

---

## 🚀 Sprint UX-1: Quick Wins (2 giorni)

### Obiettivo
Risolvere i problemi più evidenti con minimo sforzo.

### Tasks

#### UX-1.1: Fix Branding Base 🔴 P0
**File**: `src/server/templates/base.html`

- [ ] Sostituire logo CSS placeholder con SVG reale
- [ ] Aggiornare `<title>` default da "Community Manager" a "Civiqo"
- [ ] Aggiungere favicon con logo Civiqo
- [ ] Aggiungere skip link per accessibilità

**Dettagli implementazione**:
```html
<!-- Skip link (prima di tutto nel body) -->
<a href="#main-content" class="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 bg-civiqo-blue text-white px-4 py-2 rounded-lg z-50">
    Vai al contenuto principale
</a>

<!-- Logo reale -->
<img src="/static/images/civiqo_logo_full.svg" alt="Civiqo" class="h-8">

<!-- Main content id -->
<main id="main-content" class="container mx-auto px-4 py-8">
```

#### UX-1.2: Fix Titles 🔴 P0
**Files**: Tutti i template

- [ ] `index.html`: "Home - Civiqo"
- [ ] `dashboard.html`: "Dashboard - Civiqo"
- [ ] `communities.html`: "Community - Civiqo"
- [ ] `governance.html`: "Votazioni - Civiqo"
- [ ] Altri template...

#### UX-1.3: Error Pages 🟠 P1
**Nuovi file**:

- [ ] Creare `src/server/templates/404.html`
- [ ] Creare `src/server/templates/500.html`
- [ ] Registrare handler in Axum

**Design 404**:
```
┌─────────────────────────────────────┐
│         [Civiqo Logo]               │
│                                     │
│     🔍                              │
│     404                             │
│     Pagina non trovata              │
│                                     │
│     La pagina che cerchi non        │
│     esiste o è stata spostata.      │
│                                     │
│     [Torna alla Home]               │
│     [Cerca qualcosa]                │
└─────────────────────────────────────┘
```

#### UX-1.4: Focus States 🟠 P1
**File**: `src/server/static/styles/main.css`

```css
/* Focus visible per accessibilità */
*:focus-visible {
    outline: 2px solid var(--civiqo-blue);
    outline-offset: 2px;
}

/* Rimuovi outline default solo se focus-visible supportato */
@supports selector(:focus-visible) {
    *:focus:not(:focus-visible) {
        outline: none;
    }
}
```

### Deliverables Sprint UX-1
- [ ] Logo SVG integrato
- [ ] Tutti i title corretti
- [ ] Error pages funzionanti
- [ ] Focus states accessibili
- [ ] Skip link funzionante

### Acceptance Criteria
- [ ] Nessuna menzione di "Community Manager" visibile
- [ ] Logo reale visibile in navbar
- [ ] 404 page mostra design brand-compliant
- [ ] Tab navigation mostra focus ring

---

## 🚀 Sprint UX-2: Mobile & Navigation (3 giorni)

### Obiettivo
Rendere il sito usabile su mobile.

### Tasks

#### UX-2.1: Mobile Navigation 🔴 P0
**File**: `src/server/templates/base.html`

Implementare hamburger menu con Alpine.js:

```html
<!-- Mobile menu button -->
<button @click="mobileMenuOpen = !mobileMenuOpen" 
        class="md:hidden p-2"
        aria-label="Menu">
    <svg x-show="!mobileMenuOpen" class="w-6 h-6">...</svg>
    <svg x-show="mobileMenuOpen" class="w-6 h-6">...</svg>
</button>

<!-- Mobile menu panel -->
<div x-show="mobileMenuOpen" 
     x-transition
     class="md:hidden absolute top-full left-0 right-0 bg-white shadow-lg">
    <nav class="flex flex-col p-4 space-y-2">
        <a href="/communities">Communities</a>
        <a href="/governance">Votazioni</a>
        <a href="/chat">Chat</a>
    </nav>
</div>
```

#### UX-2.2: Mobile Search 🟠 P1
- [ ] Aggiungere icona search in mobile header
- [ ] Creare search overlay full-screen per mobile
- [ ] Mantenere funzionalità HTMX

#### UX-2.3: Touch Targets 🟡 P2
- [ ] Audit tutti i bottoni (min 44x44px)
- [ ] Aumentare padding su link navbar mobile
- [ ] Verificare tap targets in forms

#### UX-2.4: Mobile Tabs 🟡 P2
**File**: `community_detail.html`

- [ ] Rendere tabs scrollabili orizzontalmente
- [ ] Aggiungere indicatore scroll

### Deliverables Sprint UX-2
- [ ] Hamburger menu funzionante
- [ ] Search accessibile su mobile
- [ ] Touch targets >= 44px
- [ ] Tabs scrollabili

### Acceptance Criteria
- [ ] Navigazione completa possibile su mobile
- [ ] Nessun elemento "nascosto" senza alternativa
- [ ] Test su iPhone SE (320px) passa

---

## 🚀 Sprint UX-3: Onboarding Flow (5 giorni)

### Obiettivo
Guidare i nuovi utenti verso la prima azione significativa.

### Tasks

#### UX-3.1: Welcome Modal 🔴 P0
**Nuovo file**: `src/server/templates/fragments/welcome-modal.html`

Trigger: Primo login (flag in session/DB)

```
┌─────────────────────────────────────┐
│  Benvenuto su Civiqo! 🎉           │
├─────────────────────────────────────┤
│                                     │
│  Civiqo ti connette con la tua      │
│  comunità locale.                   │
│                                     │
│  Cosa vuoi fare?                    │
│                                     │
│  [🏘️ Esplora Community]            │
│  [👤 Completa Profilo]              │
│  [🗳️ Vedi Votazioni]               │
│                                     │
│           [Salta per ora]           │
└─────────────────────────────────────┘
```

#### UX-3.2: Profile Completion Prompt 🟠 P1
**File**: `dashboard.html`

Se profilo incompleto, mostrare banner:

```html
{% if profile_incomplete %}
<div class="bg-civiqo-yellow/10 border border-civiqo-yellow rounded-lg p-4 mb-6">
    <div class="flex items-center justify-between">
        <div>
            <h3 class="font-semibold text-civiqo-gray-900">Completa il tuo profilo</h3>
            <p class="text-sm text-civiqo-gray-600">Aggiungi una foto e una bio per farti conoscere</p>
        </div>
        <a href="/users/{{ user_id }}/edit" class="btn-primary">Completa</a>
    </div>
</div>
{% endif %}
```

#### UX-3.3: Community Suggestions 🟠 P1
**Nuovo endpoint**: `/htmx/onboarding/suggested-communities`

Logica:
1. Se utente ha località → community vicine
2. Altrimenti → community popolari
3. Max 3 suggerimenti

#### UX-3.4: First Action Celebration 🟡 P2
Quando utente completa prima azione (join, post, voto):
- Toast celebrativo
- Suggerimento prossima azione

### Deliverables Sprint UX-3
- [ ] Welcome modal funzionante
- [ ] Profile completion banner
- [ ] Community suggestions
- [ ] First action celebration

### Acceptance Criteria
- [ ] Nuovo utente vede welcome modal
- [ ] Profilo incompleto mostra prompt
- [ ] Almeno 3 community suggerite
- [ ] Feedback positivo dopo prima azione

---

## 🚀 Sprint UX-4: Notifications & Feedback (3 giorni)

### Obiettivo
Tenere gli utenti informati e fornire feedback immediato.

### Tasks

#### UX-4.1: Notifications Page 🔴 P0
**Nuovo file**: `src/server/templates/notifications.html`

- [ ] Lista tutte le notifiche
- [ ] Filtri: Tutte / Non lette / Per tipo
- [ ] Mark as read singola/tutte
- [ ] Pagination

#### UX-4.2: Toast System 🟠 P1
**Nuovo file**: `src/server/templates/fragments/toast.html`

Implementare con Alpine.js:

```html
<div x-data="{ toasts: [] }" 
     @toast.window="toasts.push($event.detail); setTimeout(() => toasts.shift(), 5000)"
     class="fixed bottom-4 right-4 z-50 space-y-2">
    <template x-for="toast in toasts">
        <div :class="toast.type === 'success' ? 'bg-civiqo-green' : 'bg-civiqo-coral'"
             class="text-white px-4 py-3 rounded-lg shadow-lg">
            <span x-text="toast.message"></span>
        </div>
    </template>
</div>
```

Trigger:
```javascript
window.dispatchEvent(new CustomEvent('toast', { 
    detail: { type: 'success', message: 'Operazione completata!' }
}));
```

#### UX-4.3: Loading States Consistenti 🟡 P2
**Nuovo file**: `src/server/templates/fragments/loading-skeleton.html`

Componente riutilizzabile per skeleton loading.

#### UX-4.4: Empty States 🟡 P2
**Nuovo file**: `src/server/templates/fragments/empty-state.html`

```html
<div class="text-center py-12">
    <div class="text-4xl mb-4">{{ icon }}</div>
    <h3 class="text-lg font-semibold text-civiqo-gray-900 mb-2">{{ title }}</h3>
    <p class="text-civiqo-gray-600 mb-4">{{ message }}</p>
    {% if cta_url %}
    <a href="{{ cta_url }}" class="btn-primary">{{ cta_text }}</a>
    {% endif %}
</div>
```

### Deliverables Sprint UX-4
- [ ] Notifications page
- [ ] Toast system globale
- [ ] Loading skeletons
- [ ] Empty states

---

## 🚀 Sprint UX-5: Accessibility (3 giorni)

### Obiettivo
Raggiungere WCAG 2.1 AA compliance.

### Tasks

#### UX-5.1: ARIA Labels 🔴 P0
Audit e fix di tutti i componenti interattivi:

- [ ] Bottoni icona: `aria-label`
- [ ] Form inputs: `aria-describedby` per errori
- [ ] Modal: `role="dialog"`, `aria-modal="true"`
- [ ] Tabs: `role="tablist"`, `role="tab"`, `aria-selected`

#### UX-5.2: Keyboard Navigation 🟠 P1
- [ ] Modal trap focus
- [ ] Dropdown navigabile con frecce
- [ ] Escape chiude modal/dropdown

#### UX-5.3: Screen Reader Testing 🟡 P2
- [ ] Test con VoiceOver (Mac)
- [ ] Verificare annunci corretti
- [ ] Fix problemi identificati

#### UX-5.4: Color Contrast 🟡 P2
- [ ] Audit con axe DevTools
- [ ] Fix testi grigi troppo chiari
- [ ] Verificare stati hover/focus

### Deliverables Sprint UX-5
- [ ] ARIA labels completi
- [ ] Keyboard navigation funzionante
- [ ] Contrast ratio >= 4.5:1
- [ ] Screen reader friendly

---

## 📊 Timeline Complessiva

```
Settimana 1:
├── Sprint UX-1: Quick Wins (2 giorni)
└── Sprint UX-2: Mobile & Navigation (3 giorni)

Settimana 2:
├── Sprint UX-3: Onboarding Flow (5 giorni)

Settimana 3:
├── Sprint UX-4: Notifications & Feedback (3 giorni)
└── Sprint UX-5: Accessibility (3 giorni)
```

---

## 🔄 Integrazione con Development Roadmap

### Allineamento con Phase 3 (User Profiles & Search)

Gli sprint UX si integrano con Phase 3:

| UX Sprint | Development Phase | Sinergia |
|-----------|-------------------|----------|
| UX-3 (Onboarding) | Profile completion | Stesso flusso |
| UX-4 (Notifications) | Notifications system | Stesso backend |
| UX-2 (Search) | Global search | Stesso endpoint |

### Raccomandazione
Eseguire **Sprint UX-1 e UX-2** prima di Phase 3, poi procedere in parallelo.

---

## 🗺️ Sprint UX-0: Mappatura e Governance (Continuo)

### Obiettivo
Mantenere la documentazione UX sempre aggiornata e tracciare tutte le connessioni.

### Documenti da Mantenere

| Documento | Frequenza Aggiornamento | Trigger |
|-----------|------------------------|---------|
| `UX_NAVIGATION_MATRIX.md` | Ad ogni nuova pagina/link | Nuova route o connessione |
| `UX_USER_FLOWS_MASTER.md` | Ad ogni nuovo flow | Nuova feature |
| `UX_MAP.md` | Settimanale | Review generale |
| `UX_COMPONENTS.md` | Ad ogni nuovo componente | Nuovo fragment |
| `UX_CHANGELOG.md` | Ad ogni modifica UX | Qualsiasi cambiamento |

### Processo per Nuova Feature

```
1. PRIMA dello sviluppo:
   └── Agente UX definisce flow in UX_USER_FLOWS_MASTER.md
   └── Agente UX aggiorna UX_NAVIGATION_MATRIX.md con nuove connessioni
   └── Agente UX crea specifiche UI

2. DURANTE lo sviluppo:
   └── Agent 1 segue specifiche
   └── Agent 1 segnala deviazioni

3. DOPO lo sviluppo:
   └── Agente UX verifica implementazione
   └── Agente UX aggiorna UX_MAP.md con stato reale
   └── Agente UX aggiorna UX_CHANGELOG.md
```

### Checklist Nuova Pagina

Quando si aggiunge una nuova pagina:

- [ ] Aggiungere a `UX_MAP.md` (sezione Pagine e Stati)
- [ ] Aggiungere a `UX_NAVIGATION_MATRIX.md` (matrice + entry/exit points)
- [ ] Verificare connessioni da/verso altre pagine
- [ ] Documentare stati UI (default, loading, empty, error)
- [ ] Aggiornare grafo navigazione se necessario

### Checklist Nuovo Flow

Quando si aggiunge un nuovo user flow:

- [ ] Creare entry in `UX_USER_FLOWS_MASTER.md`
- [ ] Definire tutti gli step con stato implementazione
- [ ] Identificare dipendenze da altri flows
- [ ] Definire metriche di successo
- [ ] Aggiornare matrice dipendenze

### Checklist Nuova Connessione

Quando si aggiunge un link tra pagine:

- [ ] Aggiornare matrice in `UX_NAVIGATION_MATRIX.md`
- [ ] Verificare bidirezionalità (se applicabile)
- [ ] Aggiornare entry/exit points delle pagine coinvolte
- [ ] Verificare che il flow sia documentato

---

## ✅ Definition of Done per ogni Sprint

- [ ] Tutti i task completati
- [ ] Code review passata
- [ ] Test manuali su desktop e mobile
- [ ] Nessuna regressione visiva
- [ ] **UX_NAVIGATION_MATRIX.md aggiornato**
- [ ] **UX_USER_FLOWS_MASTER.md aggiornato**
- [ ] UX_MAP.md aggiornato
- [ ] UX_CHANGELOG.md aggiornato

---

## 📝 Note per Agent 1 e Agent 2

### Per Agent 1 (Executor)
- Seguire le specifiche HTML/CSS fornite
- Mantenere consistenza con componenti esistenti
- Testare su mobile prima di PR

### Per Agent 2 (Verifier)
- Verificare brand compliance
- Controllare accessibility basics
- Invocare @Agente UX per review UI

---

**Prossimo Step**: Approvazione del piano e inizio Sprint UX-1

---

*Piano creato da Agente UX - Da rivedere settimanalmente*
