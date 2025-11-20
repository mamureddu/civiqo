# 🚀 API Guide - Come Creare Utenti, Communities e Post

## 📋 Endpoints Disponibili

### Users
- `POST /api/users` - Crea nuovo utente
- `GET /api/users` - Lista tutti gli utenti

### Communities
- `POST /api/communities` - Crea nuova community
- `GET /api/communities` - Lista tutte le communities

### Posts
- `POST /api/communities/:id/posts` - Crea post in una community
- `GET /api/communities/:id/posts` - Lista post di una community

## 🧪 Esempi Pratici

### 1. Creare un Utente

```bash
curl -X POST http://localhost:9001/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mario",
    "email": "mario@example.com",
    "password": "password123"
  }'
```

**Risposta:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "mario",
    "email": "mario@example.com",
    "created_at": "2025-11-20 06:00"
  },
  "message": "User created successfully"
}
```

### 2. Vedere Tutti gli Utenti

```bash
curl http://localhost:9001/api/users | jq
```

**Risposta:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "mario",
    "email": "mario@example.com",
    "created_at": "2025-11-20 06:00"
  }
]
```

### 3. Creare una Community

Prima ottieni l'ID di un utente:
```bash
USER_ID=$(curl -s http://localhost:9001/api/users | jq -r '.[0].id')
echo $USER_ID
```

Poi crea la community:
```bash
curl -X POST http://localhost:9001/api/communities \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"Tech Enthusiasts\",
    \"description\": \"A community for technology lovers\",
    \"created_by\": \"$USER_ID\"
  }"
```

**Risposta:**
```json
{
  "success": true,
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "name": "Tech Enthusiasts",
    "description": "A community for technology lovers",
    "created_at": "2025-11-20 06:05"
  },
  "message": "Community created successfully"
}
```

### 4. Vedere Tutte le Communities

```bash
curl http://localhost:9001/api/communities | jq
```

### 5. Creare un Post in una Community

```bash
# Ottieni IDs
USER_ID=$(curl -s http://localhost:9001/api/users | jq -r '.[0].id')
COMMUNITY_ID=$(curl -s http://localhost:9001/api/communities | jq -r '.[0].id')

# Crea il post
curl -X POST http://localhost:9001/api/communities/$COMMUNITY_ID/posts \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"Welcome to our community!\",
    \"content\": \"This is the first post. Let's build something amazing together!\",
    \"author_id\": \"$USER_ID\"
  }"
```

**Risposta:**
```json
{
  "success": true,
  "data": {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "title": "Welcome to our community!",
    "content": "This is the first post...",
    "created_at": "2025-11-20 06:10"
  },
  "message": "Post created successfully"
}
```

### 6. Vedere Posts di una Community

```bash
COMMUNITY_ID=$(curl -s http://localhost:9001/api/communities | jq -r '.[0].id')
curl http://localhost:9001/api/communities/$COMMUNITY_ID/posts | jq
```

## 🌐 Usare l'Interfaccia Web

### Pagina Communities
**URL**: http://localhost:9001/communities

Questa pagina ora include:
- ✅ **Form per creare community** (in alto)
- ✅ **Lista di tutte le communities** dal database
- ✅ **Link a dettagli community**

**Come usare:**
1. Compila il form "Create New Community"
2. Inserisci nome e descrizione
3. Clicca "Create Community"
4. La pagina si ricarica automaticamente con la nuova community

### Pagina Community Detail
**URL**: http://localhost:9001/communities/:id

Mostra:
- ✅ Dettagli della community
- ✅ Lista dei post
- ✅ Info sul creatore

### Pagina Test Database
**URL**: http://localhost:9001/test-db

Mostra statistiche in tempo reale:
- Numero utenti
- Numero communities
- Numero posts
- Ultimi utenti e communities

## 🔄 Workflow Completo

### Setup Iniziale

