# 📋 Single-Community Instance Refactor - Specifiche

## 🎯 Obiettivo
Trasformare Civiqo da piattaforma multi-community a istanza single-community federabile.

## Decisioni Prese
1. **Istanze esistenti**: Rimuovere tutto, partire fresh con onboarding
2. **Primo setup**: Wizard al primo avvio (Opzione A)
3. **Federazione**: Predisporre schema, implementare dopo (Opzione B)
4. **Branding**: Colori custom con override Tailwind (Opzione B)
5. **Modello dati**: Opzione C (ibrida)

---

## 📊 KPI di Successo

| Metrica | Target | Verifica |
|---------|--------|----------|
| Build | 0 errori | `cargo build -p server` |
| Test | 100% pass | `cargo test -p server` |
| Onboarding | Funzionante | Test manuale |
| Settings page | Funzionante | Test manuale |

---

## 🗂️ Deliverables

### 1. Database (Model)

#### Nuova Migration: `009_single_community.sql`

```sql
-- Nuove tabelle
CREATE TABLE instance_settings (
    key VARCHAR(100) PRIMARY KEY,
    value TEXT,
    value_type VARCHAR(50) DEFAULT 'string',
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE federation_config (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    hub_url VARCHAR(500),
    api_key VARCHAR(255),
    enabled BOOLEAN DEFAULT false,
    sync_members BOOLEAN DEFAULT false,
    sync_posts BOOLEAN DEFAULT false,
    sync_proposals BOOLEAN DEFAULT false,
    last_sync_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Constraint: solo 1 community
-- Nota: CockroachDB non supporta CHECK con subquery, usiamo trigger o app logic
```

#### Seed: Community di default al primo avvio
- Se `communities` è vuota → mostra wizard onboarding
- Wizard crea la community con i dati inseriti

### 2. API Handlers (Controller)

#### Nuovi Endpoints

| Endpoint | Metodo | Auth | Descrizione |
|----------|--------|------|-------------|
| `/api/instance` | GET | No | Info pubblica istanza |
| `/api/instance/settings` | GET | Admin | Leggi settings |
| `/api/instance/settings` | PUT | Admin | Aggiorna settings |
| `/api/instance/federation` | GET | Admin | Config federazione |
| `/api/instance/federation` | PUT | Admin | Aggiorna federazione |
| `/setup` | GET | No | Pagina onboarding (se no community) |
| `/setup` | POST | No | Crea community iniziale |

#### Endpoints da Rimuovere/Modificare

| Endpoint | Azione |
|----------|--------|
| `POST /api/communities` | Rimuovere (solo setup) |
| `DELETE /api/communities/:id` | Rimuovere |
| `GET /api/communities` | Modificare (ritorna singola) |
| `GET /communities` | Redirect a `/` o community detail |
| `GET /communities/create` | Rimuovere |

### 3. Templates (View)

#### Nuovi Templates

1. **`setup.html`** - Wizard onboarding
   - Step 1: Nome e descrizione community
   - Step 2: Impostazioni privacy (pubblica/privata)
   - Step 3: Branding (colori custom opzionali)
   - Step 4: Admin account (primo utente)

2. **`admin/instance_settings.html`** - Configurazione istanza
   - Sezione: Identità (nome, slug, descrizione, logo)
   - Sezione: Accesso (pubblica/privata, approvazione)
   - Sezione: Branding (colori primary, secondary, accent)
   - Sezione: Avanzate (export, danger zone)

3. **`admin/federation.html`** - Gestione federazione
   - Stato connessione
   - URL hub Civiqo
   - Opzioni sync

#### Templates da Modificare

| Template | Modifica |
|----------|----------|
| `base.html` | Rimuovere dropdown communities, semplificare navbar |
| `index.html` | Diventa landing community (non aggregatore) |
| `dashboard.html` | Rimuovere selettore community |
| `communities.html` | Rimuovere o redirect |

### 4. Routes

#### Nuove Routes

```rust
// Setup/Onboarding
.route("/setup", get(pages::setup_page))
.route("/setup", post(api::complete_setup))

// Instance settings
.route("/admin/settings", get(pages::instance_settings))
.route("/api/instance", get(api::get_instance_info))
.route("/api/instance/settings", get(api::get_instance_settings))
.route("/api/instance/settings", put(api::update_instance_settings))

// Federation
.route("/admin/federation", get(pages::federation_settings))
.route("/api/instance/federation", get(api::get_federation_config))
.route("/api/instance/federation", put(api::update_federation_config))
```

