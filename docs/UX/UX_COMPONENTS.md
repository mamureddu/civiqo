# Civiqo Design System & Components

> Catalogo dei componenti UI riutilizzabili e pattern di design.

**Ultimo aggiornamento**: 2025-11-27  
**Versione**: 1.0.0  
**Maintainer**: Agente UX

---

## 🎨 Fondamenti

### Colori

#### Primari
| Nome | Hex | Uso |
|------|-----|-----|
| Civiqo Blue | `#2563EB` | CTA primarie, link, focus |
| Civiqo Green | `#57C98A` | Successo, conferme, online |
| Civiqo Coral | `#FF6B6B` | Alert, urgenza, errori |

#### Neutri
| Nome | Hex | Uso |
|------|-----|-----|
| Gray 900 | `#111827` | Testo principale |
| Gray 600 | `#4B5563` | Testo secondario |
| Gray 400 | `#9CA3AF` | Placeholder, disabled |
| Gray 200 | `#E5E7EB` | Bordi, divisori |
| Gray 50 | `#F9FAFB` | Background secondario |
| White | `#FFFFFF` | Background principale |

#### Semantici
| Nome | Colore | Uso |
|------|--------|-----|
| Success | Green | Operazioni completate |
| Warning | `#F59E0B` | Attenzione richiesta |
| Error | Coral | Errori, azioni distruttive |
| Info | Blue | Informazioni, suggerimenti |

### Tipografia

```css
/* Font Family */
--font-brand: 'Inter', system-ui, sans-serif;
--font-body: 'Inter', system-ui, sans-serif;

/* Scale */
--text-xs: 0.75rem;    /* 12px - Caption, badge */
--text-sm: 0.875rem;   /* 14px - Secondary text */
--text-base: 1rem;     /* 16px - Body */
--text-lg: 1.125rem;   /* 18px - Lead text */
--text-xl: 1.25rem;    /* 20px - H4 */
--text-2xl: 1.5rem;    /* 24px - H3 */
--text-3xl: 1.875rem;  /* 30px - H2 */
--text-4xl: 2.25rem;   /* 36px - H1 */

/* Weight */
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;
```

### Spaziatura

```css
/* Scale (Tailwind-based) */
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;      /* 16px */
--space-5: 1.25rem;   /* 20px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
--space-10: 2.5rem;   /* 40px */
--space-12: 3rem;     /* 48px */
```

### Border Radius

```css
--radius-sm: 0.25rem;   /* 4px - Piccoli elementi */
--radius-md: 0.5rem;    /* 8px - Card, input */
--radius-lg: 0.75rem;   /* 12px - Modal, panel */
--radius-xl: 1rem;      /* 16px - Large cards */
--radius-full: 9999px;  /* Pill, avatar */
```

### Ombre

```css
--shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
--shadow-md: 0 4px 6px rgba(0,0,0,0.1);
--shadow-lg: 0 10px 15px rgba(0,0,0,0.1);
--shadow-xl: 0 20px 25px rgba(0,0,0,0.15);
```

---

## 🧩 Componenti

### Buttons

#### Primary
```html
<button class="px-4 py-2 bg-civiqo-blue text-white rounded-lg 
               hover:bg-civiqo-blue/90 transition font-medium">
    Label
</button>
```
- **Uso**: Azione principale della pagina
- **Stati**: default, hover, focus, disabled, loading

#### Secondary
```html
<button class="px-4 py-2 border border-civiqo-gray-200 text-civiqo-gray-700 
               rounded-lg hover:bg-civiqo-gray-50 transition font-medium">
    Label
</button>
```
- **Uso**: Azioni secondarie, cancel

#### Danger
```html
<button class="px-4 py-2 bg-civiqo-coral text-white rounded-lg 
               hover:bg-civiqo-coral/90 transition font-medium">
    Delete
</button>
```
- **Uso**: Azioni distruttive (con conferma)

#### Ghost
```html
<button class="px-4 py-2 text-civiqo-blue hover:bg-civiqo-blue/10 
               rounded-lg transition font-medium">
    Label
</button>
```
- **Uso**: Azioni terziarie, link-like

#### Icon Button
```html
<button class="p-2 text-civiqo-gray-600 hover:bg-civiqo-gray-100 
               rounded-lg transition">
    <svg class="w-5 h-5">...</svg>
</button>
```

