# 📦 Dependency Audit Report - Civiqo

**Data**: 28 Novembre 2025  
**Analisi effettuata da**: Agent 2 (Tech Lead Verifier)  
**Stato**: ✅ MIGRAZIONE COMPLETATA

---

## 📊 Riepilogo Esecutivo

| Categoria | Conteggio |
|-----------|-----------|
| 🔴 **Critiche** (breaking changes significativi) | 4 |
| 🟠 **Importanti** (aggiornamenti consigliati) | 8 |
| 🟢 **Minori** (patch/bugfix) | 12 |
| ✅ **Aggiornate** | 5 |

---

## 🔴 Migrazioni CRITICHE (Richiedono Modifiche al Codice)

### 1. **axum** `0.7` → `0.8.7`
**Impatto**: 🔴 ALTO - Breaking changes significativi

**Cambiamenti principali**:
1. **Sintassi path parameters**: `/:id` → `/{id}`, `/*path` → `/{*path}`
2. **`Option<T>` extractor**: Richiede `OptionalFromRequestParts` trait
3. **Rimozione `#[async_trait]`**: Extractors custom devono essere aggiornati

**File da modificare**:
- `src/server/src/main.rs` - Tutte le route definitions
- `src/server/src/i18n_tera.rs` - `LocaleExtractor` implementation
- `src/server/src/auth.rs` - `AuthUser`, `OptionalAuthUser` extractors

**Esempio migrazione route**:
```rust
// PRIMA (0.7)
.route("/communities/:id", get(community_detail))
.route("/posts/:id/comments", get(comments))

// DOPO (0.8)
.route("/communities/{id}", get(community_detail))
.route("/posts/{id}/comments", get(comments))
```

**Stima effort**: 2-3 ore

---

### 2. **reqwest** `0.11` → `0.12.24`
**Impatto**: 🔴 ALTO - Upgrade a hyper v1

**Cambiamenti principali**:
1. Upgrade interno a `hyper v1` e `http v1`
2. Cambio comportamento GZIP decoding
3. Nuove API per streaming

**File da modificare**:
- `src/server/src/auth.rs` - HTTP client calls
- Qualsiasi uso di `reqwest::Client`

**Stima effort**: 1-2 ore

---

### 3. **openidconnect** `3.0` → `4.0.1`
**Impatto**: 🔴 ALTO - Breaking changes API

**Cambiamenti principali**:
1. Bump a `oauth2 5.0.0`
2. Rimozione feature `jwk-alg`
3. Cambio trait JWT-related (associated types invece di generics)
4. Richiede `reqwest 0.12` e `http 1.0`

**File da modificare**:
- `src/server/src/auth.rs` - Tutto il flusso OAuth2/OIDC

**⚠️ NOTA**: Questa migrazione è **bloccante** per reqwest 0.12

**Stima effort**: 3-4 ore

---

### 4. **rustls** `0.21` → `0.23.35`
**Impatto**: 🔴 ALTO - Cambio architettura crypto provider

**Cambiamenti principali**:
1. Nuovo sistema `CryptoProvider` (pluggable cryptography)
2. Rimozione `dangerous_configuration` feature
3. Cambio API `ServerName`
4. Richiede `rustls-native-certs 0.8`

**File da modificare**:
- Configurazione TLS in tutto il progetto
- `tokio-rustls` deve essere aggiornato di conseguenza

**Stima effort**: 2-3 ore

---

## 🟠 Aggiornamenti IMPORTANTI (Consigliati)

### 5. **tower-http** `0.5` → `0.6.7`
**Impatto**: 🟠 MEDIO

**Cambiamenti**:
- Nuove features e middleware
- Miglioramenti performance
- Compatibilità con axum 0.8

**Stima effort**: 30 min

---

### 6. **validator** `0.16` → `0.20.0`
**Impatto**: 🟠 MEDIO

**Cambiamenti**:
- Nuove regole di validazione
- Miglioramenti derive macro
- Possibili breaking changes minori

**Stima effort**: 1 ora

---

### 7. **lambda_runtime** `0.8` → `1.0.1`
**Impatto**: 🟠 MEDIO - Major version

**Cambiamenti**:
- API stabilizzata (v1.0!)
- Nuove features per cold start
- Miglioramenti error handling

**Stima effort**: 1 ora

---

### 8. **lambda_http** `0.11` → `1.0.1`
**Impatto**: 🟠 MEDIO - Major version

**Cambiamenti**:
- Allineamento con lambda_runtime 1.0
- Nuove API per request/response

**Stima effort**: 1 ora

---

### 9. **config** `0.13` → `0.15.19`
**Impatto**: 🟠 MEDIO

**Cambiamenti**:
- Nuovi formati supportati
- Miglioramenti API

