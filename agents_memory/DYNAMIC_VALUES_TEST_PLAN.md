# Piano Test Valori Dinamici

**Data**: 26 Novembre 2025  
**Stato**: In Progress  
**Obiettivo**: Verificare che tutti i valori dinamici nelle pagine siano correttamente popolati dal database

---

## Analisi Template Completata

### Template Analizzati

| Template | Valori Dinamici | Priorità |
|----------|-----------------|----------|
| `community_detail.html` | member_count, post_count, community_name, description | ✅ FATTO |
| `dashboard.html` | communities_count, username, email, communities list, activity | Alta |
| `communities.html` | communities list (name, description, creator_name, created_at) | Alta |
| `post_detail.html` | post.title, post.content, post.author_name, comments count, view_count | Alta |
| `governance.html` | Active Proposals (12), Passed (45), Participants (1,234), Ending Soon (3) | Media |
| `businesses.html` | businesses list | Media |
| `fragments/community-card.html` | community.name, community.member_count, community.description | Alta |
| `fragments/post-card.html` | post.title, reaction_count, comment_count, view_count | Alta |
| `fragments/members-list.html` | total members, member details, pagination | Alta |
| `index.html` | Recent communities (via HTMX) | Media |

---

## Test da Implementare

### Gruppo 1: Dashboard (Priorità Alta)

#### Test 1: `test_view_interaction_dashboard_dynamic_stats`
**Cosa testare**:
- `{{ communities_count }}` - Numero community create dall'utente
- `{{ username }}` - Nome utente
- `{{ email }}` - Email utente

**Setup**:
1. Creare utente di test
2. Creare 2 community per quell'utente
3. Simulare sessione autenticata (difficile senza mock)

**Nota**: Richiede autenticazione - potrebbe essere necessario testare via API o mock

---

### Gruppo 2: Communities List (Priorità Alta)

#### Test 2: `test_view_interaction_communities_list_shows_data`
**Cosa testare**:
- Lista community mostra nome, descrizione, creator_name
- HTMX fragment `/htmx/communities/list` ritorna le community dal DB

**Setup**:
1. Creare 2 community con nomi univoci
2. Verificare che appaiano nel fragment HTMX

**Implementazione**:
```rust
#[tokio::test]
#[serial]
async fn test_view_interaction_communities_list_shows_data() {
    // 1. Setup: creare community con nomi univoci
    // 2. GET /htmx/communities/list
    // 3. Verificare che i nomi appaiano nella risposta
}
```

---

### Gruppo 3: Community Card Fragment (Priorità Alta)

#### Test 3: `test_view_interaction_community_card_member_count`
**Cosa testare**:
- `{{ community.member_count }}` mostra il conteggio corretto

**Setup**:
1. Creare community
2. Aggiungere 5 membri
3. Verificare che il fragment mostri "5 members"

---

### Gruppo 4: Post Detail (Priorità Alta)

#### Test 4: `test_view_interaction_post_detail_shows_data`
**Cosa testare**:
- `{{ post.title }}` - Titolo del post
- `{{ post.content }}` - Contenuto
- `{{ post.author_name }}` - Nome autore
- `{{ comments | length }}` - Conteggio commenti
- `{{ post.view_count }}` - Visualizzazioni

**Setup**:
1. Creare community, utente, post con titolo univoco
2. Aggiungere 3 commenti al post
3. GET /posts/{id}
4. Verificare titolo, autore, "Commenti (3)"

---

### Gruppo 5: Post Card Fragment (Priorità Alta)

#### Test 5: `test_view_interaction_post_card_stats`
**Cosa testare**:
- `{{ post.reaction_count }}` - Conteggio reazioni
- `{{ post.comment_count }}` - Conteggio commenti

**Setup**:
1. Creare post
2. Aggiungere 2 commenti
3. Verificare che il fragment mostri i conteggi

---

### Gruppo 6: Members List Fragment (Priorità Alta)

#### Test 6: `test_view_interaction_members_list_pagination`
**Cosa testare**:
- `{{ total }}` - Totale membri
- Lista membri con email, role, joined_at
- Paginazione funzionante

**Setup**:
1. Creare community con 5 membri
2. GET /api/communities/{id}/members?page=1&limit=10
3. Verificare "Members (5)" e lista

---

### Gruppo 7: Governance (Priorità Media - HARDCODED)

#### Test 7: `test_view_interaction_governance_stats_hardcoded`
**Nota**: Attualmente i valori sono HARDCODED (12, 45, 1234, 3)
**Azione**: Documentare che sono hardcoded, creare issue per fix futuro

---

### Gruppo 8: Index Recent Communities (Priorità Media)

#### Test 8: `test_view_interaction_index_recent_communities`
**Cosa testare**:
- HTMX fragment `/htmx/communities/recent` ritorna community recenti

**Setup**:
1. Creare 2 community
2. GET /htmx/communities/recent
3. Verificare che appaiano

---

## Ordine di Implementazione

1. ✅ `test_view_interaction_06b_community_dynamic_stats` - FATTO
2. ✅ `test_view_interaction_06c_community_feed_shows_posts` - FATTO
3. ✅ `test_view_interaction_06d_communities_list_shows_data` - FATTO
4. ✅ `test_view_interaction_06e_index_recent_communities` - FATTO
5. ✅ `test_view_interaction_06f_members_list_shows_data` - FATTO
6. ✅ `test_view_interaction_06g_post_detail_shows_data` - FATTO

## Fix Implementati

### Handler HTMX Fixati (erano hardcoded!)
- ✅ `recent_communities` - Ora legge dal DB
- ✅ `communities_list` - Ora legge dal DB con supporto search

### Template Fix
- ✅ Sostituito `| first` con `| truncate(length=1)` in tutti i template (Tera `first` funziona solo su array)
- ✅ Rimosso `| date()` filter dove `created_at` è già formattato come stringa
- ✅ Aggiunto `user_id` al context anche per utenti non autenticati
- ✅ Gestione nullable fields (`is_pinned`, `is_locked`, `view_count`) con `Option<T>`

---

## Note Tecniche

### Pattern di Test
```rust
#[tokio::test]
#[serial]
async fn test_view_interaction_XXX() {
    let db = setup_db().await;
    
    // 1. Setup: Creare dati con prefisso TEST_RUN_ID
    let slug = format!("{}_xxx_test", *TEST_RUN_ID);
    
    // 2. Creare entità nel DB
    
    // 3. Fare richiesta HTTP
    let server = create_server().await;
    let response = server.get("/path").await;
    
    // 4. Verificare risposta
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("expected_value"));
    
    // 5. Cleanup (opzionale, zz_cleanup lo fa comunque)
}
```

### Cleanup Automatico
Tutti i dati con prefisso `__test_runner_<uuid>_` vengono puliti da `test_view_interaction_zz_cleanup`

### Esecuzione
```bash
# Per cleanup garantito
cargo test view_interaction -p server -- --test-threads=1

# Per esecuzione veloce
cargo test view_interaction -p server
```

---

## Checklist Agent 1

- [ ] Implementare test 3-8
- [ ] Verificare che tutti passino
- [ ] Aggiornare documentazione test
- [ ] Segnalare valori hardcoded trovati (governance.html)

---

## Valori Hardcoded Trovati (da fixare in futuro)

| File | Valore | Linea |
|------|--------|-------|
| `governance.html` | Active Proposals: 12 | 24 |
| `governance.html` | Passed: 45 | 38 |
| `governance.html` | Participants: 1,234 | 52 |
| `governance.html` | Ending Soon: 3 | 66 |

Questi richiedono handler dedicati con query al DB.
