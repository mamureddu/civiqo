# Prompt: Analisi Completa e Test di una Web App

Copia e incolla questo prompt in una nuova sessione Claude Code per analizzare e testare qualsiasi web app.

---

## Il Prompt

```
Devi fare un'analisi completa e test funzionale di questa repo. Segui questo processo in ordine.

## FASE 1: Analisi della repo

1. Esplora la struttura completa del progetto (directory, file, stack tecnologico)
2. Leggi TUTTI i file di configurazione (Cargo.toml, package.json, .env, docker-compose, etc.)
3. Mappa l'architettura: backend, frontend, database, auth, deploy
4. Identifica TUTTE le route/endpoint (API + pagine)
5. Identifica il sistema di autenticazione e i ruoli utente
6. Identifica le dipendenze esterne (DB, servizi, API terze)
7. Produci un report strutturato con: stack, architettura, route, modelli DB, stato features

## FASE 2: Setup locale

1. Verifica che tutti i prerequisiti siano installati (DB, runtime, dipendenze)
2. Crea/verifica il file .env con configurazione locale
3. Crea il database di sviluppo se non esiste
4. Compila/installa il progetto
5. Avvia il server e verifica che risponda (health check)
6. Se servono dati iniziali (seed, setup wizard), eseguili

## FASE 3: Definizione User Journey

Per OGNI funzionalità dell'app, definisci una user journey dettagliata:

- Auth: registrazione, login, login errato, email duplicata, logout, token refresh
- Pagine pubbliche: ogni pagina accessibile senza login
- Pagine protette: ogni pagina che richiede autenticazione
- CRUD: per ogni entità (post, commenti, utenti, etc.) testa crea/leggi/modifica/elimina
- Interazioni: bottoni, form, HTMX/AJAX, reazioni, ricerca
- Ruoli: testa come admin, come utente normale, come guest
- Edge case: 404, input invalido, permessi negati

## FASE 4: Test con Playwright MCP (Ciclo Ralph Wiggum)

Usa il Playwright MCP per testare OGNI journey nel browser reale.

Per OGNI pagina dell'app:
1. Naviga alla pagina
2. Verifica che il **header** sia corretto (guest vs loggato)
3. Verifica che il **footer** sia presente
4. Verifica che **non ci siano errori console** (0 errors)
5. Verifica che **tutti i dati vengano dal DB** (nessun placeholder, dummy, mock)
6. **Clicca OGNI link** presente nella pagina e verifica che porti a una pagina funzionante (no 404, no crash)
7. **Testa OGNI bottone** e form presente
8. Prendi nota di OGNI problema trovato

Per ogni problema trovato:
- **Identifica la causa** (leggi il codice sorgente)
- **Fai la fix minima**
- **Ricompila** se necessario
- **Ri-testa** la stessa pagina
- Se la fix funziona, **ri-testa anche le pagine già passate** per verificare che non ci siano regressioni

## FASE 5: Test automatizzato dei link

Dopo i test manuali, fai un crawl automatizzato:
1. Login via curl e salva il cookie
2. Visita ogni pagina seed ed estrai TUTTI i link interni (href)
3. Testa OGNI link trovato (status code)
4. Segnala: crash (000), 404, redirect, errori
5. Fix e ri-testa fino a 0 fallimenti

## FASE 6: Test di regressione finale

1. Ri-esegui il test completo di TUTTE le pagine (status code + header corretto)
2. Esegui la test suite del progetto (cargo test / npm test / etc.)
3. Verifica 0 errori di compilazione
4. Produci il report finale con:
   - Tabella di TUTTE le pagine testate (status, header, errori console)
   - Tabella di TUTTI i link testati
   - Lista dei fix applicati
   - Bug rimanenti (se presenti)

## Regole

- NON saltare nessuna pagina
- NON segnare PASSED senza aver verificato TUTTI i punti
- Se trovi un placeholder o dato finto, RIMUOVILO (dati dal DB o nulla)
- Se un handler crasha (risposta vuota), il problema è quasi sempre un type mismatch in sqlx Row::get() o un campo mancante nel template context
- Se un header mostra lo stato sbagliato (guest quando loggato), il handler non passa le variabili auth al template
- Se un form HTMX ritorna JSON raw invece di HTML, il handler non controlla l'header HX-Request
- Ogni fix deve essere seguita da ricompilazione e re-test
```