**Stima effort**: 30 min

---

### 10. **mockall** `0.11` → `0.14.0`
**Impatto**: 🟠 MEDIO (solo test)

**Cambiamenti**:
- Nuove features mocking
- Miglioramenti macro

**Stima effort**: 30 min

---

### 11. **fluent-templates** `0.9` → `0.13.2`
**Impatto**: 🟠 MEDIO

**Cambiamenti**:
- Nuove API
- Miglioramenti performance
- Possibili breaking changes

**Stima effort**: 1 ora

---

### 12. **tokio-tungstenite** `0.21` → `0.28.0`
**Impatto**: 🟠 MEDIO

**Cambiamenti**:
- Upgrade tungstenite
- Nuove features WebSocket

**Stima effort**: 1 ora (chat-service)

---

## 🟢 Aggiornamenti MINORI (Patch/Bugfix)

| Pacchetto | Attuale | Ultima | Note |
|-----------|---------|--------|------|
| tokio | 1.0 | 1.48.0 | Semver compatible |
| serde | 1.0 | 1.0.x | Semver compatible |
| serde_json | 1.0 | 1.0.x | Semver compatible |
| sqlx | 0.8 | 0.8.x (0.9 alpha) | Mantieni 0.8 |
| uuid | 1.0 | 1.x | Semver compatible |
| chrono | 0.4 | 0.4.x | Semver compatible |
| anyhow | 1.0 | 1.0.x | Semver compatible |
| thiserror | 1.0 | 1.0.x → 2.0 | 2.0 disponibile |
| base64 | 0.21 | 0.22.1 | Minor update |
| tera | 1.19 | 1.20.1 | Minor update |
| cookie | 0.18 | 0.18.1 | Patch |
| wiremock | 0.5 | 0.6.5 | Minor update |
| axum-test | 14.0 | 18.3.0 | Richiede axum 0.8 |
| actix-files | 0.6 | 0.6.9 | Patch |
| rustls-native-certs | 0.6 | 0.8.2 | Richiede rustls 0.23 |

---

## ✅ Pacchetti Già Aggiornati

| Pacchetto | Versione |
|-----------|----------|
| tower-sessions | 0.14.0 ✅ |
| http | 1.0 ✅ |
| accept-language | 3.1 ✅ |
| ring | 0.17 ✅ |
| dotenvy | 0.15 ✅ |

---

## 📋 Piano di Migrazione Consigliato

### Fase 1: Preparazione (1 giorno)
1. Creare branch `feature/dependency-upgrade`
2. Backup completo
3. Documentare stato attuale test

### Fase 2: Migrazioni Core (2-3 giorni)
**Ordine consigliato** (per dipendenze):

1. **rustls** 0.21 → 0.23 + rustls-native-certs
2. **reqwest** 0.11 → 0.12 (dipende da rustls)
3. **openidconnect** 3.0 → 4.0 (dipende da reqwest)
4. **axum** 0.7 → 0.8 + tower-http 0.6
5. **lambda_runtime/http** → 1.0

### Fase 3: Migrazioni Secondarie (1 giorno)
- validator, config, mockall
- fluent-templates
- tokio-tungstenite

### Fase 4: Test e Verifica (1 giorno)
- Eseguire tutti i test
- Test manuale flusso auth
- Test deployment Lambda

---

## ⚠️ Rischi e Mitigazioni

### Rischio 1: Breaking Auth Flow
**Mitigazione**: Test approfondito OAuth2 dopo openidconnect upgrade

### Rischio 2: Incompatibilità Lambda
**Mitigazione**: Test locale con cargo-lambda prima del deploy

### Rischio 3: Regressioni TLS
**Mitigazione**: Verificare connessioni DB e API esterne

---

## 📊 Stima Effort Totale

| Fase | Ore |
|------|-----|
| Fase 1: Preparazione | 2-4 |
| Fase 2: Migrazioni Core | 12-16 |
| Fase 3: Migrazioni Secondarie | 4-6 |
| Fase 4: Test e Verifica | 4-8 |
| **TOTALE** | **22-34 ore** |

---

## 🎯 Raccomandazione Finale

**Priorità ALTA**: Procedere con l'aggiornamento entro le prossime 2 settimane.

**Motivazioni**:
1. `axum 0.8` è la versione stabile corrente
2. `openidconnect 4.0` ha fix di sicurezza importanti
3. `lambda_runtime 1.0` è la prima versione stabile
4. Mantenere dipendenze aggiornate riduce debito tecnico

**Approccio consigliato**: Migrazione incrementale per fase, con test completi dopo ogni fase.

---

*Report generato seguendo le linee guida di `AGENT_2_VERIFIER_PROMPT.md`*
