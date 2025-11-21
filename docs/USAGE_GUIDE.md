# 🎯 Guida Rapida - Come Usare il Sistema

## ✅ Stato Attuale

- ✅ Server running su http://localhost:9001
- ✅ Database connesso (CockroachDB Cloud)
- ✅ Pagine HTMX funzionanti
- ✅ API endpoints configurati
- ⚠️ API in debug (alcune query da ottimizzare)

## 🌐 Pagine Web Funzionanti

### 1. **Homepage**
**URL**: http://localhost:9001/
- Landing page con hero section
- Features overview

### 2. **Communities** ⭐ COLLEGATA AL DB
**URL**: http://localhost:9001/communities
- Mostra tutte le communities dal database
- Form per creare nuove communities
- Link ai dettagli di ogni community

### 3. **Community Detail** ⭐ COLLEGATA AL DB
**URL**: http://localhost:9001/communities/:id
- Dettagli community
- Lista posts
- Info creatore

### 4. **Database Test** ⭐ DATI REALI
**URL**: http://localhost:9001/test-db
- Statistiche in tempo reale
- Conteggio utenti, communities, posts
- Lista ultimi utenti e communities

### 5. **Dashboard**
**URL**: http://localhost:9001/dashboard
- Dashboard utente

### 6. **Businesses**
**URL**: http://localhost:9001/businesses
- Directory business locali

### 7. **Governance**
**URL**: http://localhost:9001/governance
- Sistema proposte e votazioni

### 8. **Map/POI**
**URL**: http://localhost:9001/poi
- Mappa punti di interesse

## 🗄️ Lavorare con il Database

### Connessione Diretta

```bash
# Connettiti al database
psql $DATABASE_URL

# Query di esempio
SELECT * FROM users;
SELECT * FROM communities;
SELECT * FROM posts;
```

### Creare Dati Manualmente

```sql
-- Crea un utente
INSERT INTO users (id, username, email, password_hash)
VALUES (
    gen_random_uuid(),
    'mario',
    'mario@example.com',
    'hashed_password_here'
);

-- Crea una community
INSERT INTO communities (id, name, description, created_by)
VALUES (
    gen_random_uuid(),
    'Tech Community',
    'A community for tech enthusiasts',
    (SELECT id FROM users WHERE username = 'mario')
);

-- Crea un post
INSERT INTO posts (id, community_id, author_id, title, content)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM communities WHERE name = 'Tech Community'),
    (SELECT id FROM users WHERE username = 'mario'),
    'Welcome Post',
    'Welcome to our community!'
);
```

### Verificare i Dati

```sql
-- Conta elementi
SELECT COUNT(*) FROM users;
SELECT COUNT(*) FROM communities;
SELECT COUNT(*) FROM posts;

-- Vedi dati recenti
SELECT username, email, created_at FROM users ORDER BY created_at DESC LIMIT 5;
SELECT name, description, created_at FROM communities ORDER BY created_at DESC LIMIT 5;
SELECT title, content, created_at FROM posts ORDER BY created_at DESC LIMIT 5;

-- Join per vedere community con creatori
SELECT c.name, c.description, u.username as creator
FROM communities c
LEFT JOIN users u ON c.created_by = u.id;

-- Join per vedere posts con autori
SELECT p.title, p.content, u.username as author, c.name as community
FROM posts p
LEFT JOIN users u ON p.author_id = u.id
LEFT JOIN communities c ON p.community_id = c.id;
```

## 🎨 Usare l'Interfaccia Web

### Creare una Community

1. Vai su http://localhost:9001/communities
2. Compila il form "Create New Community"
3. Inserisci:
   - **Nome**: es. "Tech Enthusiasts"
   - **Descrizione**: es. "A community for tech lovers"
4. Clicca "Create Community"
5. La pagina si ricarica con la nuova community

**Nota**: Per ora usa automaticamente il primo utente nel database come creatore.

### Vedere i Dettagli

1. Dalla pagina communities, clicca sul nome di una community
2. Vedrai:
   - Nome e descrizione
   - Creatore
   - Lista dei posts (se presenti)

### Monitorare i Dati

1. Vai su http://localhost:9001/test-db
2. Vedrai in tempo reale:
   - Numero totale di utenti
   - Numero totale di communities
   - Numero totale di posts
   - Ultimi 5 utenti creati
   - Ultime 5 communities create

## 🔧 Workflow Completo

### 1. Crea Utenti nel Database

```sql
psql $DATABASE_URL << EOF
INSERT INTO users (id, username, email, password_hash) VALUES
  (gen_random_uuid(), 'alice', 'alice@example.com', 'hash1'),
  (gen_random_uuid(), 'bob', 'bob@example.com', 'hash2'),
  (gen_random_uuid(), 'charlie', 'charlie@example.com', 'hash3');
EOF
```

### 2. Crea Communities via Web

- Vai su http://localhost:9001/communities
- Usa il form per creare 2-3 communities

### 3. Crea Posts nel Database

```sql
psql $DATABASE_URL << EOF
-- Ottieni IDs
WITH user_id AS (SELECT id FROM users WHERE username = 'alice' LIMIT 1),
     comm_id AS (SELECT id FROM communities LIMIT 1)
INSERT INTO posts (id, community_id, author_id, title, content)
SELECT 
  gen_random_uuid(),
  (SELECT id FROM comm_id),
  (SELECT id FROM user_id),
  'Welcome Post',
  'This is our first post!';
EOF
```

### 4. Verifica Tutto

1. **Test DB**: http://localhost:9001/test-db
2. **Communities**: http://localhost:9001/communities
3. **Community Detail**: Clicca su una community per vedere i posts

## 📊 Monitoraggio

### Logs del Server

Il server mostra logs dettagliati:
```
INFO Connecting to database...
INFO Database connected and migrations complete
INFO Templates loaded successfully
INFO API Gateway listening on http://0.0.0.0:9001
```

### Health Check

```bash
curl http://localhost:9001/health | jq
```

### Database Stats

```bash
# Via web
open http://localhost:9001/test-db

# Via SQL
psql $DATABASE_URL -c "
SELECT 
  (SELECT COUNT(*) FROM users) as users,
  (SELECT COUNT(*) FROM communities) as communities,
  (SELECT COUNT(*) FROM posts) as posts;
"
```

## 🐛 Troubleshooting

### Server non risponde

```bash
# Verifica che sia running
curl http://localhost:9001/health

# Riavvia
lsof -ti:9001 | xargs kill -9
cd src && cargo run --bin server
```

### Pagina vuota

- Verifica che ci siano dati nel database
- Controlla i logs del server
- Prova la pagina test-db per vedere se il DB è connesso

### Database non connesso

```bash
# Testa connessione
psql $DATABASE_URL -c "SELECT 1"

# Verifica .env
grep DATABASE_URL .env
```

## 🎯 Prossimi Passi

1. ✅ Creare utenti nel database
2. ✅ Creare communities via web
3. ✅ Creare posts nel database
4. ✅ Vedere tutto funzionare insieme
5. ⏳ Aggiungere autenticazione Auth0
6. ⏳ Aggiungere API REST complete
7. ⏳ Aggiungere upload immagini

## 📝 Note Importanti

- Le pagine **communities** e **community_detail** sono completamente collegate al database
- La pagina **test-db** mostra dati in tempo reale
- Per creare communities usa il form web
- Per creare utenti e posts usa SQL direttamente (per ora)
- Le API REST sono in sviluppo (alcune query da ottimizzare)

**Il sistema è funzionante e pronto per essere usato!** 🚀
