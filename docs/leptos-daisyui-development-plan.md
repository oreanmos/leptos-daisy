# leptos-daisyui — Development Plan (65 components + variations)

This document is a practical, implementation-oriented plan for building a **Leptos component library** that wraps **all 65 daisyUI components** (and their documented variations), designed to work cleanly across:

- **SSR** (`ssr` feature)
- **CSR** (`csr` feature)
- **Hydration** (`hydrate` feature)
- **Islands** (`islands` feature; selective hydration)

References used while writing this plan (official docs):
- daisyUI components list (65 components) — https://daisyui.com/components/?lang=en
- daisyUI v5 upgrade + Tailwind v4 integration (`@import "tailwindcss"; @plugin "daisyui";`) — https://daisyui.com/docs/upgrade/?lang=en and https://daisyui.com/docs/install/?lang=en
- daisyUI config (prefixing, theme-controller exception) — https://daisyui.com/docs/config/?lang=en
- daisyUI modal methods (dialog recommended, showModal/close) — https://daisyui.com/components/modal/?lang=en
- Tailwind v4 `@source` directive (scan extra paths, e.g., workspace crates) — https://tailwindcss.com/docs/functions-and-directives#source
- Leptos Islands guide (`#[island]`, `hydrate_islands`, `HydrationScripts islands=true`) — https://book.leptos.dev/islands.html
- Leptos SSR lifecycle (“dehydrated” HTML then hydrate) — https://book.leptos.dev/ssr/22_life_cycle.html

---

## 0) Goals and non-goals

### Goals
1. **Complete coverage** of daisyUI’s 65 components, including “parts” (e.g., `card-body`, `navbar-start`) and all documented variants/modifiers.
2. **Leptos-first API**: ergonomic props + strong defaults, while never preventing “escape hatches” (raw classes + attributes).
3. **Rendering-mode friendly**:
   - pure wrappers compile on **SSR** with no Web APIs required,
   - optional interactive helpers for **CSR/Hydrate/Islands** that don’t break SSR builds.
4. **Theme + prefix aware**: supports daisyUI prefixing and Tailwind prefixing configurations.
5. **Reusable in new projects**: prelude exports, demo/playground, consistent naming, semver discipline.

### Non-goals
- Re-implementing daisyUI styling. The library only composes class names and markup.
- Providing a full design system beyond daisyUI (e.g., custom tokens). Consumers can add these later.

---

## 1) Baseline technical decisions

### 1.1 daisyUI + Tailwind versions
Target **daisyUI v5** and **Tailwind CSS v4** (CSS-first plugin setup):

```css
@import "tailwindcss";
@plugin "daisyui";
```

Optional theme enabling is also done in CSS via the plugin block.

### 1.2 Workspace recommendation (important for Tailwind scanning)
Tailwind generates CSS by scanning files for class tokens. To ensure Tailwind sees daisyUI classes inside your component crate, **keep the component crate in the same workspace** as the app, or instruct consumers to add an `@source` directive pointing at the crate sources.

Example:

```css
@import "tailwindcss";
@source "../crates/leptos-daisyui/src";
@plugin "daisyui";
```

This matters for both SSR and CSR builds.

### 1.3 Feature flags
Provide feature flags that mirror Leptos’ build modes:

- `ssr`
- `csr`
- `hydrate`
- `islands` (optional; typically used alongside `hydrate`)

Avoid unconditional dependencies on browser-only APIs in the core crate.

---

## 2) Library architecture

### 2.1 Proposed crate layout

```
leptos-daisyui/
  src/
    lib.rs
    prelude.rs
    util/
      mod.rs
      class.rs
      attrs.rs
      aria.rs
    variants/
      mod.rs
      color.rs
      size.rs
      state.rs
    components/
      mod.rs
      actions/
      data_display/
      navigation/
      feedback/
      data_input/
      layout/
      mockup/
    interactive/        # optional helpers (feature-gated)
      mod.rs
      modal_dialog.rs
      dropdown_controller.rs
```

### 2.2 The “two-tier” component model
To stay SSR-safe and still be ergonomic:

**Tier A — Pure wrappers (default)**
- Only render markup and class names.
- Always accept `class` and `#[prop(attrs)] attrs`.
- Do **not** require browser APIs.
- Interactivity is achieved by users attaching events via normal Leptos attributes at the call site (or via `attrs` spread).

**Tier B — Interactive helpers (optional)**
- Provide controlled/uncontrolled APIs (signals) for components that have canonical imperative behaviors (e.g., dialog modals).
- Feature-gate under `hydrate` or `csr` (and optionally `islands`).

This avoids “SSR compile explosions” caused by unconditional DOM/event types.

