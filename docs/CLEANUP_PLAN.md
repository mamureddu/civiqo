# 📋 Documentation Cleanup Plan

## 🗑️ CANCELLARE (Obsoleti)

### Root Directory
1. **AUTHENTICATION.md** - ❌ Parla di NextAuth.js (non usato)
2. **FIX_DB_SYNC.md** - ❌ Fix specifico, già applicato
3. **MIGRATION_COMPLETE.md** - ❌ Report storico
4. **PROGRESS_UPDATE.md** - ❌ Report storico
5. **VERIFICATION_SUMMARY.md** - ❌ Report storico
6. **OLD-CONV.md** - ❌ Conversazione vecchia

### Src Directory
1. **RUSTLS_VALIDATION_REPORT.md** - ❌ Report tecnico storico
2. **SECURITY_TEST_REPORT.md** - ❌ Report storico
3. **TEST_CLEANUP_SUMMARY.md** - ❌ Report storico

### Docs Directory
1. **AUTH0_SETUP.md** - ❌ Duplicato di AUTH0_INTEGRATION.md
2. **ENV_SETUP.md** - ❌ Duplicato di ENVIRONMENT.md
3. **HTMX_WASM_MIGRATION.md** - ❌ Report storico
4. **TEST_REPORT.md** - ❌ Report storico

**Total: 13 file da cancellare**

---

## 📦 SPOSTARE IN DOCS (così come sono)

### Root → docs/
1. **NEXT_STEPS.md** → `docs/NEXT_STEPS.md`
2. **PROJECT_ROADMAP.md** → `docs/PROJECT_ROADMAP.md`
3. **PROJECT_STRUCTURE.md** → `docs/PROJECT_STRUCTURE.md`
4. **DEPLOY_AUTHORIZER.md** → `docs/DEPLOY_AUTHORIZER.md`
5. **README.md** → Mantieni in root (è il main README)

### Src → docs/
1. **API_GUIDE.md** → `docs/API_GUIDE.md`
2. **AUTH_GUIDE.md** → Aggregare con DEPLOY_AUTHORIZER
3. **DATABASE_INTEGRATION.md** → `docs/DATABASE_INTEGRATION.md`
4. **DEMO_GUIDE.md** → `docs/DEMO_GUIDE.md`
5. **LAMBDA_AUTHORIZER_GUIDE.md** → Aggregare con DEPLOY_AUTHORIZER
6. **QUICK_START.md** → `docs/QUICK_START.md`
7. **USAGE_GUIDE.md** → `docs/USAGE_GUIDE.md`
8. **TESTING.md** → Merge con docs/TESTING.md

---

## 🔗 AGGREGARE (Consolidare in un file)

### 1. **DEPLOY_AUTHORIZER.md** (Root)
   + **AUTH_GUIDE.md** (src/)
   + **LAMBDA_AUTHORIZER_GUIDE.md** (src/)
   
   **Risultato**: `docs/AUTHORIZER_DEPLOYMENT.md`
   
   **Contenuto**:
   - Overview autenticazione
   - OAuth2 flow
   - Lambda Authorizer setup
   - API Gateway configuration
   - Context injection
   - Caching strategy

### 2. **TESTING.md** (src/)
   + **docs/TESTING.md**
   
   **Risultato**: `docs/TESTING.md` (merge)
   
   **Contenuto**:
   - Unit tests
   - Integration tests
   - E2E tests
   - Test coverage

---

## 📁 Struttura Finale (docs/)

```
docs/
├── AUTHORIZER_DEPLOYMENT.md    (aggregato)
├── API_GUIDE.md                (spostato)
├── CLAUDE.md                   (mantieni)
├── DATABASE_INTEGRATION.md      (spostato)
├── DEMO_GUIDE.md               (spostato)
├── DEVELOPMENT.md              (mantieni)
├── ENVIRONMENT.md              (mantieni)
├── MIGRATION.md                (mantieni)
├── NEXT_STEPS.md               (spostato)
├── PROJECT_ROADMAP.md          (spostato)
├── PROJECT_STRUCTURE.md        (spostato)
├── QUICK_START.md              (spostato)
├── SCHEMA.md                   (mantieni)
├── TESTING.md                  (merge)
└── USAGE_GUIDE.md              (spostato)
```

---

## 🎯 Azioni da Eseguire

### 1. Cancellare (13 file)
```bash
rm AUTHENTICATION.md
rm FIX_DB_SYNC.md
rm MIGRATION_COMPLETE.md
rm PROGRESS_UPDATE.md
rm VERIFICATION_SUMMARY.md
rm OLD-CONV.md
rm src/RUSTLS_VALIDATION_REPORT.md
rm src/SECURITY_TEST_REPORT.md
rm src/TEST_CLEANUP_SUMMARY.md
rm docs/AUTH0_SETUP.md
rm docs/ENV_SETUP.md
rm docs/HTMX_WASM_MIGRATION.md
rm docs/TEST_REPORT.md
```

### 2. Aggregare
- [ ] Merge AUTH_GUIDE.md + LAMBDA_AUTHORIZER_GUIDE.md → DEPLOY_AUTHORIZER.md
- [ ] Merge src/TESTING.md + docs/TESTING.md

### 3. Spostare
- [ ] NEXT_STEPS.md → docs/
- [ ] PROJECT_ROADMAP.md → docs/
- [ ] PROJECT_STRUCTURE.md → docs/
- [ ] DEPLOY_AUTHORIZER.md → docs/
- [ ] src/API_GUIDE.md → docs/
- [ ] src/DATABASE_INTEGRATION.md → docs/
- [ ] src/DEMO_GUIDE.md → docs/
- [ ] src/QUICK_START.md → docs/
- [ ] src/USAGE_GUIDE.md → docs/

---

## ✅ Risultato Finale

**Prima**: 37 file .md sparsi in 3 directory
**Dopo**: 15 file .md organizzati in docs/ + README.md in root

**Benefici**:
- ✅ Documentazione centralizzata
- ✅ Niente duplicati
- ✅ Niente file obsoleti
- ✅ Facile da navigare
- ✅ Facile da mantenere

---

## 📝 Note

- **README.md** rimane in root (entry point)
- **authorizer/** ha i suoi README.md (specifici del servizio)
- **src/** avrà solo codice, niente .md
- **docs/** avrà tutta la documentazione