### Form Elements

#### Text Input
```html
<input type="text" 
       class="w-full px-3 py-2 border border-civiqo-gray-200 rounded-lg 
              focus:ring-2 focus:ring-civiqo-blue focus:border-civiqo-blue
              placeholder:text-civiqo-gray-400"
       placeholder="Placeholder text">
```

#### Textarea
```html
<textarea rows="4"
          class="w-full px-3 py-2 border border-civiqo-gray-200 rounded-lg 
                 focus:ring-2 focus:ring-civiqo-blue focus:border-civiqo-blue
                 placeholder:text-civiqo-gray-400 resize-none">
</textarea>
```

#### Select
```html
<select class="w-full px-3 py-2 border border-civiqo-gray-200 rounded-lg 
               focus:ring-2 focus:ring-civiqo-blue focus:border-civiqo-blue">
    <option>Option 1</option>
</select>
```

#### Label
```html
<label class="block text-sm font-medium text-civiqo-gray-700 mb-1">
    Field Label
</label>
```

#### Error State
```html
<input class="... border-civiqo-coral focus:ring-civiqo-coral">
<p class="mt-1 text-sm text-civiqo-coral">Error message</p>
```

### Cards

#### Base Card
```html
<div class="bg-white rounded-lg shadow-sm border border-civiqo-gray-200 p-4">
    Content
</div>
```

#### Interactive Card
```html
<div class="bg-white rounded-lg shadow-sm border border-civiqo-gray-200 p-4
            hover:shadow-md hover:border-civiqo-blue/50 transition cursor-pointer">
    Content
</div>
```

#### Community Card
```html
<div class="bg-white rounded-lg shadow-sm border border-civiqo-gray-200 overflow-hidden">
    <div class="h-24 bg-gradient-to-r from-civiqo-blue to-civiqo-green"></div>
    <div class="p-4">
        <h3 class="font-semibold text-civiqo-gray-900">Community Name</h3>
        <p class="text-sm text-civiqo-gray-600 mt-1">Description...</p>
        <div class="flex items-center justify-between mt-4">
            <span class="text-xs text-civiqo-gray-600">👥 123 members</span>
            <button class="...">Join</button>
        </div>
    </div>
</div>
```

### Badges

#### Status Badges
```html
<!-- Active/Success -->
<span class="px-2 py-1 text-xs font-medium bg-civiqo-green/10 text-civiqo-green rounded-full">
    Active
</span>

<!-- Draft/Pending -->
<span class="px-2 py-1 text-xs font-medium bg-civiqo-gray-200 text-civiqo-gray-600 rounded-full">
    Draft
</span>

<!-- Closed/Info -->
<span class="px-2 py-1 text-xs font-medium bg-civiqo-blue/10 text-civiqo-blue rounded-full">
    Closed
</span>

<!-- Alert/Urgent -->
<span class="px-2 py-1 text-xs font-medium bg-civiqo-coral/10 text-civiqo-coral rounded-full">
    Urgent
</span>
```

#### Counter Badge
```html
<span class="inline-flex items-center justify-center w-5 h-5 text-xs font-bold 
             bg-civiqo-coral text-white rounded-full">
    3
</span>
```

### Modal / Dialog

```html
<dialog class="rounded-xl shadow-xl max-w-md w-full p-0 backdrop:bg-black/50">
    <div class="p-6">
        <!-- Header -->
        <div class="flex items-center justify-between mb-4">
            <h3 class="text-xl font-bold text-civiqo-gray-900">Modal Title</h3>
            <button onclick="this.closest('dialog').close()" 
                    class="text-civiqo-gray-400 hover:text-civiqo-gray-600">
                <svg class="w-6 h-6"><!-- X icon --></svg>
            </button>
        </div>
        
        <!-- Content -->
        <div class="mb-6">
            Content here
        </div>
        
        <!-- Footer -->
        <div class="flex justify-end space-x-3">
            <button class="...secondary">Cancel</button>
            <button class="...primary">Confirm</button>
        </div>
    </div>
</dialog>
```

### Tabs

