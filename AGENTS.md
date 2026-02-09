# AGENTS.md — Implementing `leptos-daisyui` (65 components + variations)

This file gives step-by-step instructions for coding agents contributing to the `leptos-daisyui` library.

The project goal is to wrap **all 65 daisyUI components** with their documented variations, producing an ergonomic Leptos library that works in **SSR / CSR / Hydrate / Islands** modes.

Primary references:
- daisyUI components index (65) — https://daisyui.com/components/?lang=en
- daisyUI install/upgrade (Tailwind v4, daisyUI v5, `@plugin "daisyui"`) — https://daisyui.com/docs/install/?lang=en and https://daisyui.com/docs/upgrade/?lang=en
- daisyUI config/prefixing (theme-controller exception) — https://daisyui.com/docs/config/?lang=en
- Leptos Islands guide (`#[island]`, `hydrate_islands`) — https://book.leptos.dev/islands.html
- Tailwind v4 `@source` directive — https://tailwindcss.com/docs/functions-and-directives#source

Optional tooling:
- daisyUI “BLUEPRINT” MCP server (official code generator) — https://daisyui.com/blueprint/
  - Use only if project has a license and the maintainer asks you to.

---

## Core contribution principles

1. **Tier A components must be SSR-safe**
   - No unconditional browser-only APIs.
   - Default components should mostly be markup + class composition.

2. **Always allow escape hatches**
   - Every component takes:
     - `class: MaybeProp<String>`
     - `#[prop(attrs)] attrs: Vec<Attribute>`

3. **Prefer semantic HTML**
   - Use correct base tags (e.g., `<button>`, `<a>`, `<dialog>`, `<nav>`, `<ul>`).
   - Add accessibility attributes where needed.

4. **Do not re-implement daisyUI**
   - No custom CSS.
   - Only classes, structure, and optional helper behaviors where daisyUI docs indicate.

5. **Verify with builds**
   - Every PR must pass:
     - `cargo fmt --all`
     - `cargo clippy --all-targets --all-features -D warnings`
     - `cargo check -p leptos-daisyui --features ssr`
     - `cargo check -p leptos-daisyui --features csr`
     - `cargo check -p leptos-daisyui --features hydrate`
     - demo app: `cargo leptos build` (and ideally release build too)

Project standard: **verify every change with `cargo leptos build`** in the demo app.

---

## Repository structure expectations

- `src/components/**` contains Tier A components.
- `src/interactive/**` contains feature-gated helpers meant for hydration/islands.
- `components_manifest/**` (YAML/TOML) is the checklist for each component’s modifiers and examples.
- A workspace demo app renders every component and variation so Tailwind includes all classes.

If you add a new component:
- create its module file
- export it from `components/mod.rs`
- export it from `prelude.rs`
- add a demo page + navigation entry
- add/update its manifest file

---

## The per-component workflow (do this every time)

### Step 1 — Read the docs page
From the daisyUI docs page:
- Identify the **base class**
- Identify **parts** (sub-element classes)
- Identify **modifiers** (size, color, state, placement, etc.)
- Identify any **recommended semantic tags** or accessibility notes

### Step 2 — Update or create the manifest
Create `components_manifest/<component>.yml`:

- `base`: required classes
- `parts`: subcomponents/classes required
- `modifiers`: all documented class modifiers, grouped by type
- `patterns`: named recipes (e.g., “alignment”, “with icon”, “direction”)
- `examples`: list of example IDs the demo app must render

Do not ship a component without its manifest entry.

### Step 3 — Implement Tier A (pure wrapper) first
A Tier A component:
- renders semantic HTML + required daisyUI classes
- supports `class` merging (`tw_merge`)
- supports attribute passthrough (`attrs`)
- has minimal or no behavioral logic

**Important:** avoid attaching event handlers internally unless necessary.
Consumers can attach `on:*` handlers at the call site, and those can be captured via `attrs`.

### Step 4 — Add typed props only for stable/common modifiers
Use typed props for:
- stable sizes/colors/states
- boolean flags for common toggles

Leave the long tail to `class`.

### Step 5 — Add demo coverage
In the demo app:
- create a page for the component
- render every variation listed in the manifest
- include “parts” examples (e.g., `CardBody`, `CardActions`)
- include a minimal accessibility check (e.g., focusable, disabled)

Tailwind scanning rule: **if it’s not rendered in the demo app, it may not be emitted in CSS**.

### Step 6 — Build matrix verification
Run:
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -D warnings`
- `cargo check -p leptos-daisyui --features ssr`
- `cargo check -p leptos-daisyui --features csr`
- `cargo check -p leptos-daisyui --features hydrate`
- in demo app: `cargo leptos build`

If any step fails, fix before submitting.

---

## Interactive helpers (Tier B) guidelines

Only create Tier B helpers when:
- daisyUI docs describe an imperative behavior (e.g., dialog modal control), or
- you need a controlled/uncontrolled pattern that can’t be expressed with pure markup

### Islands-first design
When implementing Tier B helpers, assume they will be used as islands:
- keep islands small
- move shared markup to Tier A components
- make the controller component the island, not the whole page

### Feature-gate
Put Tier B helpers under:
- `cfg(feature = "hydrate")` or `cfg(any(feature = "csr", feature = "hydrate"))`
- optionally `cfg(feature = "islands")` for islands-specific helpers

Never make Tier A depend on Tier B.

---

## Naming conventions

- Component names are PascalCase: `Button`, `Card`, `Navbar`.
- Subcomponents use clear suffixes: `CardBody`, `CardTitle`, `NavbarStart`.
- Enums use suffixes: `ButtonSize`, `ButtonColor` only when component-specific; otherwise use shared enums.
- Files are snake_case: `button.rs`, `card.rs`.

---

## API conventions (consistency rules)

Every component should follow the same prop ordering:

1. `class`, `attrs`
2. typed variants (`color`, `size`, `variant`, etc.)
3. semantic attributes (`href`, `type`, `value`, etc.)
4. state booleans (`disabled`, `active`, etc.)
5. `children`

Do not add bespoke one-off patterns without discussing first.

---

## Accessibility checklist (minimum)

For each component ensure:

- Correct tag choice (`button` vs `a`).
- `type="button"` default to avoid accidental form submits.
- Disabled semantics:
  - `disabled` attribute for native controls,
  - `aria-disabled="true"` + `tabindex=-1` for links.
- Add `aria-*` passthrough via `attrs` (don’t block it).

---

## Tailwind scanning / `@source` rule

If the component crate is not in the same package root as the app, Tailwind v4 may not scan it automatically. The app must add an `@source` directive pointing to the crate source directory.

Agents:
- whenever adding new variations/classes, verify they appear in the demo app
- if styles are missing, ensure the app’s CSS includes `@source` for the crate

Reference: Tailwind `@source` directive docs.

---

## PR checklist

A PR is ready when it includes:

- [ ] Component module file added/updated
- [ ] `components_manifest/<component>.yml` updated
- [ ] Exports added to `components/mod.rs` and `prelude.rs`
- [ ] Demo page updated with all variations
- [ ] `cargo leptos build` passes
- [ ] `cargo fmt` and `cargo clippy -D warnings` pass
- [ ] SSR/CSR/Hydrate compile checks pass

---

## If something is unclear

Do not guess class names or variants.
Use one of:
- daisyUI official docs page for that component
- (optional) BLUEPRINT MCP server if licensed and enabled
- Leptos book/docs for islands/hydration expectations

If you still can’t find a modifier:
- add it to the manifest as “unknown/needs confirmation”
- implement as raw class example only (escape hatch)
- leave a note in the demo page explaining the gap
