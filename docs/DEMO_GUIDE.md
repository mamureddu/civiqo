# 🎯 Demo Guide - Cosa Puoi Fare Ora

## 🚀 Server Status
- ✅ Server running su http://localhost:9001
- ✅ Database connesso (CockroachDB Cloud)
- ✅ 1 utente e 1 community nel database

## 📱 Pagine da Testare

### 1. **Database Test Page** 🔥 NUOVO!
**URL**: http://localhost:9001/test-db

Mostra dati REALI dal database:
- Conteggio utenti, communities, posts
- Lista ultimi 5 utenti registrati
- Lista ultime 5 communities create
- Info sulla connessione DB

**Cosa vedere:**
- ✅ Connessione database funzionante
- ✅ Query SQL che funzionano
- ✅ Dati seed caricati (1 user, 1 community)

### 2. **Homepage**
**URL**: http://localhost:9001/

Landing page con:
- Hero section
- Features overview
- Call to action

### 3. **Communities**
**URL**: http://localhost:9001/communities

Lista communities (al momento usa dati mock, ma puoi collegarla al DB)

### 4. **Dashboard**
**URL**: http://localhost:9001/dashboard

Dashboard utente (richiederà login con Auth0)

### 5. **Businesses**
**URL**: http://localhost:9001/businesses

Directory business locali

### 6. **Governance**
**URL**: http://localhost:9001/governance

Sistema di proposte e votazioni

### 7. **Map/POI**
**URL**: http://localhost:9001/poi

Mappa punti di interesse

### 8. **Health Check**
**URL**: http://localhost:9001/health

Endpoint per verificare lo stato del server

## 🧪 Test con cURL

```bash
# Health check
curl http://localhost:9001/health | jq

# Database test (JSON)
curl -s http://localhost:9001/test-db | grep "text-3xl"

# Homepage
curl -s http://localhost:9001/ | head -20

# Communities
curl -s http://localhost:9001/communities | head -20
```

## 🗄️ Interrogare il Database Direttamente

```bash
# Connettiti al database
psql $DATABASE_URL

# Query di esempio
SELECT * FROM users;
SELECT * FROM communities;
SELECT * FROM posts;
SELECT COUNT(*) FROM users;
```

## 📊 Aggiungere Dati di Test

Puoi aggiungere dati via SQL:

```sql
-- Aggiungi un utente
INSERT INTO users (id, username, email, password_hash)
VALUES (
    gen_random_uuid(),
    'mario',
    'mario@example.com',
    'hash_placeholder'
);

-- Aggiungi una community
INSERT INTO communities (id, name, description, created_by)
VALUES (
    gen_random_uuid(),
    'Tech Community',
    'A community for tech enthusiasts',
    (SELECT id FROM users LIMIT 1)
);

-- Aggiungi un post
INSERT INTO posts (id, community_id, author_id, title, content)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM communities LIMIT 1),
    (SELECT id FROM users LIMIT 1),
    'Welcome Post',
    'Welcome to our community!'
);
```

## 🔧 Modificare i Page Handlers

Per collegare altre pagine al database, modifica:
`server/src/handlers/pages.rs`

Esempio per la pagina communities:

```rust
pub async fn communities(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    
    // Fetch real communities from DB
    let communities = sqlx::query("SELECT * FROM communities ORDER BY created_at DESC")
        .fetch_all(&state.db.pool)
        .await?;
    
    ctx.insert("communities", &communities);
    
    let html = state.tera.render("communities.html", &ctx)?;
    Ok(Html(html).into_response())
}
```

## 🎨 Prossimi Passi

1. **Aggiungere più dati seed** nel database
2. **Collegare le altre pagine** al database (communities, businesses, etc.)
3. **Implementare Auth0 login** per autenticazione
4. **Creare API endpoints** per CRUD operations
5. **Aggiungere WebSocket** per chat real-time

## 🐛 Troubleshooting

**Server non risponde?**
```bash
# Verifica che sia running
curl http://localhost:9001/health

# Riavvia se necessario
lsof -ti:9001 | xargs kill -9
cd src && cargo run --bin server
```

**Database non connesso?**
```bash
# Verifica DATABASE_URL in .env
grep DATABASE_URL .env

# Testa connessione
psql $DATABASE_URL -c "SELECT 1"
```

**Template non trovato?**
```bash
# Verifica che i template esistano
ls -la server/templates/

# Ricompila
cargo build --bin server
```

## 📝 Log Utili

Il server mostra log dettagliati:
```
INFO Connecting to database...
INFO Database connected and migrations complete
INFO Templates loaded successfully
INFO API Gateway listening on http://0.0.0.0:9001
```

## 🎉 Success Checklist

- ✅ Server running
- ✅ Database connesso
- ✅ Migrations applicate
- ✅ Template caricati
- ✅ Pagina test-db funzionante
- ✅ Query SQL funzionano
- ✅ Dati seed presenti

**Tutto pronto per sviluppare features!** 🚀
