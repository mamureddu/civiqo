# UX Specification: Community Membership System

**Data**: 2 Dicembre 2025  
**Status**: 📋 Da Implementare  
**Priorità**: Alta

---

## 1. Contesto

In single-community mode, ogni utente autenticato può vedere la community ma non è automaticamente membro. Il sistema di membership deve gestire:

- **Chi può iscriversi** (ruoli disponibili all'iscrizione)
- **Come ci si iscrive** (automatico, richiesta, invito)
- **Tipo di community** (pubblica, privata, mista)

---

## 2. Tipi di Community

### 2.1 Community Pubblica
- **Visibilità**: Tutti possono vedere contenuti
- **Iscrizione**: Automatica (click su "Iscriviti")
- **Ruolo default**: `member`

### 2.2 Community Privata
- **Visibilità**: Solo membri vedono contenuti
- **Iscrizione**: Solo su invito o approvazione admin
- **Ruolo default**: `member` (dopo approvazione)

### 2.3 Community Mista (Ibrida)
- **Visibilità**: Contenuti pubblici visibili a tutti, contenuti privati solo ai membri
- **Iscrizione**: Richiesta + approvazione
- **Ruolo default**: `member`

---

## 3. Ruoli e Permessi

### 3.1 Gerarchia Ruoli (ENUM esistente)
```
owner > admin > moderator > member
```

### 3.2 Ruoli Disponibili all'Iscrizione
L'admin può configurare quali ruoli sono disponibili per i nuovi iscritti:

| Configurazione | Descrizione |
|----------------|-------------|
| `member_only` | Solo ruolo `member` (default) |
| `member_or_moderator` | Utente sceglie tra member/moderator |
| `custom` | Admin assegna ruolo manualmente |

---

## 4. Flussi di Iscrizione

### 4.1 Flusso Community Pubblica

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Utente vede    │────▶│  Click          │────▶│  Iscritto come  │
│  "Iscriviti"    │     │  "Iscriviti"    │     │  member         │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

### 4.2 Flusso Community Privata

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Utente vede    │────▶│  Click          │────▶│  Admin riceve   │────▶│  Approvato/     │
│  "Richiedi"     │     │  "Richiedi"     │     │  notifica       │     │  Rifiutato      │
└─────────────────┘     └─────────────────┘     └─────────────────┘     └─────────────────┘
```

### 4.3 Flusso Invito

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Admin invia    │────▶│  Utente riceve  │────▶│  Utente accetta │
│  invito         │     │  email/notifica │     │  → membro       │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

---

## 5. UI Components

### 5.1 Pulsante Iscrizione (community_home.html)

**Stato: Non membro, Community Pubblica**
```html
<button class="btn-primary">
  <icon>user-plus</icon>
  Iscriviti alla community
</button>
```

**Stato: Non membro, Community Privata**
```html
<button class="btn-secondary">
  <icon>lock</icon>
  Richiedi accesso
</button>
```

**Stato: Richiesta in attesa**
```html
<button class="btn-disabled" disabled>
  <icon>clock</icon>
  Richiesta in attesa
</button>
```

**Stato: Già membro**
```html
<div class="badge-success">
  <icon>check</icon>
  Membro
</div>
<button class="btn-ghost text-sm">
  Abbandona
</button>
```

### 5.2 Sezione Admin: Impostazioni Membership

**Path**: `/admin/settings` → Tab "Membership"

```
┌─────────────────────────────────────────────────────────────┐
│ Impostazioni Membership                                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ Tipo di Community                                            │
│ ○ Pubblica - Chiunque può iscriversi                        │
│ ● Privata - Solo su invito o approvazione                   │
│ ○ Mista - Contenuti pubblici, funzioni per membri           │
│                                                              │
│ ─────────────────────────────────────────────────────────── │
│                                                              │
│ Ruolo Default per Nuovi Iscritti                            │
│ [▼ Member                                              ]     │
│                                                              │
│ ─────────────────────────────────────────────────────────── │
│                                                              │
│ Approvazione Richieste                                       │
│ ☑ Richiedi approvazione admin per nuove iscrizioni          │
│ ☐ Invia notifica email agli admin                           │
│                                                              │
│ ─────────────────────────────────────────────────────────── │
│                                                              │
│ [Salva Impostazioni]                                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Database Schema

### 6.1 Nuove Colonne in `communities`

```sql
ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  membership_type VARCHAR(20) DEFAULT 'public' 
  CHECK (membership_type IN ('public', 'private', 'hybrid'));

ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  default_member_role member_role DEFAULT 'member';

ALTER TABLE communities ADD COLUMN IF NOT EXISTS 
  require_approval BOOLEAN DEFAULT false;
```

### 6.2 Tabella `membership_requests` (nuova)

```sql
CREATE TABLE IF NOT EXISTS membership_requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
  requested_role member_role DEFAULT 'member',
  message TEXT,
  reviewed_by UUID REFERENCES users(id),
  reviewed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(community_id, user_id)
);
```

---

## 7. API Endpoints

| Method | Endpoint | Descrizione |
|--------|----------|-------------|
| POST | `/api/communities/{id}/join` | Iscrizione diretta (public) |
| POST | `/api/communities/{id}/request` | Richiesta iscrizione (private) |
| POST | `/api/communities/{id}/leave` | Abbandona community |
| GET | `/api/communities/{id}/requests` | Lista richieste (admin) |
| POST | `/api/communities/{id}/requests/{req_id}/approve` | Approva richiesta |
| POST | `/api/communities/{id}/requests/{req_id}/reject` | Rifiuta richiesta |
| POST | `/api/communities/{id}/invite` | Invia invito (admin) |

---

## 8. Piano di Implementazione

### Fase 1: Base (2-3 ore) ✅ COMPLETATO
1. ✅ Aggiungere pulsante "Iscriviti" in `community_home.html`
2. ✅ Creare endpoint `/htmx/communities/{id}/join`
3. ✅ Aggiornare UI per mostrare stato membership
4. ✅ Pulsante dinamico via HTMX (`/htmx/communities/{id}/membership-button`)

### Fase 2: Tipi Community (2-3 ore) ✅ COMPLETATO
1. ✅ Aggiungere colonne DB per membership settings (migration 010)
2. ✅ Logica condizionale nel pulsante (public vs private)
3. 🔲 Creare UI admin per configurazione (nice-to-have)

### Fase 3: Richieste e Approvazioni (3-4 ore) ✅ COMPLETATO
1. ✅ Endpoint `/htmx/communities/{id}/request` per richieste
2. ✅ Endpoint `/htmx/communities/{id}/requests` per lista (admin)
3. ✅ Endpoint `/htmx/communities/{id}/requests/{user_id}/approve`
4. ✅ Endpoint `/htmx/communities/{id}/requests/{user_id}/reject`
5. 🔲 Notifiche per admin (nice-to-have)

### Fase 4: Inviti (2-3 ore) - PENDING
1. 🔲 Sistema inviti via email
2. 🔲 Link di invito con token

---

## 9. Priorità Immediata

Per sbloccare l'utente ORA, implementare **Fase 1** con community pubblica:

1. Aggiungere pulsante "Iscriviti" visibile se:
   - Utente è loggato
   - Utente NON è già membro

2. Endpoint semplice che:
   - Inserisce record in `community_members`
   - Ruolo = `member`
   - Status = `active`

3. Dopo iscrizione:
   - Mostrare badge "Membro"
   - Abilitare creazione post/proposte
