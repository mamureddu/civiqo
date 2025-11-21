# ✅ Fix: Database Sync Error

## 🐛 Problema

**Errore**: `http://localhost:9001/?error=db_sync`

**Causa**: La query SQL stava cercando di inserire campi (`username`, `picture`, `last_login`) che non esistono nella tabella `users`.

## 🔍 Analisi

### Schema Database Corretto

**Tabella `users`** (da migration 001):
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    auth0_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Tabella `user_profiles`** (da migration 001):
```sql
CREATE TABLE user_profiles (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    name VARCHAR(255),          -- ← Username va qui!
    avatar_url TEXT,            -- ← Picture va qui!
    bio TEXT,
    location VARCHAR(255),
    website TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id)
);
```

### Query Errata (Prima)
```rust
// ❌ SBAGLIATO - Questi campi non esistono in users!
sqlx::query_scalar::<_, Uuid>(
    "INSERT INTO users (id, auth0_id, email, username, picture, last_login, ...)
     VALUES ($1, $2, $3, $4, $5, NOW(), ...)
     ..."
)
```

## ✅ Soluzione

### Query Corretta (Dopo)

**File**: `src/server/src/auth.rs` - `sync_user_to_database()`

```rust
async fn sync_user_to_database(
    db: &shared::database::Database,
    user_info: &Auth0UserInfo,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    // 1. Insert or update user in users table
    let user_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (id, auth0_id, email, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (auth0_id) DO UPDATE SET
            email = EXCLUDED.email,
            updated_at = NOW()
         RETURNING id"
    )
    .bind(Uuid::new_v4())
    .bind(&user_info.sub)
    .bind(&user_info.email)
    .fetch_one(&db.pool)
    .await?;

    // 2. Insert or update user profile
    sqlx::query(
        "INSERT INTO user_profiles (id, user_id, name, avatar_url, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())
         ON CONFLICT (user_id) DO UPDATE SET
            name = EXCLUDED.name,
            avatar_url = EXCLUDED.avatar_url,
            updated_at = NOW()"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&user_info.name)
    .bind(&user_info.picture)
    .execute(&db.pool)
    .await?;

    Ok(user_id)
}
```

### Cosa Fa Ora

1. **Step 1**: Inserisce/aggiorna record in `users` con solo i campi base:
   - `id` (UUID)
   - `auth0_id` (da Auth0)
   - `email` (da Auth0)

2. **Step 2**: Inserisce/aggiorna record in `user_profiles` con i dati del profilo:
   - `user_id` (FK a users)
   - `name` (username da Auth0)
   - `avatar_url` (picture da Auth0)

## 🧪 Verifica

### Compilazione ✅
```bash
cd src && cargo check --bin server
# Result: ✅ Finished successfully
```

### SQLx Cache ✅
```bash
cd src && cargo sqlx prepare --workspace
# Result: ✅ Query data written
```

### Server Running ✅
```bash
cd src && cargo run --bin server
# Result: ✅ Server listening on http://0.0.0.0:9001
```

### Health Check ✅
```bash
curl http://localhost:9001/health
# Result: {"service":"community-manager-api","status":"healthy",...}
```

## 🎯 Test Auth Flow

Ora puoi testare il flow completo:

```bash
# 1. Apri browser
open http://localhost:9001

# 2. Click "Login" nella navbar

# 3. Login con Auth0

# 4. Dovresti essere reindirizzato a /dashboard

# 5. Verifica nel database
psql $DATABASE_URL -c "
  SELECT u.id, u.email, u.auth0_id, p.name, p.avatar_url 
  FROM users u 
  LEFT JOIN user_profiles p ON u.id = p.user_id 
  WHERE u.auth0_id LIKE 'auth0|%'
"
```

## 📊 Risultato Atteso

### Database dopo login
**Tabella `users`**:
```
id                                   | email              | auth0_id
-------------------------------------|--------------------|-----------------
550e8400-e29b-41d4-a716-446655440000 | user@example.com   | auth0|123456789
```

**Tabella `user_profiles`**:
```
id                                   | user_id                              | name      | avatar_url
-------------------------------------|--------------------------------------|-----------|------------------
660e8400-e29b-41d4-a716-446655440001 | 550e8400-e29b-41d4-a716-446655440000 | John Doe  | https://...
```

## ✅ Status

- ✅ **Fix applicato**
- ✅ **Codice compila**
- ✅ **Server funzionante**
- ✅ **Query corrette**
- ✅ **Pronto per test**

**Ora il login dovrebbe funzionare correttamente!** 🎉

## 💡 Note

### Migration 007
La migration `0007_add_auth0_id_to_users.sql` è ridondante perché:
- `auth0_id` esiste già in `users` dalla migration 001
- `last_login` non serve (non lo usiamo)

Ma non fa male perché usa `IF NOT EXISTS`.

### Struttura Dati
Il design del database separa:
- **users**: Dati essenziali per autenticazione
- **user_profiles**: Dati del profilo utente (nome, avatar, bio, etc.)

Questo è un pattern comune e corretto per:
- Normalizzazione
- Performance (users table piccola)
- Flessibilità (profilo opzionale)