---

## 3) Core design rules (reusability + completeness)

### Rule 1 — Always provide the escape hatch
Every component must accept:

- `class: MaybeProp<String>`
- `#[prop(attrs)] attrs: Vec<Attribute>`

This ensures:
- any new daisyUI modifier not yet wrapped can still be used immediately;
- consumers can attach any ARIA/data attributes without waiting on library changes.

### Rule 2 — Prefer semantic props, but never block composition
For frequently used variants, provide typed props:

- `color`, `size`, `variant`, `shape`, `state`, etc.

But do not attempt to model **everything** as enums. Some modifiers are combinatorial or change rapidly. The escape hatch covers the long tail.

### Rule 3 — Do not hard-code JS behavior into base components
daisyUI is mostly CSS classes. When there are multiple ways to open/close something (e.g., modal dialog vs checkbox), expose both patterns:

- a pure wrapper for each pattern, plus
- a small helper in `interactive/` for ergonomic control.

### Rule 4 — Accessibility is not optional
For each component:
- choose the most semantic HTML element available
- expose ARIA props where needed
- provide safe defaults (e.g., `type="button"` for buttons)
- ensure disabled states map to correct attributes and aria semantics

---

## 4) Variants system

### 4.1 Common variant enums
Create shared enums for the common daisyUI patterns:

- `Color`: `Primary`, `Secondary`, `Accent`, `Neutral`, `Info`, `Success`, `Warning`, `Error`
- `Size`: `Xs`, `Sm`, `Md`, `Lg`
- `Variant`: `Solid`, `Outline`, `Ghost`, `Link`, plus component-specific variants where necessary
- `State`: `Active`, `Disabled`, `Loading` (if applicable)

### 4.2 Class composition strategy
Use a tiny utility that merges:
- base class
- classes from enums
- conditional modifiers
- user-provided `class`

Use `tw_merge` (or equivalent) so consumers can override safely.

### 4.3 Prefix-aware class building
daisyUI supports prefixing; Tailwind can also prefix utilities. daisyUI docs note `theme-controller` has special prefixing behavior.

Provide a `DaisyConfig` context:

```rust
pub struct DaisyConfig {
  pub daisy_prefix: Option<&'static str>,   // e.g. "d-"
  pub tailwind_prefix: Option<&'static str> // e.g. "tw:"
}
```

Then a `cx()` helper:

- `cx("btn") -> "btn"` by default
- `cx("btn") -> "tw:d-btn"` if configured
- special-case `theme-controller` per docs

Also allow overriding config per component via direct `class`.

---

## 5) Islands support: “small islands, big reuse”
In islands mode, only code inside an `#[island]` is shipped to the browser. The library should:

- keep **Tier A** components 100% reusable (SSR-only safe)
- place any signal-driven interactive logic in **Tier B** components that are intended to be used as islands

Example pattern:
- `Modal` (pure wrapper) renders correct daisyUI markup
- `ModalDialogIsland` (island) manages `open` signal and calls `showModal()` when hydrated

Leptos islands usage patterns:
- enable `islands` feature in `leptos`
- call `leptos::mount::hydrate_islands()`
- add `islands=true` to `HydrationScripts`
- convert interactive components to `#[island]` where appropriate

---

## 6) Component inventory (65 daisyUI components)

Pulled from official daisyUI component index (65 total):

### Actions
- Button
- Dropdown
- FAB / Speed Dial
- Modal
- Swap
- Theme Controller

### Data display
- Accordion
- Avatar
- Badge
- Card
- Carousel
- Chat bubble
- Collapse
- Countdown
- Diff
- Hover 3D card
- Hover Gallery
- Kbd
- List
- Stat
- Status
- Table
- Text Rotate
- Timeline

### Navigation
- Breadcrumbs
- Dock
- Link
- Menu
- Navbar
- Pagination
- Steps
- Tabs

### Feedback
- Alert
- Loading
- Progress
- Radial progress
- Skeleton
- Toast
- Tooltip

### Data input
- Calendar
- Checkbox
- Fieldset
- File Input
- Filter
- Label
- Radio
- Range
- Rating
- Select
- Text Input
- Textarea
- Toggle
- Validator

### Layout
- Divider
- Drawer sidebar
- Footer
- Hero
- Indicator
- Join (group items)
- Mask
- Stack

### Mockup
- Browser mockup
- Code mockup
- Phone mockup
- Window mockup

---

## 7) “Variations” coverage: how to make it complete without drowning

The fastest path to completeness is to treat every component as:

1) **Base** class
2) **Modifiers** (size, color, state, placement, etc.)
3) **Parts** (sub-elements with their own required classes)
4) **Patterns** (documented “recipes”, e.g., tooltip directions, dropdown alignment)

