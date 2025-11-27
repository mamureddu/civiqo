# Two-Agent Development Workflow

## Overview

Workflow collaborativo tra due agenti specializzati per lo sviluppo del Community Manager project.

### Architecture Context
- **Backend**: Rust (Axum) + SQLx
- **Frontend**: HTMX + TailwindCSS + Alpine.js
- **Database**: CockroachDB Cloud
- **Auth**: Auth0 OAuth2
- **Working Dir**: `/Users/mariomureddu/CascadeProjects/community-manager/src`

### Brand Guidelines
**MANDATORY**: `brand_id/Civiqo_Brand_Book_v1.1.pdf`
- Assets: `civiqo_assets_structured/`

---

## 🔄 WORKFLOW LIFECYCLE

```
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 0: PLANNING                            │
│                    (Agent 2 - Tech Lead)                        │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ 1. Analizza richiesta utente                            │   │
│  │ 2. Verifica stato esistente (DB, handlers, templates)   │   │
│  │ 3. Crea Phase Specification Document                    │   │
│  │ 4. Definisce KPI, deliverables, test plan               │   │
│  │ 5. Identifica rischi e blockers                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           ↓                                     │
│              OUTPUT: Specification Document                     │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 1: IMPLEMENTATION                      │
│                    (Agent 1 - Executor)                         │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ 1. Legge specifiche Agent 2                             │   │
│  │ 2. Implementa Model (migration)                         │   │
│  │ 3. Implementa Controller (handlers)                     │   │
│  │ 4. Implementa View (templates)                          │   │
│  │ 5. Scrive test                                          │   │
│  │ 6. Verifica checklist                                   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           ↓                                     │
│              OUTPUT: Feature completa + test                    │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 2: REVIEW                              │
│                    (Agent 2 - Tech Lead)                        │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ 1. Build & test verification                            │   │
│  │ 2. Security review                                      │   │
│  │ 3. Code quality check                                   │   │
│  │ 4. Brand compliance                                     │   │
│  │ 5. Test coverage                                        │   │
│  │ 6. Manual testing                                       │   │
│  │ 7. Decision: APPROVE / CHANGES / REJECT                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           ↓                                     │
│              OUTPUT: Review decision                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 👤 Agent 1: Fullstack Executor

### Responsabilità
- Implementare features seguendo le specifiche di Agent 2
- Scrivere codice production-ready con test
- Rispettare brand guidelines e pattern esistenti

### Input Richiesto
**Phase Specification Document** da Agent 2 con:
- Obiettivo della fase
- KPI di successo
- Deliverables (Model, Controller, View, Test)
- Regole di business
- Piano di testing

### Output Richiesto
- Migration file (se necessario)
- Handlers (API + HTMX)
- Templates (pages + fragments)
- Test (unit + integration + view interaction)
- Routes registrate

### Definition of Done
- [ ] `cargo build --workspace` passa
- [ ] `cargo test --workspace` passa
- [ ] Tutti i KPI raggiunti
- [ ] Checklist pre-consegna completata

### Quando Escalare
- Schema DB ambiguo
- Conflitto con pattern esistenti
- Requisito non chiaro
- Blocco tecnico >15 min

---

## 👤 Agent 2: Tech Lead Verifier

### Responsabilità
- **Phase 0**: Creare specifiche dettagliate per Agent 1
- **Phase 2**: Verificare qualità e approvare/rifiutare

### Phase 0 Output: Specification Document

```markdown
# 📋 Phase [N]: [Feature] - Specifiche per Agent 1

## 🎯 Obiettivo della Fase
[Descrizione]

## 📊 KPI di Successo
| Metrica | Target | Verifica |
|---------|--------|----------|
| Build | 0 errori | cargo build |
| Test | 100% pass | cargo test |
| Nuovi Test | ≥10 | count |

## 🗂️ Deliverables Richiesti
### 1. Database (Model)
### 2. API Handlers (Controller)
### 3. Templates (View)
### 4. Routes
### 5. Test

## 🔗 Integrazione con Esistente
### Cosa Esiste Già
### Pattern da Seguire
### File da NON Modificare

## 📝 Regole di Business

## 🧪 Piano di Testing
### Unit Test
### Integration Test
### View Interaction Test

## ⏱️ Timeline Suggerita

## ✅ Checklist Pre-Consegna

## 🚨 Blockers Potenziali

## 📞 Escalation
```

### Phase 2: Review Checklist
1. Build & test (5 min)
2. Security review (10 min)
3. Code quality (10 min)
4. Brand compliance (5 min)
5. Test coverage (10 min)
6. Manual testing (10 min)

### Decision Framework
- ✅ **APPROVE**: Tutti i criteri soddisfatti
- ⚠️ **CHANGES**: Issue minori da correggere
- ❌ **REJECT**: Problemi critici (security, build fail)

---

## 📋 HANDOFF CRITERIA

### Phase 0 → Phase 1 (Agent 2 → Agent 1)
- [ ] Specification document completo
- [ ] KPI definiti
- [ ] Deliverables chiari
- [ ] Rischi identificati

### Phase 1 → Phase 2 (Agent 1 → Agent 2)
- [ ] Build passa
- [ ] Test passano
- [ ] Checklist completata
- [ ] Ready for review

### Phase 2 → Done (Agent 2 → User)
- [ ] Review completata
- [ ] Decision documentata
- [ ] Feedback fornito

---

## 🎯 SUCCESS METRICS

| Metrica | Target |
|---------|--------|
| Build errors | 0 |
| Test pass rate | 100% |
| Review time | <30 min |
| Approval rate | >70% |
| Security issues | 0 |
| Brand compliance | 100% |

---

## 📚 REFERENCE FILES

| Documento | Scopo |
|-----------|-------|
| `docs/AGENT_1_EXECUTOR_PROMPT.md` | Guida completa Agent 1 |
| `docs/AGENT_2_VERIFIER_PROMPT.md` | Guida completa Agent 2 |
| `brand_id/Civiqo_Brand_Book_v1.1.pdf` | Brand guidelines |
| `PROJECT_ROADMAP_FINAL.md` | Roadmap progetto |

---

**Questo workflow garantisce sviluppo di alta qualità attraverso responsabilità chiare, processi sistematici e collaborazione continua tra agenti specializzati.**
