# Civiqo Brand Identity Update Walkthrough

## Panoramica
Questo aggiornamento allinea l'intera applicazione web alle linee guida del **Civiqo Brand Book v1.1**. Abbiamo introdotto la nuova palette colori, i font ufficiali e aggiornato i componenti UI principali.

## Modifiche Implementate

### 1. Configurazione Tailwind e Font
Abbiamo configurato Tailwind CSS con la palette ufficiale Civiqo e aggiunto i font Google:
- **Font Brand**: `Nunito` (per titoli e logo)
- **Font UI**: `Inter` (per il testo generale)
- **Palette**: `civiqo-blue`, `civiqo-teal`, `civiqo-eco-green`, ecc.

### 2. Navbar e Branding (`base.html`)
- **Logo**: Sostituito il testo "Community Manager" con il logo Civiqo stilizzato in CSS.
- **Colori**: Navbar bianca con bordo `civiqo-gray-200`.
- **Link**: Usano `civiqo-gray-600` con hover `civiqo-blue`.
- **Bottoni**: Login/Logout aggiornati allo stile brand.

### 3. Stili Globali (`main.css`)
- Aggiornati i componenti base come `.btn-primary`, `.btn-secondary`, `.chat-message`.
- Le ombreggiature sono state rese più soft.
- I focus ring dei form usano ora il `civiqo-blue`.

### 4. Pagine Aggiornate

#### Dashboard
- Sostituiti i colori generici (blu, verde, viola) con i colori semantici Civiqo:
    - **Communities**: `civiqo-blue`
    - **Welcome**: `civiqo-eco-green`
    - **Email**: `civiqo-lilac`

#### Communities
- Aggiornati tutti i pulsanti e i link.
- Card delle comunità con bordo sottile e hover effect colorato (`civiqo-blue-light`).
- Messaggi di successo/errore allineati alla palette.

#### Landing Page (`index.html`)
- Hero section con font `Nunito` e colori brand.
- Feature grid con icone e testi allineati.

#### Chat (`chat.html`)
- Header e spinner di caricamento aggiornati.

## Verifica Visiva
Le modifiche sono state applicate a livello di codice sostituendo le classi utility di Tailwind.
- **Tipografia**: Verificare che i titoli usino Nunito (più rotondo) e il testo Inter (più pulito).
- **Colori**: Il blu primario è ora `#3B7FBA` (più morbido del blu standard).
- **Logo**: Dovrebbe apparire il simbolo Civiqo nella navbar.

## Prossimi Passi
- Verificare il rendering effettivo nel browser.
- Considerare la creazione di una build pipeline per Tailwind invece del CDN per produzione.
