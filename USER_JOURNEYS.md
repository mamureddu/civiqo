# User Journeys — Civiqo Community Manager

Ogni journey deve essere testata con Playwright MCP navigando il sito reale su http://localhost:9001.

---

## UJ-1: Registrazione nuovo utente

**Precondizioni**: nessun utente loggato, DB pulito o email non esistente
**Flusso**:
1. Naviga a http://localhost:9001
2. Verifica che la homepage si carichi (titolo visibile, nessun errore 500)
3. Clicca il link "Login" o "Accedi" nella navbar
4. Verifica di essere su /login
5. Clicca il link "Register" / "Registrati" nella pagina login
6. Verifica di essere su /register
7. Compila: name="Test User", email="testuser@example.com", password="TestPass123!", confirm password="TestPass123!"
8. Clicca il bottone di registrazione
9. Verifica redirect a / (homepage)
10. Verifica che la navbar mostri l'utente loggato (nome o email visibile, bottone logout presente)
11. **Successo**: utente registrato e sessione attiva

---

## UJ-2: Login utente esistente

**Precondizioni**: utente "testuser@example.com" gia' registrato (da UJ-1)
**Flusso**:
1. Se loggato, esegui logout prima (POST /auth/logout)
2. Naviga a http://localhost:9001/login
3. Verifica che il form di login sia visibile
4. Compila: email="testuser@example.com", password="TestPass123!"
5. Clicca il bottone di login
6. Verifica redirect a / (homepage)
7. Verifica che la navbar mostri l'utente loggato
8. **Successo**: login effettuato

---

## UJ-3: Login con credenziali errate

**Precondizioni**: nessun utente loggato
**Flusso**:
1. Naviga a http://localhost:9001/login
2. Compila: email="testuser@example.com", password="WrongPassword!"
3. Clicca il bottone di login
4. Verifica di essere ancora su /login (con parametro error nell'URL)
5. Verifica che NON si sia loggato (nessun nome utente nella navbar)
6. **Successo**: login rifiutato, utente rimane su pagina login

---

## UJ-4: Registrazione con email duplicata

**Precondizioni**: utente "testuser@example.com" gia' registrato
**Flusso**:
1. Naviga a http://localhost:9001/register
2. Compila: name="Altro User", email="testuser@example.com", password="AnotherPass123!"
3. Clicca il bottone di registrazione
4. Verifica di essere ancora su /register (con errore email_taken)
5. Verifica che NON si sia loggato
6. **Successo**: registrazione rifiutata per email duplicata

---

## UJ-5: Logout

**Precondizioni**: utente loggato (da UJ-2)
**Flusso**:
1. Verifica che la navbar mostri l'utente loggato
2. Clicca il bottone "Logout" / "Esci"
3. Verifica redirect a / (homepage)
4. Verifica che la navbar mostri il link "Login" (non piu' il nome utente)
5. **Successo**: sessione terminata

---

## UJ-6: Homepage carica correttamente (guest)

**Precondizioni**: nessun utente loggato
**Flusso**:
1. Naviga a http://localhost:9001
2. Verifica che la pagina carichi (status 200)
3. Verifica che il titolo o heading principale sia visibile
4. Verifica che il link "Login" sia visibile nella navbar
5. Verifica che la pagina non mostri errori visibili (nessun testo "error", "500", "panic")
6. **Successo**: homepage accessibile come guest

---

## UJ-7: Dashboard (protetta, richiede login)

**Precondizioni**: utente loggato
**Flusso**:
1. Login come testuser@example.com
2. Naviga a http://localhost:9001/dashboard
3. Verifica che la pagina carichi (non 401/redirect a login)
4. Verifica che il contenuto della dashboard sia visibile
5. **Successo**: dashboard accessibile per utente autenticato

---

## UJ-8: Dashboard redirect per guest

**Precondizioni**: nessun utente loggato
**Flusso**:
1. Naviga a http://localhost:9001/dashboard
2. Verifica il comportamento: o redirect a /login, o mostra errore 401, o mostra pagina con invito a loggarsi
3. **Successo**: dashboard non accessibile senza login

---

## UJ-9: Navigazione pagine principali (guest)

**Precondizioni**: nessun utente loggato
**Flusso**:
1. Naviga a http://localhost:9001 → verifica status 200
2. Naviga a http://localhost:9001/governance → verifica status 200
3. Naviga a http://localhost:9001/businesses → verifica status 200
4. Naviga a http://localhost:9001/login → verifica status 200
5. Naviga a http://localhost:9001/register → verifica status 200
6. **Successo**: tutte le pagine pubbliche caricano senza errori

---

## UJ-10: API login (JSON)

**Precondizioni**: utente "testuser@example.com" registrato
**Flusso**:
1. Invia POST http://localhost:9001/api/auth/login con JSON {"email":"testuser@example.com","password":"TestPass123!"}
2. Verifica risposta 200 con body contenente "token", "token_type":"Bearer", "expires_in"
3. Verifica che il token sia un JWT valido (3 parti separate da punto)
4. **Successo**: API login restituisce JWT

---

## UJ-11: API register (JSON)

**Precondizioni**: email non ancora registrata
**Flusso**:
1. Invia POST http://localhost:9001/api/auth/register con JSON {"email":"apiuser@example.com","password":"ApiPass123!","name":"API User"}
2. Verifica risposta 201 con body contenente "token"
3. **Successo**: API registration restituisce JWT

---

## UJ-12: Pagina 404

**Precondizioni**: nessuna
**Flusso**:
1. Naviga a http://localhost:9001/pagina-che-non-esiste
2. Verifica che la risposta sia gestita (non panic del server)
3. Verifica che la pagina mostri un messaggio comprensibile (404 o "non trovata")
4. **Successo**: errore 404 gestito gracefully
