# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

leptos-daisyui is a Rust library providing Leptos component wrappers for all DaisyUI components. The project is a workspace with two crates:
- `leptos-daisyui`: The component library (crates/leptos-daisyui/)
- `showcase`: A demo/showcase application (crates/showcase/)

This is a pure frontend project using Leptos 0.8 with CSR (Client-Side Rendering) as the primary feature.

## Development Commands

### Building and Running

```bash
# Build the library
cargo build -p leptos-daisyui

# Build the showcase
cargo build -p showcase

# Build all
cargo build

# Run the showcase app (using trunk)
trunk serve --config Trunk.toml

# Build showcase for production
trunk build --release --config Trunk.toml
```

### Testing

```bash
# Run tests for the library
cargo test -p leptos-daisyui

# Run all tests
cargo test
```

### Linting and Formatting

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run clippy for all workspace members
cargo clippy --workspace -- -D warnings
```

### CSS Processing

The project uses Tailwind CSS v4 with DaisyUI. CSS is automatically processed during Trunk builds via a pre_build hook:

```bash
# Manually process CSS (if needed)
npx @tailwindcss/cli -i crates/showcase/style/main.css -o crates/showcase/style/output.css --minify
```

## Architecture

### Workspace Structure

```
crates/
├── leptos-daisyui/    # Component library
│   ├── src/
│   │   ├── components/    # DaisyUI component wrappers
│   │   ├── variants/      # Shared enums (Color, Size, Variant, State)
│   │   └── utils/         # Class merging utilities
│   └── Cargo.toml
└── showcase/          # Demo application
    ├── src/
    │   ├── app.rs         # Main app with router and theme switcher
    │   ├── pages/         # Component showcase pages
    │   └── components/    # Showcase-specific components
    ├── style/
    │   ├── main.css       # Tailwind/DaisyUI config
    │   └── output.css     # Generated CSS (not committed)
    └── Cargo.toml
```

### Component Library Design

**Key principles:**
1. All components follow the "base class + modifiers" pattern
2. Shared variant enums (Color, Size, Variant, State) provide consistent APIs across components
3. Components accept optional `class` prop for custom classes that merge with base classes
4. Components use `Children` prop for flexible content composition
5. Three feature flags: `csr`, `ssr`, `hydrate` (default: none, showcase uses `csr`)

**Component structure example:**
```rust
#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    // ... other props
    children: Children,
) -> impl IntoView
```

**Class merging:**
- Use `class_signal("base-class", &["modifier1", "modifier2"], custom_class)` from `utils::class`
- This merges DaisyUI classes with user-provided classes
- Example: `"btn"` + `["btn-primary", "btn-lg"]` + custom classes

### Showcase Application

The showcase app is a single-page application with:
- **Theme switching**: 35 built-in DaisyUI themes with localStorage persistence
- **Sidebar navigation**: Uses `SidebarLayout` component with categorized nav sections
- **Component pages**: Each component has a dedicated showcase page at `/[component-name]`
- **Live theme preview**: Theme selector shows color swatches for each theme

**Key features:**
- Client-side routing with `leptos_router`
- Theme state managed with Leptos signals
- Reactive active nav item detection using `use_location()`
- Theme applied to `<html data-theme="...">` via `document_element()`

### Variants System

Four shared variant enums provide consistent theming:

1. **Color**: `Primary`, `Secondary`, `Accent`, `Info`, `Success`, `Warning`, `Error`, `Ghost`, `Neutral`
2. **Size**: `Xs`, `Sm`, `Md`, `Lg`
3. **Variant**: `Outline`, `Link`, `Active`, `Disabled`, `Glass`
4. **State**: `Loading`, `Disabled`

Each enum has a `.class(base: &str) -> String` method that returns the appropriate DaisyUI class.

## Component Implementation Guidelines

When adding new components:

1. Create file in `crates/leptos-daisyui/src/components/[name].rs`
2. Add module to `components/mod.rs`
3. Export types in `prelude.rs`
4. Follow the Button component as a template
5. Use shared variant enums where applicable
6. Accept `class` prop for custom styling
7. Use `Children` for flexible content
8. Create a showcase page in `crates/showcase/src/pages/[name]_page.rs`
9. Add route to `app.rs`
10. Add nav item to appropriate category in `NAV_SECTIONS`

## Tailwind CSS Configuration

The project uses Tailwind CSS v4 with the new `@import` syntax:

```css
@import "tailwindcss";
@plugin "daisyui" {
  themes: all;
}

@source "../../crates/leptos-daisyui/src";
@source "../src";
```

The `@source` directives tell Tailwind to scan both the library and showcase for class usage.

## Leptos 0.8 Specifics

This project uses Leptos 0.8 APIs:
- `leptos::prelude::*` for core types
- `Signal::derive()` for derived signals
- `Effect::new()` for side effects
- `mount::mount_to_body()` for CSR mounting
- New `view!` macro syntax
- `Children` type for component children

## Feature Flags

Both crates support three mutually exclusive feature flags:
- `csr`: Client-side rendering (showcase default)
- `ssr`: Server-side rendering
- `hydrate`: Hydration mode

The showcase uses `csr` by default. The library defaults to no features (allows SSR/CSR/hydrate to be chosen by consumer).

## Testing Strategy

When writing tests:
- Component tests should verify class generation
- Test that variant enums produce correct DaisyUI classes
- Test class merging utilities work correctly
- Showcase pages are primarily for visual testing (no automated UI tests currently)

## Common Gotchas

1. **Trunk watches both crates**: Changes to `leptos-daisyui/src` trigger rebuilds
2. **CSS must be regenerated**: The pre_build hook handles this automatically
3. **Theme requires HTML attribute**: DaisyUI themes only work with `data-theme` on `<html>`
4. **LocalStorage is client-only**: Theme persistence uses `#[cfg(any(feature = "csr", feature = "hydrate"))]`
5. **New components need multiple updates**: Component file, mod.rs, prelude.rs, showcase page, app routes, nav sections