### 5. Middleware

#### Setup Check Middleware
- Se `communities` è vuota E path != `/setup` → redirect a `/setup`
- Eccezioni: `/auth/*`, `/api/health`, assets statici

---

## 🎨 UX Specification (Agent UX)

### Onboarding Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    STEP 1: BENVENUTO                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  🏛️ Benvenuto in Civiqo                             │   │
│  │                                                      │   │
│  │  Configura la tua comunità in pochi passi.          │   │
│  │                                                      │   │
│  │  Nome comunità: [________________]                   │   │
│  │  Descrizione:   [________________]                   │   │
│  │                 [________________]                   │   │
│  │                                                      │   │
│  │                              [Continua →]            │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    STEP 2: PRIVACY                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  🔒 Impostazioni di accesso                         │   │
│  │                                                      │   │
│  │  ○ Comunità pubblica                                │   │
│  │    Chiunque può vedere e partecipare                │   │
│  │                                                      │   │
│  │  ○ Comunità privata                                 │   │
│  │    Solo membri approvati possono accedere           │   │
│  │                                                      │   │
│  │  □ Richiedi approvazione per nuovi membri           │   │
│  │                                                      │   │
│  │  [← Indietro]                    [Continua →]       │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    STEP 3: BRANDING                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  🎨 Personalizza l'aspetto (opzionale)              │   │
│  │                                                      │   │
│  │  Colore primario:   [#2563EB] [■]                   │   │
│  │  Colore secondario: [#57C98A] [■]                   │   │
│  │  Colore accento:    [#FF6B6B] [■]                   │   │
│  │                                                      │   │
│  │  [Usa colori Civiqo default]                        │   │
│  │                                                      │   │
│  │  [← Indietro]                    [Continua →]       │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    STEP 4: COMPLETA                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  ✅ Tutto pronto!                                   │   │
│  │                                                      │   │
│  │  Accedi con il tuo account per diventare            │   │
│  │  l'amministratore della comunità.                   │   │
│  │                                                      │   │
│  │              [Accedi con Auth0]                     │   │
│  │                                                      │   │
│  │  Dopo l'accesso sarai reindirizzato alla            │   │
│  │  dashboard della tua nuova comunità.                │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Stati UI

| Stato | Comportamento |
|-------|---------------|
| **No community** | Redirect a `/setup` |
| **Setup in progress** | Wizard multi-step |
| **Setup complete** | Redirect a `/` (community home) |
| **Error** | Messaggio inline con retry |

### Brand Compliance
- Colori: Civiqo palette (blue, green, coral)
- Tipografia: Inter/system fonts
- Icone: Lucide icons
- Spacing: Tailwind scale

---

## 🧪 Piano di Testing

### Test da Aggiornare
- Tutti i test che creano multiple communities
- Test che usano `POST /api/communities`
- Test che listano communities

### Nuovi Test

```rust
// Setup tests
test_setup_page_shown_when_no_community
test_setup_creates_community
test_setup_redirects_after_complete
test_setup_not_accessible_after_community_exists

// Instance settings tests
test_instance_settings_requires_admin
test_instance_settings_update
test_instance_info_public

// Federation tests
test_federation_config_requires_admin
test_federation_disabled_by_default
```

---

## ⏱️ Timeline

| Fase | Task | Tempo |
|------|------|-------|
| 1 | Pulire DB + Migration | 30 min |
| 2 | Setup page + handlers | 1 ora |
| 3 | Refactor handlers esistenti | 1 ora |
| 4 | Instance settings page | 45 min |
| 5 | Aggiornare templates | 45 min |
| 6 | Aggiornare test | 1 ora |
| 7 | Review e fix | 30 min |

---

## ✅ Checklist Pre-Consegna

- [ ] `cargo build -p server` passa
- [ ] `cargo test -p server` passa
- [ ] Setup wizard funziona
- [ ] Settings page funziona
- [ ] Navbar semplificata
- [ ] Redirect corretti
- [ ] No regression su funzionalità esistenti

