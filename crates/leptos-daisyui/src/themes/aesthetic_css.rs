//! CSS constants for the aesthetic system.
//!
//! Injected via `AestheticStyles` component. Contains:
//! - `:root` default tokens (clean-minimal baseline)
//! - `[data-aesthetic="..."]` overrides for each preset
//! - DaisyUI bridge rules mapping aesthetic tokens to DaisyUI variables
//! - Utility classes: `.aesthetic-card`, `.aesthetic-page`, `.aesthetic-section`

/// Complete CSS for the aesthetic token system.
///
/// Include via [`AestheticStyles`] component at app root.
pub const AESTHETIC_CSS: &str = r#"
/* ── Default aesthetic tokens (clean-minimal baseline) ─────── */
:root {
  --font-heading: ui-sans-serif, system-ui, -apple-system, sans-serif;
  --font-body: ui-sans-serif, system-ui, -apple-system, sans-serif;
  --font-mono: ui-monospace, 'Fira Code', 'Cascadia Code', monospace;
  --radius-card: 0.5rem;
  --radius-btn: 0.375rem;
  --radius-input: 0.375rem;
  --shadow-card: 0 1px 2px rgba(0,0,0,0.05);
  --shadow-card-hover: 0 4px 8px rgba(0,0,0,0.1);
  --spacing-page-x: 1.5rem;
  --spacing-page-y: 2rem;
  --spacing-section: 1.5rem;
  --border-card: 1px solid oklch(var(--bc) / 0.1);
  --transition-card: all 200ms ease;
}

/* ── TUI Aesthetic ────────────────────────────────────── */
[data-aesthetic="tui"] {
  --font-heading: ui-monospace, 'Fira Code', 'Cascadia Code', monospace;
  --font-body: ui-monospace, 'Fira Code', 'Cascadia Code', monospace;
  --radius-card: 0;
  --radius-btn: 0;
  --radius-input: 0;
  --shadow-card: none;
  --shadow-card-hover: none;
  --spacing-page-x: 1rem;
  --spacing-page-y: 1rem;
  --spacing-section: 0.75rem;
  --border-card: 1px solid oklch(var(--bc) / 0.2);
  --transition-card: none;
}

/* ── Cozy Journal Aesthetic ───────────────────────────── */
[data-aesthetic="journal"] {
  --font-heading: Georgia, 'Times New Roman', serif;
  --font-body: Georgia, 'Times New Roman', serif;
  --radius-card: 0.75rem;
  --radius-btn: 0.5rem;
  --radius-input: 0.5rem;
  --shadow-card: 0 2px 8px rgba(0,0,0,0.06);
  --shadow-card-hover: 0 6px 20px rgba(0,0,0,0.1);
  --spacing-page-x: 2rem;
  --spacing-page-y: 2.5rem;
  --spacing-section: 1.75rem;
  --border-card: 1px solid oklch(var(--bc) / 0.08);
  --transition-card: all 300ms ease;
}

/* ── Obsidian Aesthetic ───────────────────────────────── */
[data-aesthetic="obsidian"] {
  --font-heading: ui-sans-serif, system-ui, sans-serif;
  --font-body: ui-sans-serif, system-ui, sans-serif;
  --radius-card: 0.375rem;
  --radius-btn: 0.25rem;
  --radius-input: 0.25rem;
  --shadow-card: 0 1px 3px rgba(0,0,0,0.3);
  --shadow-card-hover: 0 4px 12px rgba(0,0,0,0.4);
  --spacing-page-x: 1.5rem;
  --spacing-page-y: 1.5rem;
  --spacing-section: 1rem;
  --border-card: 1px solid oklch(var(--bc) / 0.15);
  --transition-card: all 150ms ease;
}

/* ── Clean Minimal Aesthetic (same as :root defaults) ── */
[data-aesthetic="minimal"] {
  --font-heading: ui-sans-serif, system-ui, -apple-system, sans-serif;
  --font-body: ui-sans-serif, system-ui, -apple-system, sans-serif;
  --radius-card: 0.5rem;
  --radius-btn: 0.375rem;
  --radius-input: 0.375rem;
  --shadow-card: 0 1px 2px rgba(0,0,0,0.05);
  --shadow-card-hover: 0 4px 8px rgba(0,0,0,0.1);
  --spacing-page-x: 1.5rem;
  --spacing-page-y: 2rem;
  --spacing-section: 1.5rem;
  --border-card: 1px solid oklch(var(--bc) / 0.1);
  --transition-card: all 200ms ease;
}

/* ── Synthwave Neon Aesthetic ─────────────────────────── */
[data-aesthetic="synthwave-neon"] {
  --font-heading: 'Inter', 'Segoe UI', system-ui, sans-serif;
  --font-body: ui-sans-serif, system-ui, sans-serif;
  --radius-card: 0.25rem;
  --radius-btn: 0.25rem;
  --radius-input: 0.25rem;
  --shadow-card: 0 0 12px rgba(255,0,255,0.15), 0 0 4px rgba(0,255,255,0.1);
  --shadow-card-hover: 0 0 24px rgba(255,0,255,0.25), 0 0 8px rgba(0,255,255,0.2);
  --spacing-page-x: 1.5rem;
  --spacing-page-y: 1.5rem;
  --spacing-section: 1.25rem;
  --border-card: 1px solid oklch(var(--bc) / 0.15);
  --transition-card: all 200ms ease;
}

