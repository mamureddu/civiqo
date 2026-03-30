# Ralph Wiggum Loop — User Journey Testing

## Istruzioni

Devi testare TUTTE le user journey definite in `USER_JOURNEYS.md` usando il Playwright MCP.
Segui il ciclo Ralph Wiggum: **test → fail → fix → retest** finche' TUTTE passano.

## Setup

1. Il server DEVE girare su http://localhost:9001. Se non gira, avvialo:
   ```
   cd src && cargo run -p server
   ```
   Aspetta che stampi "API Gateway listening on http://0.0.0.0:9001"

2. Usa il Playwright MCP per navigare il browser reale su localhost:9001

## Procedura per OGNI user journey

Per ogni UJ-N nel file USER_JOURNEYS.md:

### Step 1: TEST
- Esegui la user journey passo per passo usando Playwright MCP
- Naviga, clicca, compila form, verifica risultati
- Prendi uno snapshot dopo ogni azione critica per verificare lo stato

### Step 2: VALUTA
- La journey e' passata? Tutti i punti di verifica soddisfatti?
- Se SI → segna come PASSED, vai alla prossima journey
- Se NO → vai a Step 3

### Step 3: FIX
- Identifica la causa del fallimento (errore nel template? handler? route? CSS?)
- Leggi il codice sorgente rilevante
- Fai la fix minima necessaria
- Se hai modificato Rust: ricompila (`cargo build -p server`) e riavvia il server

### Step 4: RETEST
- Riesegui la STESSA journey da capo
- Se passa → PASSED, vai alla prossima
- Se fallisce di nuovo → torna a Step 3

## Ordine di esecuzione

Esegui le journey in questo ordine (dipendenze):
1. UJ-6: Homepage guest (baseline — il sito carica?)
2. UJ-9: Navigazione pagine pubbliche
3. UJ-12: Pagina 404
4. UJ-1: Registrazione (crea l'utente per i test successivi)
5. UJ-4: Registrazione duplicata
6. UJ-5: Logout
7. UJ-2: Login
8. UJ-3: Login credenziali errate
9. UJ-7: Dashboard protetta
10. UJ-8: Dashboard guest redirect
11. UJ-10: API login JSON
12. UJ-11: API register JSON

## Tracking

Mantieni una tabella di stato aggiornata:

| Journey | Status | Tentativi | Note |
|---------|--------|-----------|------|
| UJ-6   | ?      | 0         |      |
| UJ-9   | ?      | 0         |      |
| ...     | ...    | ...       |      |

Aggiorna dopo ogni test/fix cycle.

## Regole

- NON skippare nessuna journey
- NON segnare PASSED se non hai verificato TUTTI i punti
- Se una fix rompe una journey gia' passata, devi ri-testarla
- Quando TUTTE sono PASSED, stampa il report finale
- Se dopo 5 tentativi una journey non passa, documenta il blocco e vai avanti
