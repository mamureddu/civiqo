# Guida all'Internazionalizzazione (i18n) - Civiqo

## Panoramica

Civiqo utilizza **Mozilla Fluent** per l'internazionalizzazione, integrato con il template engine **Tera**.

### Lingue Supportate

| Codice | Lingua | Flag | Default |
|--------|--------|------|---------|
| `it` | Italiano | 🇮🇹 | ✅ |
| `en` | English | 🇬🇧 | |

## Architettura

```
src/server/
├── locales/
│   ├── it/                    # Traduzioni italiane
│   │   ├── main.ftl          # Stringhe generali
│   │   ├── auth.ftl          # Autenticazione
│   │   ├── communities.ftl   # Community
│   │   ├── governance.ftl    # Governance
│   │   ├── businesses.ftl    # Attività
│   │   ├── dashboard.ftl     # Dashboard
│   │   ├── posts.ftl         # Post e commenti
│   │   ├── chat.ftl          # Chat
│   │   └── errors.ftl        # Errori
│   └── en/                    # Traduzioni inglesi
│       └── ... (stessa struttura)
├── src/
│   ├── i18n.rs               # Modulo i18n principale
│   └── i18n_tera.rs          # Integrazione con Tera
```

## Uso nei Template

### Accesso alle Traduzioni

Le traduzioni sono disponibili nel context Tera come oggetto `t`:

```html
<!-- Accesso diretto -->
<h1>{{ t.communities_title }}</h1>

<!-- Con fallback -->
<p>{{ t.communities_subtitle | default(value="Scopri le community") }}</p>
```

### Variabile Lingua

```html
<!-- Lingua corrente -->
<html lang="{{ lang }}">

<!-- Condizionale per lingua -->
{% if lang == 'en' %}
  <span>🇬🇧 English</span>
{% else %}
  <span>🇮🇹 Italiano</span>
{% endif %}
```

## Sintassi Fluent (.ftl)

### Stringhe Semplici

```ftl
# Commento
nav-home = Home
nav-communities = Community
```

### Stringhe con Variabili

```ftl
header-welcome = Benvenuto, { $name }
community-created-by = Creata da { $name }
```

### Plurali

```ftl
community-members = { $count ->
    [one] { $count } membro
   *[other] { $count } membri
}
```

### Selezione per Genere

```ftl
user-greeting = { $gender ->
    [male] Benvenuto
    [female] Benvenuta
   *[other] Bentornato/a
}
```

## Aggiungere una Nuova Traduzione

### 1. Aggiungi al file .ftl

```ftl
# locales/it/communities.ftl
my-new-key = La mia nuova stringa
```

```ftl
# locales/en/communities.ftl
my-new-key = My new string
```

### 2. Registra la chiave in i18n_tera.rs

```rust
// In get_translations_for_locale()
let keys = [
    // ... chiavi esistenti
    "my-new-key",  // Aggiungi qui
];
```

### 3. Usa nel template

```html
<p>{{ t.my_new_key }}</p>
```

> **Nota**: I trattini (`-`) nelle chiavi Fluent diventano underscore (`_`) nel template.

## Language Switcher

Il language switcher è nel footer di `base.html`:

```html
<form hx-post="/api/set-language" hx-swap="none">
    <button type="submit" name="lang" value="it">🇮🇹 Italiano</button>
    <button type="submit" name="lang" value="en">🇬🇧 English</button>
</form>
```

### Comportamento

1. L'utente clicca su una lingua
2. HTMX invia POST a `/api/set-language`
3. Il server imposta un cookie `civiqo_lang`
4. La pagina viene ricaricata con la nuova lingua

## Priorità Rilevamento Lingua

1. **Cookie** `civiqo_lang` (se presente)
2. **Header** `Accept-Language` (dal browser)
3. **Default** `it` (italiano)

## API Endpoints

### GET /api/languages

Restituisce le lingue disponibili:

```json
[
  {"code": "it", "name": "Italiano", "flag": "🇮🇹"},
  {"code": "en", "name": "English", "flag": "🇬🇧"}
]
```

### POST /api/set-language

Imposta la lingua preferita:

```
Content-Type: application/x-www-form-urlencoded
lang=en
```

Risposta: Cookie `civiqo_lang=en` + header `HX-Refresh: true`

## Best Practices

### 1. Organizzazione File

- Un file `.ftl` per area funzionale
- Commenti per sezioni
- Chiavi descrittive con prefisso

```ftl
# =============================================================================
# NAVIGAZIONE
# =============================================================================

nav-home = Home
nav-communities = Community
```

### 2. Chiavi Consistenti

```ftl
# ✅ Buono - prefisso consistente
community-create-title = Crea Community
community-create-submit = Crea
community-create-cancel = Annulla

# ❌ Evitare - prefissi inconsistenti
create-community-title = Crea Community
community-submit-btn = Crea
cancel = Annulla
```

### 3. Evitare Stringhe Hardcoded

```html
<!-- ❌ Evitare -->
<button>Salva</button>

<!-- ✅ Preferire -->
<button>{{ t.action_save }}</button>
```

### 4. Gestire Plurali

```ftl
# ✅ Sempre usare plurali per conteggi
post-comments = { $count ->
    [one] { $count } commento
   *[other] { $count } commenti
}
```

## Aggiungere una Nuova Lingua

### 1. Crea la cartella

```bash
mkdir -p src/server/locales/es
```

### 2. Copia i file .ftl

```bash
cp src/server/locales/en/*.ftl src/server/locales/es/
```

### 3. Traduci i file

### 4. Registra la lingua

```rust
// i18n.rs
pub const SUPPORTED_LANGUAGES: &[&str] = &["it", "en", "es"];

// i18n.rs - get_available_languages()
vec![
    LanguageInfo { code: "it".to_string(), name: "Italiano".to_string(), flag: "🇮🇹".to_string() },
    LanguageInfo { code: "en".to_string(), name: "English".to_string(), flag: "🇬🇧".to_string() },
    LanguageInfo { code: "es".to_string(), name: "Español".to_string(), flag: "🇪🇸".to_string() },
]
```

### 5. Ricompila

```bash
cargo build --bin server
```

## Troubleshooting

### Traduzione non appare

1. Verifica che la chiave esista in entrambi i file `.ftl`
2. Verifica che la chiave sia registrata in `i18n_tera.rs`
3. Controlla la conversione trattino → underscore

### Cookie non funziona

1. Verifica che il browser accetti cookie
2. Controlla che il cookie non sia scaduto
3. Verifica il path del cookie (`/`)

### Fallback non funziona

Il fallback è automatico a italiano. Se una chiave manca in inglese, viene usata la versione italiana.

## Testing

```bash
# Verifica compilazione
cargo build --bin server

# Avvia server
cargo run --bin server

# Test cambio lingua
curl -X POST http://localhost:9001/api/set-language -d "lang=en"
```

## Risorse

- [Fluent Project](https://projectfluent.org/)
- [fluent-templates Rust](https://docs.rs/fluent-templates)
- [Tera Templates](https://tera.netlify.app/)
