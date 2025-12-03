# 📋 Phase: Membership System - Specifiche per Agent 1

**Data**: 2 Dicembre 2025  
**Status**: 🚀 Ready for Implementation  
**Autore**: Agent 2 (Tech Lead)

---

## 🎯 Obiettivo della Fase

Implementare un sistema completo di membership per le community che gestisca:
1. Iscrizione diretta (community pubbliche)
2. Richieste di iscrizione (community private)
3. Approvazione/rifiuto richieste (admin)
4. Configurazione membership da admin
5. UI dinamica basata su stato membership

---

## 📊 KPI di Successo

| Metrica | Target | Verifica |
|---------|--------|----------|
| Build | 0 errori | `cargo build --workspace` |
| Test | 100% pass | `cargo test --workspace` |
| Nuovi endpoint | 6 | count |
| Nuovi template/fragment | 3 | count |
| Migration applicata | ✅ | DB check |

---

## 🗂️ Deliverables Richiesti

### 1. Database (Model)

#### 1.1 Nuove colonne in `communities`
```sql
-- Già esistenti (verificare):
-- is_public BOOLEAN DEFAULT true
-- requires_approval BOOLEAN DEFAULT false

-- Da aggiungere:
ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  membership_type VARCHAR(20) DEFAULT 'public' 
  CHECK (membership_type IN ('public', 'private', 'hybrid'));

ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  default_member_role member_role DEFAULT 'member';
```

#### 1.2 Nuova tabella `membership_requests`
```sql
CREATE TABLE IF NOT EXISTS membership_requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
  requested_role member_role DEFAULT 'member',
  message TEXT,
  reviewed_by UUID REFERENCES users(id),
  reviewed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(community_id, user_id)
);

CREATE INDEX idx_membership_requests_community ON membership_requests(community_id);
CREATE INDEX idx_membership_requests_user ON membership_requests(user_id);
CREATE INDEX idx_membership_requests_status ON membership_requests(status);
```

### 2. API Handlers (Controller)

#### 2.1 Endpoint esistenti da verificare
- `POST /api/communities/{id}/join` ✅ Esiste
- `POST /api/communities/{id}/leave` ✅ Esiste

#### 2.2 Nuovi endpoint da creare

| Method | Endpoint | Handler | Descrizione |
|--------|----------|---------|-------------|
| POST | `/api/communities/{id}/request-join` | `request_join_community` | Richiesta iscrizione (private) |
| GET | `/api/communities/{id}/requests` | `list_join_requests` | Lista richieste (admin) |
| POST | `/api/communities/{id}/requests/{user_id}/approve` | `approve_join_request` | Approva richiesta |
| POST | `/api/communities/{id}/requests/{user_id}/reject` | `reject_join_request` | Rifiuta richiesta |
| PUT | `/api/communities/{id}/settings/membership` | `update_membership_settings` | Aggiorna config (admin) |

#### 2.3 HTMX Handlers

| Method | Endpoint | Handler | Descrizione |
|--------|----------|---------|-------------|
| POST | `/htmx/communities/{id}/join` | `join_community_htmx` | ✅ Già implementato |
| POST | `/htmx/communities/{id}/request` | `request_join_htmx` | Richiesta con feedback HTML |
| GET | `/htmx/communities/{id}/membership-button` | `membership_button_htmx` | Pulsante dinamico |

### 3. Templates (View)

#### 3.1 Fragment: `fragments/membership-button.html`
Pulsante dinamico che mostra stato corretto:
- Non loggato → "Accedi per iscriverti"
- Loggato, non membro, public → "Iscriviti"
- Loggato, non membro, private → "Richiedi accesso"
- Richiesta pending → "Richiesta in attesa"
- Membro → Badge "Membro" + ruolo

#### 3.2 Fragment: `fragments/membership-requests.html`
Lista richieste per admin con azioni approve/reject

#### 3.3 Sezione Admin: Membership Settings
In `/admin/settings` o pagina dedicata:
- Tipo community (public/private/hybrid)
- Ruolo default
- Richiede approvazione

### 4. Routes

```rust
// In main.rs - già esistenti
.route("/api/communities/{id}/join", post(api::join_community))
.route("/api/communities/{id}/leave", post(api::leave_community))

// Da aggiungere
.route("/api/communities/{id}/request-join", post(api::request_join_community))
.route("/api/communities/{id}/requests", get(api::list_join_requests))
.route("/api/communities/{id}/requests/{user_id}/approve", post(api::approve_join_request))
.route("/api/communities/{id}/requests/{user_id}/reject", post(api::reject_join_request))
.route("/api/communities/{id}/settings/membership", put(api::update_membership_settings))

// HTMX
.route("/htmx/communities/{id}/join", post(htmx::join_community_htmx)) // ✅ Già fatto
.route("/htmx/communities/{id}/request", post(htmx::request_join_htmx))
.route("/htmx/communities/{id}/membership-button", get(htmx::membership_button_htmx))
```

