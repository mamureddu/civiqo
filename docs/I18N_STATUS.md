# 🌐 Internazionalizzazione (i18n) - Status

**Ultimo aggiornamento**: 2025-11-28  
**Stato**: ✅ COMPLETATO

---

## 📊 Overview

| Componente | Stato | Note |
|------------|-------|------|
| Sistema i18n Rust | ✅ Completo | `i18n.rs`, `i18n_tera.rs` |
| File traduzioni IT | ✅ Completo | 9 file, ~750+ chiavi |
| File traduzioni EN | ✅ Completo | 9 file, ~750+ chiavi |
| Middleware locale | ✅ Attivo | Cookie + Accept-Language |
| Language switcher UI | ✅ Presente | Footer di `base.html` |
| **Template integration** | ✅ **COMPLETATO** | Tutti i template usano `{{ t.* }}` |

---

## ✅ Template Integrati (Phase 8.1)

| Template | Chiavi i18n | Stato |
|----------|-------------|-------|
| `base.html` | ~15 | ✅ |
| `index.html` | ~12 | ✅ |
| `dashboard.html` | ~10 | ✅ |
| `communities.html` | ~15 | ✅ |
| `governance.html` | ~12 | ✅ |
| `admin.html` | ~10 | ✅ |
| `create_business.html` | ~5 | ✅ |
| `fragments/community-card.html` | ~8 | ✅ |

---

## 🏗️ Architettura i18n

### Backend (Rust)

```
src/server/src/
├── i18n.rs              # Core i18n: Locale, LOCALES, middleware
├── i18n_tera.rs         # Tera integration: add_i18n_context(), LocaleExtractor
└── main.rs              # Routes: /api/set-language, /api/languages
```

### File di Traduzione (Fluent)

```
src/server/locales/
├── it/                  # Italiano (default)
│   ├── main.ftl         # 119 chiavi - nav, azioni, stati, footer
│   ├── dashboard.ftl    # 100 chiavi - dashboard completa
│   ├── communities.ftl  # 174 chiavi - community, membership
│   ├── governance.ftl   # Proposte, votazioni
│   ├── businesses.ftl   # Attività locali
│   ├── chat.ftl         # Chat
│   ├── posts.ftl        # Post e commenti
│   ├── auth.ftl         # Autenticazione
│   └── errors.ftl       # Pagine errore
└── en/                  # English
    └── (stessa struttura)
```

---

## 🔧 Come Funziona

### 1. Middleware (già attivo)
```rust
// main.rs
.layer(axum::middleware::from_fn(locale_middleware))
```

### 2. Handler (già implementato)
```rust
pub async fn index(
    LocaleExtractor(locale): LocaleExtractor,  // ✅ Estrae locale
    // ...
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);  // ✅ Aggiunge traduzioni
    // ...
}
```

### 3. Template (❌ NON IMPLEMENTATO)
```html
<!-- ATTUALE (hardcoded) -->
<h1>Welcome back!</h1>
<a href="/communities">Communities</a>

<!-- CORRETTO (i18n) -->
<h1>{{ t.dashboard_welcome }}</h1>
<a href="/communities">{{ t.nav_communities }}</a>
```

---

## 📋 Chiavi Disponibili (Esempi)

### main.ftl
```fluent
nav-home = Home
nav-communities = Community
nav-governance = Governance
nav-businesses = Attività
header-login = Accedi
header-logout = Esci
action-save = Salva
action-cancel = Annulla
state-loading = Caricamento...
```

### dashboard.ftl
```fluent
dashboard-title = Dashboard
dashboard-welcome = Bentornato, { $name }!
dashboard-subtitle = Ecco cosa succede nelle tue community
dashboard-stats-communities = Le tue Community
dashboard-section-activity = Attività Recente
```

### communities.ftl
```fluent
communities-title = Community
community-public = Pubblica
community-private = Privata
community-join = Unisciti
community-members = { $count -> 
    [one] { $count } membro
   *[other] { $count } membri
}
```

---

## ✅ Template da Aggiornare

### Priorità Alta
- [ ] `base.html` - navbar, footer, skip link
- [ ] `index.html` - hero, features
- [ ] `dashboard.html` - tutto in inglese attualmente
- [ ] `communities.html` - titoli, filtri, empty state
- [ ] `fragments/community-card.html` - "Public", "Private", "members"

### Priorità Media
- [ ] `governance.html` - tabs, stats, form
- [ ] `community_detail.html` - tabs, azioni
- [ ] `create_business.html` - labels, buttons
- [ ] `admin.html` - tabs, titoli

### Priorità Bassa
- [ ] `chat.html`, `chat_list.html`
- [ ] `profile.html`
- [ ] `404.html`, `500.html`
- [ ] Altri fragments

---

## 🔄 Pattern di Migrazione

### Prima (hardcoded)
```html
<h1 class="text-3xl font-bold">Welcome back!</h1>
<p>Manage your communities</p>
<button>Save</button>
```

### Dopo (i18n)
```html
<h1 class="text-3xl font-bold">{{ t.dashboard_welcome }}</h1>
<p>{{ t.dashboard_subtitle }}</p>
<button>{{ t.action_save }}</button>
```

### Note
- Le chiavi in Tera usano `_` invece di `-` (es. `nav_home` non `nav-home`)
- Questo è gestito automaticamente da `i18n_tera.rs`

---

## 📈 Effort Stimato

| Template | Chiavi da sostituire | Tempo |
|----------|---------------------|-------|
| `base.html` | ~15 | 30 min |
| `index.html` | ~10 | 20 min |
| `dashboard.html` | ~20 | 30 min |
| `communities.html` | ~15 | 25 min |
| `governance.html` | ~20 | 30 min |
| `community-card.html` | ~8 | 15 min |
| Altri | ~50 | 1.5h |
| **Totale** | **~140** | **~4 ore** |

---

## 🧪 Testing

```bash
# Cambia lingua via cookie
curl -b "civiqo_lang=en" http://localhost:3000/

# Cambia lingua via header
curl -H "Accept-Language: en-US" http://localhost:3000/

# API per cambiare lingua
curl -X POST http://localhost:3000/api/set-language -d "lang=en"
```

---

## 📚 Riferimenti

- **Fluent**: https://projectfluent.org/
- **fluent-templates**: https://docs.rs/fluent-templates
- **Tera**: https://tera.netlify.app/

---

*Documento da aggiornare dopo ogni sprint di integrazione i18n.*
