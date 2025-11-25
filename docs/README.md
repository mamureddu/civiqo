# 📚 Community Manager Documentation

Benvenuto nella documentazione del progetto Community Manager. Questa guida ti aiuta a navigare rapidamente tra i documenti essenziali per lo sviluppo, deployment e manutenzione dell'applicazione.

## 📋 Documenti Essenziali

### 🚀 Sviluppo e Setup
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Guida completa allo sviluppo locale
  - Quick start e setup one-command
  - Configurazione ambiente e variabili
  - Script di sviluppo e testing
  - Troubleshooting comune

### 🏗️ Architettura e Progetto
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Architettura del sistema e stato del progetto
  - Stack tecnologico completo
  - Struttura del progetto
  - Stato attuale e roadmap
  - Pattern architetturali

### 🔐 Autenticazione e Autorizzazione
- **[AUTHENTICATION.md](./AUTHENTICATION.md)** - Sistema di autenticazione completo
  - Flusso OAuth2 con Auth0
  - Lambda Authorizer deployment
  - Gestione sessioni
  - Configurazione sicurezza

### 📡 API e Database
- **[API.md](./API.md)** - Guide API e schema database
  - Tutti gli endpoint REST
  - Schema database completo
  - Esempi di utilizzo
  - Query patterns e performance

### 🎨 Brand e UI/UX
- **[BRAND.md](./BRAND.md)** - Linee guida brand Civiqo
  - Palette colori ufficiale
  - Tipografia e componenti
  - Asset e risorse
  - Checklist compliance

### 🚀 Deployment e Operazioni
- **[DEPLOYMENT.md](./DEPLOYMENT.md)** - Guida deployment completa
  - Deploy su AWS Lambda
  - Configurazione ambienti
  - Monitoraggio e troubleshooting
  - Procedure di rollback

### 📊 Database Schema
- **[SCHEMA.md](./SCHEMA.md)** - Documentazione schema database dettagliata
  - Tutte le 22 tabelle
  - Relazioni e indici
  - Diagrammi visivi
  - Riferimento completo

---

## 🎯 Quick Start Guide

### 1. Primo Setup
```bash
# Clona il repository
git clone <repository-url>
cd community-manager

# Configura ambiente
./scripts/check-env.sh
```

### 2. Avvio Sviluppo
```bash
# Avvia tutti i servizi (backend + frontend)
./scripts/start-all.sh

# O avvia solo il backend
./scripts/start-backend.sh
```

### 3. Accesso Applicazione
- **Frontend**: http://localhost:3000
- **API Backend**: http://localhost:9001
- **Chat Service**: ws://localhost:9002

### 4. Testing
```bash
# Esegui tutti i test
cd src && cargo test --workspace

# Test specifici
cargo test --test pages_test
```

---

## 📁 Struttura Documentation

```
docs/
├── README.md                    # Questa guida - navigazione rapida
├── DEVELOPMENT.md               # Setup e guida sviluppo
├── ARCHITECTURE.md              # Architettura e stato progetto
├── AUTHENTICATION.md            # Autenticazione e autorizzazione
├── API.md                       # API endpoints e database
├── BRAND.md                     # Linee guida brand Civiqo
├── SCHEMA.md                    # Schema database dettagliato
└── [legacy files]               # File obsoleti (da ignorare)
```

---

## 🔍 Come Usare Questa Documentazione

### Per Nuovi Sviluppatori
1. Leggi **[DEVELOPMENT.md](./DEVELOPMENT.md)** per setup iniziale
2. Consulta **[ARCHITECTURE.md](./ARCHITECTURE.md)** per capire il sistema
3. Usa **[API.md](./API.md)** per riferimento degli endpoint
4. Segui **[BRAND.md](./BRAND.md)** per UI/UX compliance

### Per Feature Development
1. **[API.md](./API.md)** - Per nuovi endpoint e query database
2. **[AUTHENTICATION.md](./AUTHENTICATION.md)** - Per proteggere risorse
3. **[BRAND.md](./BRAND.md)** - Per mantenere coerenza visiva
4. **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Per testing e debug

### Per Deployment e Ops
1. **[AUTHENTICATION.md](./AUTHENTICATION.md)** - Configurazione Auth0 e Lambda
2. **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Script e ambiente
3. **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Scalabilità e monitoraggio

---

## 🛠️ Script Utili

| Script | Scopo | Comando |
|--------|-------|---------|
| **Start All** | Avvia tutti i servizi | `./scripts/start-all.sh` |
| **Start Backend** | Solo backend services | `./scripts/start-backend.sh` |
| **Start Frontend** | Solo frontend dev | `./scripts/start-frontend.sh` |
| **Check Env** | Valida configurazione | `./scripts/check-env.sh` |
| **Deploy** | Deploy su AWS | `./scripts/deploy.sh` |
| **Test Suite** | Esegue tutti i test | `./scripts/test-suite.sh` |

---

## 🌐 URLs di Riferimento

### Ambiente Locale
- **Homepage**: http://localhost:9001/
- **Dashboard**: http://localhost:9001/dashboard
- **API Health**: http://localhost:9001/health
- **Auth Login**: http://localhost:9001/auth/login

### Servizi Esterni
- **CockroachDB Cloud**: https://cockroachlabs.cloud/
- **Auth0 Dashboard**: https://manage.auth0.com/
- **AWS Console**: https://console.aws.amazon.com/

---

## 📞 Supporto e Troubleshooting

### Problemi Comuni
- **Server non parte**: Controlla `.env` in `src/`
- **Database non connette**: Verifica `DATABASE_URL`
- **Auth non funziona**: Controlla credenziali Auth0
- **Tests falliscono**: Usa `SQLX_OFFLINE=true`

### Comandi di Debug
```bash
# Controlla ambiente
./scripts/check-env.sh

# Verifica build
cargo build --workspace

# Test con offline mode
SQLX_OFFLINE=true cargo test --workspace

# Pulisci e rebuild
cargo clean && cargo build --workspace
```

---

## 📈 Stato Progetto

### ✅ Completato
- Database CockroachDB Cloud integrato
- Autenticazione Auth0 funzionante
- API REST endpoints implementati
- Dashboard con dati reali
- 204 tests passing
- Zero compilation errors

### 🚧 In Corso
- Community CRUD operations
- Chat WebSocket service
- Business directory features

### ⏳ Prossimi
- Governance tools
- WASM components
- Mobile PWA
- Production deployment

---

## 🔄 Aggiornamenti Documentazione

Questa documentazione viene aggiornata regolarmente:
- **Versione corrente**: v2.0 (Consolidated)
- **Ultimo aggiornamento**: November 25, 2025
- **Prossima revisione**: Dopo Community CRUD completion

### Per Contribuire
1. Aggiorna i documenti rilevanti
2. Aggiungi data e versione
3. Aggiorna questo README se necessario
4. Testa tutti i link e comandi

---

## 📝 Note Importanti

### ⚠️ Avvertenze
- Fare sempre riferimento a **[BRAND.md](./BRAND.md)** per modifiche UI
- Usare **[AUTHENTICATION.md](./AUTHENTICATION.md)** per modifiche sicurezza
- Consultare **[API.md](./API.md)** prima di modificare endpoint

### 💡 Tips
- Usa `./scripts/check-env.sh` per diagnosticare problemi
- Consulta **[DEVELOPMENT.md](./DEVELOPMENT.md)** per troubleshooting dettagliato
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** contiene decisioni architetturali importanti

---

**Buono sviluppo! 🚀**

Per domande o supporto, consulta i documenti specifici o contatta il team di sviluppo.
