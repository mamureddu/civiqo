# Civiqo UX Changelog

> Storico delle modifiche UX con impatto su navigazione, componenti e flussi.

**Maintainer**: Agente UX

---

## [1.2.0] - 2025-11-27

### 🗺️ Navigation & Flows Mapping

#### Added
- `UX_NAVIGATION_MATRIX.md` - Matrice completa connessioni tra pagine
- `UX_USER_FLOWS_MASTER.md` - Documento master di tutti i flussi utente

#### Changed
- `UX_ACTION_PLAN.md` - Aggiunto Sprint UX-0 per governance continua
- Aggiunte checklist per nuove pagine, flows e connessioni
- Aggiornata Definition of Done con documenti UX

#### Navigation Gaps Identified
- ❌ Notifications page mancante
- ❌ Search results page mancante
- ❌ Proposal detail page mancante
- ❌ Mobile navigation (hamburger) mancante

#### Flows Documented
- 17 user flows mappati
- 5 flows completi (✅)
- 8 flows parziali (⚠️)
- 4 flows mancanti (❌)

---

## [1.1.0] - 2025-11-27

### 📊 UX Audit Complete

#### Added
- `UX_AUDIT_REPORT.md` - Analisi completa stato UX
- `UX_ACTION_PLAN.md` - Piano d'azione strutturato in sprint

#### Changed
- `UX_MAP.md` - Aggiornato con stato reale implementazione
- Aggiunta legenda stati (✅ ⚠️ ❌ 🔄)
- Identificate pagine mancanti

#### Identified Issues
- **Brand**: Logo placeholder, title "Community Manager"
- **Navigation**: Mobile menu mancante
- **Accessibility**: Skip links, focus states, ARIA labels
- **Flows**: Onboarding assente, notifications page mancante

#### Action Plan Created
- Sprint UX-1: Quick Wins (2 giorni)
- Sprint UX-2: Mobile & Navigation (3 giorni)
- Sprint UX-3: Onboarding Flow (5 giorni)
- Sprint UX-4: Notifications & Feedback (3 giorni)
- Sprint UX-5: Accessibility (3 giorni)

---

## [1.0.0] - 2025-11-27

### 🎉 Initial Release

#### Pagine Implementate
- **Landing** (`/`) - Hero + feature highlights
- **Dashboard** (`/dashboard`) - Hub personale con widget
- **Communities List** (`/communities`) - Griglia community
- **Community Detail** (`/communities/:id`) - Tab: Feed, Members, Votazioni
- **Governance** (`/governance`) - Lista globale proposte
- **Chat List** (`/chat`) - Lista conversazioni
- **Chat Room** (`/chat/:room_id`) - Messaggistica real-time
- **Profile View** (`/users/:id`) - Profilo pubblico
- **Profile Edit** (`/users/:id/edit`) - Modifica profilo
- **Businesses** (`/businesses`) - Directory attività
- **Business Detail** (`/businesses/:id`) - Dettaglio attività
- **POI** (`/poi`) - Mappa punti interesse

#### Componenti Creati
- Navbar con navigazione condizionale (auth/anon)
- Community Card
- Proposal Card
- Post Card
- Modal Dialog
- Tab Navigation
- Form Elements (input, textarea, select)
- Buttons (primary, secondary, danger, ghost)
- Badges (status, counter)
- Loading States (spinner, skeleton)
- Empty States
- Toast Notifications

#### Flussi Documentati
- F01: Onboarding
- F02: Community Discovery
- F03: Community Participation
- F04: Governance & Voting
- F05: Chat & Messaging (parziale)
- F06: Profile Management

#### Integrazioni
- Auth0 per autenticazione
- HTMX per interazioni dinamiche
- Alpine.js per stato UI locale
- TailwindCSS per styling

---

## Template Entry

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- Nuove feature o componenti

### Changed
- Modifiche a feature esistenti

### Fixed
- Bug fix UX

### Deprecated
- Feature che verranno rimosse

### Removed
- Feature rimosse

### Breaking Changes
- Modifiche che rompono compatibilità

### Migration Notes
- Istruzioni per aggiornamento
```

---

## Convenzioni Versioning

- **Major (X)**: Redesign significativo, breaking changes
- **Minor (Y)**: Nuove feature, nuovi flussi
- **Patch (Z)**: Bug fix, micro-miglioramenti

---

*Documento mantenuto da Agente UX.*
