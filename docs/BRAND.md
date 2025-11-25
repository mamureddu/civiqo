# 🎨 Civiqo Brand Guidelines & Compliance

> **Documento di riferimento permanente per le linee guida del brand Civiqo**
> 
> Fonte: `brand_id/Civiqo_Brand_Book_v1.1.pdf`
> 
> Questo documento deve essere sempre consultato per garantire la coerenza visiva e comunicativa del progetto.

## 📋 Indice

1. [Identità Visiva](#identità-visiva)
2. [Palette Colori](#palette-colori)
3. [Logo System](#logo-system)
4. [UI Identity](#ui-identity)
5. [Tipografia](#tipografia)
6. [Tone of Voice](#tone-of-voice)
7. [Brand Narrative](#brand-narrative)
8. [Compliance Analysis](#compliance-analysis)
9. [Asset Locations](#asset-locations)
10. [Implementation Checklist](#implementation-checklist)

---

## Identità Visiva

### Palette Colori Civiqo v1.1

La palette nasce dal blu originale del logo e viene estesa con tonalità civiche, sostenibili e di supporto per la UI.

#### Colori Primari

| Nome | HEX | Uso Principale |
|------|-----|----------------|
| **Civiqo Blue** | `#3B7FBA` | Logo, pulsanti primari, testi istituzionali |
| **Civiqo Blue Dark** | `#285A86` | Navbar, header, testi su sfondo chiaro |
| **Civiqo Blue Light** | `#6FA3D3` | Background soft, hover, card leggere |

#### Colori Secondari

| Nome | HEX | Uso Principale |
|------|-----|----------------|
| **Civiqo Teal** | `#2DA9A1` | Stati informativi, connessione, mappe e comunità |
| **Civiqo Eco Green** | `#3DAA5F` | Sostenibilità ambientale, CER, progetti green |
| **Civiqo Lilac** | `#9B78D3` | Sostenibilità comunicativa/sociale, inclusione |

#### Colori Accent

| Nome | HEX | Uso Principale |
|------|-----|----------------|
| **Civiqo Yellow** | `#F5C542` | Votazioni, governance, avvisi soft |
| **Civiqo Coral** | `#EF6F5E` | Errori, alert, highlight umani |
| **Civiqo Green** | `#57C98A` | Successi, conferme, stati positivi |

#### Colori Neutral

| Nome | HEX | Uso Principale |
|------|-----|----------------|
| **Gray 50** | `#F9FAFB` | Background principale |
| **Gray 100** | `#F3F4F6` | Card, sfondi leggeri |
| **Gray 200** | `#E5E7EB` | Bordo, separatori |
| **Gray 300** | `#D1D5DB` | Placeholder, disabilitati |
| **Gray 500** | `#6B7280` | Testi secondari |
| **Gray 700** | `#374151` | Testi primari |
| **Gray 900** | `#111827` | Testi scuri, header |

---

## Logo System

### Logo Variations

I file logo sono disponibili in `civiqo_assets_structured/logo/`:

- **Logo Full**: Versione completa con testo e icona
- **Logo Icon**: Solo l'icona civica
- **Logo Monochrome**: Versione in bianco/nero
- **Logo Negative**: Versione per sfondi scuri

### Regole di Utilizzo

1. **Spazio Minimo**: Mantenere almeno 1x l'altezza dell'icona come spazio libero
2. **Dimensione Minima**: 32px per digital, 20mm per print
3. **Proporzioni**: Non deformare il logo
4. **Colori**: Usare solo i colori brand definiti
5. **Background**: Preferire sfondi chiari o Civiqo Blue Dark

---

## UI Identity

### Componenti Principali

#### Pulsanti

```css
/* Primary Button */
.btn-primary {
    background-color: #3B7FBA; /* Civiqo Blue */
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    transition: all 0.2s ease;
}

.btn-primary:hover {
    background-color: #285A86; /* Civiqo Blue Dark */
}

/* Secondary Button */
.btn-secondary {
    background-color: transparent;
    color: #3B7FBA; /* Civiqo Blue */
    border: 2px solid #3B7FBA;
}

.btn-secondary:hover {
    background-color: #3B7FBA;
    color: white;
}
```

#### Card e Container

```css
.card {
    background-color: #F9FAFB; /* Gray 50 */
    border: 1px solid #E5E7EB; /* Gray 200 */
    border-radius: 0.75rem;
    padding: 1.5rem;
}

.card-hover:hover {
    border-color: #6FA3D3; /* Civiqo Blue Light */
    box-shadow: 0 4px 6px rgba(59, 127, 186, 0.1);
}
```

#### Navbar

```css
.navbar {
    background-color: #285A86; /* Civiqo Blue Dark */
    color: white;
    padding: 1rem 2rem;
}

.navbar-link {
    color: white;
    text-decoration: none;
    transition: opacity 0.2s ease;
}

.navbar-link:hover {
    opacity: 0.8;
}
```

---

## Tipografia

### Gerarchia Tipografica

#### Font Family
- **Primary**: Inter (o system-ui fallback)
- **Monospace**: JetBrains Mono (per codice)

#### Dimensioni e Pesi

| Elemento | Font Size | Font Weight | Line Height |
|----------|-----------|-------------|-------------|
| **H1 - Title** | 2.25rem (36px) | 700 (Bold) | 1.2 |
| **H2 - Section** | 1.875rem (30px) | 600 (Semibold) | 1.3 |
| **H3 - Subsection** | 1.5rem (24px) | 600 (Semibold) | 1.4 |
| **H4 - Card Title** | 1.25rem (20px) | 600 (Semibold) | 1.5 |
| **Body Large** | 1.125rem (18px) | 400 (Regular) | 1.6 |
| **Body** | 1rem (16px) | 400 (Regular) | 1.6 |
| **Body Small** | 0.875rem (14px) | 400 (Regular) | 1.5 |
| **Caption** | 0.75rem (12px) | 500 (Medium) | 1.4 |

#### Colori Testo

```css
.text-primary {
    color: #374151; /* Gray 700 */
}

.text-secondary {
    color: #6B7280; /* Gray 500 */
}

.text-brand {
    color: #3B7FBA; /* Civiqo Blue */
}

.text-success {
    color: #57C98A; /* Civiqo Green */
}

.text-error {
    color: #EF6F5E; /* Civiqo Coral */
}

.text-warning {
    color: #F5C542; /* Civiqo Yellow */
}
```

---

## Tone of Voice

### Principi Comunicativi

1. **Civico e Inclusivo**: Linguaggio che unisce la comunità
2. **Chiaro e Accessibile**: Evitare gergo tecnico complesso
3. **Positivo e Costruttivo**: Focus su soluzioni e collaborazione
4. **Professionale ma Amichevole**: Equilibrio tra serietà e calore

### Esempi di Copy

#### Call to Action
- "Unisciti alla tua comunità"
- "Partecipa alle decisioni"
- "Scopri cosa succede vicino a te"

#### Messaggi di Successo
- "Benvenuto nella community!"
- "La tua voce è stata ascoltata"
- "Insieme possiamo fare di più"

#### Messaggi di Errore
- "Qualcosa non ha funzionato, riprova"
- "Non abbiamo trovato ciò che cerchi"
- "Serve il tuo aiuto per migliorare"

---

## Brand Narrative

### Missione

"Civiqo connette i cittadini con le loro comunità locali, rendendo la partecipazione civica accessibile, trasparente e impattante."

### Visione

"Un futuro in cui ogni cittadino può contribuire attivamente allo sviluppo della propria comunità attraverso strumenti digitali intuitivi e inclusivi."

### Valori

1. **Partecipazione**: Ogni voce conta
2. **Trasparenza**: Decisioni aperte e comprensibili
3. **Inclusione**: Spazio per tutti, senza barriere
4. **Innovazione**: Tecnologia al servizio della comunità
5. **Sostenibilità**: Città migliori per le generazioni future

---

## Compliance Analysis

### Stato Attuale della Conformità

> **Data analisi**: 2025-11-25

| Categoria | Conformità | Priorità | Azioni Richieste |
|-----------|-----------|----------|------------------|
| **Palette Colori** | ❌ Non conforme | 🔴 Alta | Sostituire colori Tailwind con brand colors |
| **Tipografia** | ⚠️ Parzialmente conforme | 🟡 Media | Implementare gerarchia completa |
| **Logo/Branding** | ❌ Non presente | 🔴 Alta | Integrare logo in navbar e footer |
| **Componenti UI** | ⚠️ Parzialmente conforme | 🟡 Media | Allineare tutti i componenti |
| **Tone of Voice** | ✅ Conforme | 🟢 Bassa | Mantenere stile attuale |

### Issue Critiche da Risolvere

#### 1. Colori Primari Non Conformi

**File**: `src/server/static/styles/main.css`

| Elemento | Colore Attuale | Colore Brand Book | Stato |
|----------|---------------|-------------------|-------|
| Pulsanti primari | `bg-blue-600` | `#3B7FBA` (Civiqo Blue) | ❌ |
| Pulsanti hover | `bg-blue-700` | `#285A86` (Civiqo Blue Dark) | ❌ |
| Link/Brand | `text-indigo-600` | `#3B7FBA` (Civiqo Blue) | ❌ |
| Navbar | `bg-white` | Dovrebbe usare Civiqo Blue Dark | ❌ |

#### 2. Logo Mancante

**File**: `src/server/templates/base.html`

- ❌ Logo Civiqo non presente nella navbar
- ❌ Brand name "Community Manager" invece di "Civiqo"
- ❌ Footer senza branding

#### 3. Componenti UI Non Allineati

- ❌ Card usano Tailwind colors invece di brand colors
- ❌ Stati (success/error/warning) non usano colori brand
- ❌ Hover effects non seguono linee guida

---

## Asset Locations

### Struttura File Brand

```
brand_id/
├── Civiqo_Brand_Book_v1.1.pdf     # Linee guida complete (OBBLIGATORIO)
└── [related files]

civiqo_assets_structured/
├── logo/                          # Variazioni logo
│   ├── civiqo_logo_full.svg
│   ├── civiqo_logo_icon.svg
│   ├── civiqo_logo_monochrome.svg
│   └── civiqo_logo_negative.svg
├── icons/                         # Icone brand-consistent
│   ├── civiqo_icon_chat.svg
│   ├── civiqo_icon_map.svg
│   ├── civiqo_icon_vote.svg
│   └── [altri icone]
├── covers/                        # Layout copertine
│   ├── civiqo_cover_brand_essence.svg
│   ├── civiqo_cover_mvv.svg
│   ├── civiqo_cover_naming_claim.svg
│   └── [altri covers]
└── dividers/                      # Separatori visivi
    └── civiqo_divider.svg
```

### Utilizzo Asset

1. **Sempre fare riferimento al PDF** del brand book per decisioni
2. **Usare asset strutturati** dalle cartelle `civiqo_assets_structured/`
3. **Verificare compliance** durante code review
4. **Documentare deviazioni** (se presenti) con giustificazione

---

## Implementation Checklist

### ✅ Compliance Checklist

- [ ] **Colors match brand hex codes**
  - [ ] Primary buttons use `#3B7FBA`
  - [ ] Hover states use `#285A86`
  - [ ] Success states use `#57C98A`
  - [ ] Error states use `#EF6F5E`
  - [ ] Warning states use `#F5C542`

- [ ] **Typography follows hierarchy**
  - [ ] H1 uses 36px, bold
  - [ ] H2 uses 30px, semibold
  - [ ] Body text uses 16px, regular
  - [ ] Line heights respected
  - [ ] Font family is Inter/system-ui

- [ ] **Logo usage respects guidelines**
  - [ ] Logo present in navbar
  - [ ] Proper spacing maintained
  - [ ] Minimum size respected
  - [ ] Colors follow brand rules

- [ ] **Icons use brand style**
  - [ ] Custom icons from `civiqo_assets_structured/icons/`
  - [ ] Consistent stroke width
  - [ ] Proper color application

- [ ] **Layouts follow brand patterns**
  - [ ] Card designs match brand
  - [ ] Button styles consistent
  - [ ] Navigation follows brand
  - [ ] Assets from structured folders used

### 🚧 Implementation Plan

#### Phase 1: Color System (Priority: Alta)
1. **Update CSS Variables** in `main.css`:
   ```css
   :root {
     --civiqo-blue: #3B7FBA;
     --civiqo-blue-dark: #285A86;
     --civiqo-blue-light: #6FA3D3;
     --civiqo-teal: #2DA9A1;
     --civiqo-eco-green: #3DAA5F;
     --civiqo-lilac: #9B78D3;
     --civiqo-yellow: #F5C542;
     --civiqo-coral: #EF6F5E;
     --civiqo-green: #57C98A;
   }
   ```

2. **Replace Tailwind colors** with CSS variables
3. **Update all button classes** to use brand colors
4. **Update navbar** to use Civiqo Blue Dark

#### Phase 2: Logo Integration (Priority: Alta)
1. **Add logo to navbar** in `base.html`
2. **Update brand name** from "Community Manager" to "Civiqo"
3. **Add footer** with proper branding
4. **Implement favicon** using logo icon

#### Phase 3: Typography (Priority: Media)
1. **Define typography scale** in CSS
2. **Update all headings** to follow hierarchy
3. **Implement proper line heights**
4. **Add font loading** for Inter

#### Phase 4: Component Alignment (Priority: Media)
1. **Update card components** with brand colors
2. **Implement proper hover states**
3. **Add brand icons** where appropriate
4. **Update form elements** styling

### 📋 Testing & Validation

#### Visual Regression Testing
```bash
# Before changes
npm run test:visual:snapshot

# After brand implementation
npm run test:visual:compare
```

#### Manual Testing Checklist
- [ ] Verify all colors match hex codes
- [ ] Check logo visibility and spacing
- [ ] Test responsive behavior
- [ ] Validate accessibility contrast
- [ ] Cross-browser testing

---

## Quick Reference

### CSS Custom Properties

```css
:root {
  /* Primary Colors */
  --civiqo-blue: #3B7FBA;
  --civiqo-blue-dark: #285A86;
  --civiqo-blue-light: #6FA3D3;
  
  /* Secondary Colors */
  --civiqo-teal: #2DA9A1;
  --civiqo-eco-green: #3DAA5F;
  --civiqo-lilac: #9B78D3;
  
  /* Accent Colors */
  --civiqo-yellow: #F5C542;
  --civiqo-coral: #EF6F5E;
  --civiqo-green: #57C98A;
  
  /* Neutral Colors */
  --gray-50: #F9FAFB;
  --gray-100: #F3F4F6;
  --gray-200: #E5E7EB;
  --gray-300: #D1D5DB;
  --gray-500: #6B7280;
  --gray-700: #374151;
  --gray-900: #111827;
}
```

### Common Utility Classes

```css
.bg-brand { background-color: var(--civiqo-blue); }
.bg-brand-dark { background-color: var(--civiqo-blue-dark); }
.bg-brand-light { background-color: var(--civiqo-blue-light); }

.text-brand { color: var(--civiqo-blue); }
.text-brand-dark { color: var(--civiqo-blue-dark); }

.btn-brand {
  background-color: var(--civiqo-blue);
  color: white;
  /* ... other styles */
}

.btn-brand:hover {
  background-color: var(--civiqo-blue-dark);
}
```

---

**Last Updated**: November 25, 2025  
**Brand Book Version**: v1.1  
**Next Review**: After Phase 1 implementation  
**Brand Owner**: Design Team  

> **⚠️ IMPORTANTE**: Questo documento è OBBLIGATORIO per tutti gli sviluppatori. Fare sempre riferimento al PDF completo del brand book per decisioni critiche.