```bash
# 1. Crea un utente
curl -X POST http://localhost:9001/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "email": "admin@example.com",
    "password": "admin123"
  }' | jq

# 2. Salva l'ID utente
USER_ID=$(curl -s http://localhost:9001/api/users | jq -r '.[0].id')
echo "User ID: $USER_ID"

# 3. Crea una community
curl -X POST http://localhost:9001/api/communities \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"My First Community\",
    \"description\": \"A test community\",
    \"created_by\": \"$USER_ID\"
  }" | jq

# 4. Salva l'ID community
COMMUNITY_ID=$(curl -s http://localhost:9001/api/communities | jq -r '.[0].id')
echo "Community ID: $COMMUNITY_ID"

# 5. Crea un post
curl -X POST http://localhost:9001/api/communities/$COMMUNITY_ID/posts \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"Hello World\",
    \"content\": \"This is my first post!\",
    \"author_id\": \"$USER_ID\"
  }" | jq

# 6. Verifica tutto
echo "=== USERS ==="
curl -s http://localhost:9001/api/users | jq

echo "=== COMMUNITIES ==="
curl -s http://localhost:9001/api/communities | jq

echo "=== POSTS ==="
curl -s http://localhost:9001/api/communities/$COMMUNITY_ID/posts | jq
```

### Script Automatico

Salva questo come `setup_demo_data.sh`:

```bash
#!/bin/bash

BASE_URL="http://localhost:9001"

echo "🚀 Creating demo data..."

# Create users
echo "Creating users..."
USER1=$(curl -s -X POST $BASE_URL/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"pass123"}' | jq -r '.data.id')

USER2=$(curl -s -X POST $BASE_URL/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"bob","email":"bob@example.com","password":"pass123"}' | jq -r '.data.id')

echo "✅ Created users: $USER1, $USER2"

# Create communities
echo "Creating communities..."
COMM1=$(curl -s -X POST $BASE_URL/api/communities \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Tech Community\",\"description\":\"For tech lovers\",\"created_by\":\"$USER1\"}" | jq -r '.data.id')

COMM2=$(curl -s -X POST $BASE_URL/api/communities \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Gaming Community\",\"description\":\"For gamers\",\"created_by\":\"$USER2\"}" | jq -r '.data.id')

echo "✅ Created communities: $COMM1, $COMM2"

# Create posts
echo "Creating posts..."
curl -s -X POST $BASE_URL/api/communities/$COMM1/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Welcome!\",\"content\":\"Welcome to Tech Community\",\"author_id\":\"$USER1\"}" > /dev/null

curl -s -X POST $BASE_URL/api/communities/$COMM2/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"First Post\",\"content\":\"Hello gamers!\",\"author_id\":\"$USER2\"}" > /dev/null

echo "✅ Created posts"

echo ""
echo "🎉 Demo data created successfully!"
echo ""
echo "📊 View at:"
echo "  - Communities: $BASE_URL/communities"
echo "  - Test DB: $BASE_URL/test-db"
echo "  - API Users: $BASE_URL/api/users"
echo "  - API Communities: $BASE_URL/api/communities"
```

Esegui:
```bash
chmod +x setup_demo_data.sh
./setup_demo_data.sh
```

## 🎨 Pagine Web Disponibili

1. **Homepage**: http://localhost:9001/
2. **Communities** (con form): http://localhost:9001/communities
3. **Community Detail**: http://localhost:9001/communities/:id
4. **Test Database**: http://localhost:9001/test-db
5. **Dashboard**: http://localhost:9001/dashboard
6. **Businesses**: http://localhost:9001/businesses
7. **Governance**: http://localhost:9001/governance

## 🔍 Verificare i Dati

```bash
# Conta totale
curl -s http://localhost:9001/test-db | grep "text-3xl"

# Lista utenti
curl -s http://localhost:9001/api/users | jq '.[].username'

# Lista communities
curl -s http://localhost:9001/api/communities | jq '.[].name'
```

## 🎯 Prossimi Passi

1. ✅ Creare utenti via API
2. ✅ Creare communities via web o API
3. ✅ Creare posts via API
4. ⏳ Aggiungere autenticazione Auth0
5. ⏳ Aggiungere upload immagini
6. ⏳ Aggiungere chat real-time

**Tutto funzionante e pronto per l'uso!** 🚀