### 5. Test

#### Unit Test
- `test_join_public_community`
- `test_join_private_community_fails`
- `test_request_join_private_community`
- `test_approve_join_request`
- `test_reject_join_request`
- `test_membership_settings_update`

#### Integration Test
- Flusso completo iscrizione pubblica
- Flusso completo richiesta/approvazione
- Verifica permessi admin

---

## 🔗 Integrazione con Esistente

### Cosa Esiste Già
- Tabella `communities` con `is_public`, `requires_approval`
- Tabella `community_members` con `role`, `status`
- ENUM `member_role` ('owner', 'admin', 'moderator', 'member')
- Handler `join_community` in `api.rs`
- Handler `join_community_htmx` in `htmx.rs`
- Pulsante "Iscriviti" in `community_home.html`

### Pattern da Seguire
- API handlers in `src/server/src/handlers/api.rs`
- HTMX handlers in `src/server/src/handlers/htmx.rs`
- Templates in `src/server/templates/`
- Fragments in `src/server/templates/fragments/`
- Routes in `src/server/src/main.rs` e `lib.rs`

### File da NON Modificare
- `migrations/001_core_users.sql`
- `migrations/002_communities.sql` (creare nuova migration)

---

## 📝 Regole di Business

1. **Community Pubblica** (`is_public = true`):
   - Chiunque loggato può iscriversi
   - Iscrizione immediata con ruolo `member`

2. **Community Privata** (`is_public = false`):
   - Richiede richiesta di iscrizione
   - Admin deve approvare
   - Utente riceve notifica

3. **Requires Approval** (`requires_approval = true`):
   - Anche se pubblica, richiede approvazione admin
   - Stato `pending` fino ad approvazione

4. **Ruoli e Permessi**:
   - Solo `owner` e `admin` possono approvare/rifiutare
   - Solo `owner` può modificare settings membership
   - `moderator` non può gestire membership

5. **Vincoli**:
   - Un utente può avere una sola richiesta pending per community
   - Non si può richiedere se già membro
   - Non si può richiedere se già rifiutato (cooldown 7 giorni)

---

## 🧪 Piano di Testing

### Unit Test
```rust
#[tokio::test]
async fn test_join_public_community() {
    // Setup: community pubblica, utente loggato
    // Action: POST /api/communities/{id}/join
    // Assert: 201 Created, utente è membro
}

#[tokio::test]
async fn test_request_join_private_community() {
    // Setup: community privata, utente loggato
    // Action: POST /api/communities/{id}/request-join
    // Assert: 201 Created, richiesta pending
}
```

### Manual Test Checklist
- [ ] Utente non loggato vede "Accedi per iscriverti"
- [ ] Utente loggato vede "Iscriviti" su community pubblica
- [ ] Click "Iscriviti" → badge "Membro" appare
- [ ] Utente loggato vede "Richiedi accesso" su community privata
- [ ] Admin vede lista richieste
- [ ] Admin può approvare/rifiutare

---

## ⏱️ Timeline Suggerita

| Task | Tempo |
|------|-------|
| Migration DB | 15 min |
| API Handlers | 45 min |
| HTMX Handlers | 30 min |
| Templates | 30 min |
| Routes | 10 min |
| Test | 30 min |
| **Totale** | **~2.5 ore** |

---

## ✅ Checklist Pre-Consegna

- [ ] `cargo build --workspace` passa
- [ ] `cargo test --workspace` passa
- [ ] Migration applicata senza errori
- [ ] Tutti gli endpoint funzionano
- [ ] UI risponde correttamente agli stati
- [ ] Nessun hardcoded string (usare i18n)
- [ ] Brand compliance (colori Civiqo)

---

## 🚨 Blockers Potenziali

1. **CockroachDB**: Verificare sintassi `gen_random_uuid()` vs `uuid_generate_v4()`
2. **ENUM**: `member_role` già esiste, non ricreare
3. **Auth**: Verificare che `AuthUser` funzioni su tutti gli endpoint

---

## 📞 Escalation

Se blocchi >15 min:
1. Documentare problema
2. Tentare workaround
3. Segnalare a Agent 2 con contesto

---

**Ready for Implementation** ✅