/* ── Paper Manuscript Aesthetic ────────────────────────── */
[data-aesthetic="paper"] {
  --font-heading: 'Palatino Linotype', Palatino, 'Book Antiqua', serif;
  --font-body: 'Palatino Linotype', Palatino, 'Book Antiqua', serif;
  --font-mono: 'Courier New', Courier, monospace;
  --radius-card: 0.125rem;
  --radius-btn: 0.125rem;
  --radius-input: 0.125rem;
  --shadow-card: 2px 2px 0 oklch(var(--bc) / 0.15);
  --shadow-card-hover: 3px 3px 0 oklch(var(--bc) / 0.2);
  --spacing-page-x: 2rem;
  --spacing-page-y: 2.5rem;
  --spacing-section: 2rem;
  --border-card: 1px solid oklch(var(--bc) / 0.2);
  --transition-card: all 150ms ease;
}

/* ── Brutalist Aesthetic ──────────────────────────────── */
[data-aesthetic="brutalist"] {
  --font-heading: ui-monospace, 'Courier New', monospace;
  --font-body: ui-monospace, 'Courier New', monospace;
  --font-mono: ui-monospace, 'Courier New', monospace;
  --radius-card: 0;
  --radius-btn: 0;
  --radius-input: 0;
  --shadow-card: 4px 4px 0 oklch(var(--bc) / 0.8);
  --shadow-card-hover: 6px 6px 0 oklch(var(--bc) / 0.9);
  --spacing-page-x: 1.5rem;
  --spacing-page-y: 1.5rem;
  --spacing-section: 1.5rem;
  --border-card: 2px solid oklch(var(--bc) / 0.8);
  --transition-card: none;
}

/* ── Glass Frost Aesthetic ────────────────────────────── */
[data-aesthetic="glass"] {
  --font-heading: 'Inter', ui-sans-serif, system-ui, sans-serif;
  --font-body: 'Inter', ui-sans-serif, system-ui, sans-serif;
  --radius-card: 1rem;
  --radius-btn: 0.75rem;
  --radius-input: 0.75rem;
  --shadow-card: 0 4px 16px rgba(0,0,0,0.06);
  --shadow-card-hover: 0 8px 32px rgba(0,0,0,0.1);
  --spacing-page-x: 2rem;
  --spacing-page-y: 2rem;
  --spacing-section: 1.75rem;
  --border-card: 1px solid oklch(var(--bc) / 0.08);
  --transition-card: all 250ms ease;
}

/* ── Ink Calligraphy Aesthetic ─────────────────────────── */
[data-aesthetic="ink"] {
  --font-heading: 'Didot', 'Bodoni MT', 'Noto Serif Display', serif;
  --font-body: 'Garamond', 'Noto Serif', Georgia, serif;
  --radius-card: 0;
  --radius-btn: 0;
  --radius-input: 0;
  --shadow-card: none;
  --shadow-card-hover: 0 1px 4px rgba(0,0,0,0.08);
  --spacing-page-x: 2rem;
  --spacing-page-y: 2.5rem;
  --spacing-section: 2rem;
  --border-card: 1px solid oklch(var(--bc) / 0.12);
  --transition-card: all 200ms ease;
}

/* ── Typography rules ─────────────────────────────────── */
body {
  font-family: var(--font-body);
}

h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-heading);
}

code, pre, .font-mono {
  font-family: var(--font-mono);
}

/* ── DaisyUI radius bridge ────────────────────────────── */
/* Maps aesthetic tokens to DaisyUI's built-in radius variables so ALL
   DaisyUI components (card, alert, collapse, modal-box, tabs-box, btn,
   badge, etc.) honor the aesthetic tokens automatically. */
[data-theme] {
  --rounded-box: var(--radius-card);
  --rounded-btn: var(--radius-btn);
  --rounded-badge: var(--radius-btn);
}

/* ── Form element radius overrides ────────────────────── */
.input, .textarea, .select, .file-input {
  border-radius: var(--radius-input);
}

.btn {
  border-radius: var(--radius-btn);
}

.btn-circle {
  border-radius: 9999px;
}

.menu :where(li:not(.menu-title)) > :not(ul, details) {
  border-radius: var(--radius-btn);
}

.progress {
  border-radius: var(--radius-input);
}

/* ── Utility classes ──────────────────────────────────── */
.aesthetic-card {
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
  border: var(--border-card);
  transition: var(--transition-card);
}

.aesthetic-card:hover {
  box-shadow: var(--shadow-card-hover);
}

.aesthetic-page {
  padding-left: var(--spacing-page-x);
  padding-right: var(--spacing-page-x);
  padding-top: var(--spacing-page-y);
  padding-bottom: var(--spacing-page-y);
}

.aesthetic-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-section);
}
"#;