### 7.1 Build a manifest for each component
Create a `components_manifest/` directory (YAML or TOML) that agents update, e.g.:

```yaml
name: Button
doc: https://daisyui.com/components/button/
base: ["btn"]
parts: []
modifiers:
  color: ["btn-primary","btn-secondary","btn-accent","btn-neutral","btn-info","btn-success","btn-warning","btn-error"]
  size: ["btn-xs","btn-sm","btn-md","btn-lg"]
  variant: ["btn-outline","btn-ghost","btn-link","btn-soft","btn-dash"]
  state: ["btn-active","btn-disabled","btn-loading"]
patterns:
  - "icon-only"
  - "button-group (join)"
```

This manifest becomes:
- the single source of truth for “what variations exist”
- the basis for docs pages in the demo app
- a checklist for agents

### 7.2 Choose the right “API shape” per modifier type
Not all modifiers deserve enums. Use this rule of thumb:

- **Stable and common** → typed enums/booleans
- **Rare, experimental, combinatorial** → keep as raw `class`

---

## 8) Interactive components: dual-mode patterns

### 8.1 Modal
daisyUI documents **dialog-based modals** as recommended, controlled via `showModal()` / `close()`.

Plan:
- `Modal` (pure) renders `<dialog class="modal">…</dialog>` and slots for `ModalBox`, `ModalAction`.
- `ModalCheckbox` (pure) renders legacy checkbox pattern for no-JS / SSR-only use.
- `interactive::ModalDialogController` (feature-gated) provides:
  - `open: RwSignal<bool>`
  - NodeRef to call `showModal()` on hydrate/island.

### 8.2 Dropdown / Drawer
Provide:
- pure wrappers that render correct daisyUI structure
- optional controller helpers if needed (but keep them small and island-friendly)

---

## 9) Implementation workflow (repeatable per component)

For each component:

1. **Read daisyUI docs page**
   - identify base class, parts, modifiers, accessibility notes
2. **Create component module**
   - `#[component]` function
   - props: `class`, `attrs`, plus key typed variants
3. **Render semantic HTML**
   - select correct tag(s): `button`, `a`, `nav`, `ul/li`, `dialog`, etc.
4. **Add doc examples**
   - add a demo page/route in the demo app
   - include examples for every modifier in the manifest
5. **Verify builds**
   - `cargo fmt`
   - `cargo clippy -D warnings` (project standard)
   - `cargo check -p leptos-daisyui --features ssr`
   - `cargo check -p leptos-daisyui --features csr`
   - `cargo check -p leptos-daisyui --features hydrate`
   - demo app: `cargo leptos build` and (if applicable) `cargo leptos build --release`
6. **Update manifest + coverage checklist**
   - mark variations completed
   - link to doc page and any caveats

---

## 10) Demo/playground app (mandatory)

Create a small Leptos app in the workspace that:
- imports the library via path dependency
- includes Tailwind + daisyUI
- generates a page per component (like a mini Storybook)
- ensures Tailwind sees every variation class (important!)

This app is also where you validate SSR/hydration/islands behavior.

---

## 11) Release plan (milestones)

### Milestone A — Scaffold + utilities
- crate layout
- `DaisyConfig` + class builder
- `prelude` exports
- demo app skeleton

### Milestone B — Tier A complete coverage
- implement all 65 as **pure wrappers**
- include parts (subcomponents) as needed
- each component has demo page + minimal examples

### Milestone C — Typed variants for top 20
Prioritize what you’ll use constantly:
- Button, Badge, Alert, Card
- Input, Select, Textarea, Checkbox, Radio, Toggle
- Navbar, Menu, Tabs, Dropdown
- Modal, Drawer
- Table, Pagination
- Toast, Tooltip, Loading

### Milestone D — Islands-friendly helpers
Add feature-gated `interactive/` controllers for:
- Modal dialog
- Dropdown controller (optional)
- Drawer controller (optional)

Keep islands small: only behavior code should be inside `#[island]`.

### Milestone E — Publish (optional)
- docs in README
- versioning rules
- consumer notes on Tailwind scanning (`@source`)

---

## 12) Definition of Done (DoD)

A component is “done” when:

- ✅ Base wrapper exists with `class` + `attrs`
- ✅ All documented modifiers are represented either:
  - as typed props, or
  - as documented raw-class patterns in the manifest
- ✅ Demo page shows every variation
- ✅ Compiles under `ssr`, `csr`, `hydrate` (and `islands` if applicable)
- ✅ Accessibility notes addressed (tag choice, aria attrs, disabled behavior)
- ✅ `cargo leptos build` passes for the demo app