```html
<div class="border-b border-civiqo-gray-200">
    <nav class="flex space-x-8">
        <button class="py-4 px-1 border-b-2 border-civiqo-blue text-civiqo-blue font-medium">
            Active Tab
        </button>
        <button class="py-4 px-1 border-b-2 border-transparent text-civiqo-gray-600 
                       hover:text-civiqo-gray-900 hover:border-civiqo-gray-300">
            Inactive Tab
        </button>
    </nav>
</div>
```

### Loading States

#### Spinner
```html
<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-civiqo-blue"></div>
```

#### Skeleton
```html
<div class="animate-pulse">
    <div class="h-4 bg-civiqo-gray-200 rounded w-3/4 mb-2"></div>
    <div class="h-4 bg-civiqo-gray-200 rounded w-1/2"></div>
</div>
```

#### Button Loading
```html
<button class="... opacity-75 cursor-not-allowed" disabled>
    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 inline">...</svg>
    Loading...
</button>
```

### Empty States

```html
<div class="text-center py-12">
    <svg class="mx-auto h-12 w-12 text-civiqo-gray-400"><!-- Icon --></svg>
    <h3 class="mt-4 text-lg font-medium text-civiqo-gray-900">No items yet</h3>
    <p class="mt-2 text-civiqo-gray-600">Get started by creating your first item.</p>
    <button class="mt-4 ...primary">Create Item</button>
</div>
```

### Alerts / Toasts

```html
<!-- Success -->
<div class="p-4 bg-civiqo-green/10 text-civiqo-green rounded-lg flex items-center">
    <svg class="w-5 h-5 mr-2"><!-- Check icon --></svg>
    Operation completed successfully!
</div>

<!-- Error -->
<div class="p-4 bg-civiqo-coral/10 text-civiqo-coral rounded-lg flex items-center">
    <svg class="w-5 h-5 mr-2"><!-- X icon --></svg>
    Something went wrong. Please try again.
</div>

<!-- Info -->
<div class="p-4 bg-civiqo-blue/10 text-civiqo-blue rounded-lg flex items-center">
    <svg class="w-5 h-5 mr-2"><!-- Info icon --></svg>
    Here's some helpful information.
</div>
```

---

## 🔄 Pattern HTMX

### Loading Indicator
```html
<div hx-get="/api/data" 
     hx-trigger="load"
     hx-indicator="#loading">
    <div id="loading" class="htmx-indicator">
        <!-- Spinner -->
    </div>
    <div id="content">
        <!-- Content loads here -->
    </div>
</div>
```

### Optimistic UI
```html
<button hx-post="/api/action"
        hx-swap="outerHTML"
        hx-on::before-request="this.classList.add('opacity-50')"
        hx-on::after-request="this.classList.remove('opacity-50')">
    Action
</button>
```

### Form Submission
```html
<form hx-post="/api/create"
      hx-target="#result"
      hx-swap="innerHTML"
      hx-on::after-request="if(event.detail.successful) this.reset()">
    <!-- Fields -->
    <button type="submit">Submit</button>
</form>
<div id="result"></div>
```

---

## 📱 Responsive Breakpoints

```css
/* Tailwind defaults */
sm: 640px   /* Mobile landscape */
md: 768px   /* Tablet */
lg: 1024px  /* Desktop */
xl: 1280px  /* Large desktop */
2xl: 1536px /* Extra large */
```

### Mobile-First Patterns

```html
<!-- Stack on mobile, row on desktop -->
<div class="flex flex-col md:flex-row">

<!-- Hide on mobile -->
<div class="hidden md:block">

<!-- Full width on mobile, constrained on desktop -->
<div class="w-full md:max-w-md">
```

---

## ♿ Accessibilità

### Focus Visible
```css
.focus-visible:focus {
    outline: 2px solid var(--civiqo-blue);
    outline-offset: 2px;
}
```

### Screen Reader Only
```html
<span class="sr-only">Descriptive text for screen readers</span>
```

### ARIA Labels
```html
<button aria-label="Close modal">
    <svg><!-- X icon --></svg>
</button>

<nav aria-label="Main navigation">
    <!-- Nav items -->
</nav>

<div role="alert" aria-live="polite">
    <!-- Dynamic content -->
</div>
```

---

*Documento mantenuto da Agente UX. Per aggiungere componenti, invocare `@Agente UX`.*
