# Stacked Layout Implementation Tasks

This document provides a detailed task breakdown for implementing the stacked layout components based on the design document.

## Task Overview

| # | Task | Priority | Estimated Complexity |
|---|------|----------|---------------------|
| 1 | Create context system for variant propagation | High | Medium |
| 2 | Implement `StackedLayout` shell component | High | Low |
| 3 | Implement `StackedLayoutHeader` with variant styling | High | Medium |
| 4 | Implement `StackedLayoutContent` with overlap support | High | Low |
| 5 | Implement `StackedLayoutNav` and `StackedLayoutNavItem` | High | Medium |
| 6 | Implement `StackedLayoutSearch` component | Medium | Low |
| 7 | Implement `StackedLayoutFooter` component | Medium | Low |
| 8 | Update module exports | High | Low |
| 9 | Update prelude re-exports | High | Low |
| 10 | Create showcase examples (9 templates) | High | High |
| 11 | Update component manifest | Medium | Low |
| 12 | Run build verification | High | Low |

---

## Task 1: Create Context System for Variant Propagation

**File**: `crates/leptos-daisyui/src/components/layout.rs` (or new `stacked_layout.rs`)

**Description**: Create a context struct to share the layout variant between parent and child components.

```rust
/// Context provided by StackedLayout to child components.
#[derive(Clone, Debug)]
struct StackedLayoutContext {
    variant: StackedLayoutVariant,
    nav_style: StackedLayoutNavStyle,
}
```

**Implementation Steps**:
1. Define `StackedLayoutContext` struct
2. Use `provide_context()` in `StackedLayout` component
3. Use `use_context::<StackedLayoutContext>()` in child components

**Verification**: Context should be accessible in all child components.

---

## Task 2: Implement `StackedLayout` Shell Component

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Create the root shell component with variant and nav style props.

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackedLayoutVariant {
    #[default]
    Light,
    LightBordered,
    Dark,
    Primary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackedLayoutNavStyle {
    #[default]
    Tabs,
    Pills,
    Compact,
    None,
}

#[component]
pub fn StackedLayout(
    children: Children,
    #[prop(optional, default = StackedLayoutVariant::Light)]
    variant: StackedLayoutVariant,
    #[prop(optional, default = StackedLayoutNavStyle::Tabs)]
    nav_style: StackedLayoutNavStyle,
    #[prop(optional, default = false)]
    overlap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes**:
- Base: `min-h-screen flex flex-col`

**Implementation Steps**:
1. Define enums
2. Create component with context provider
3. Render children in flex column container

---

## Task 3: Implement `StackedLayoutHeader`

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Header component that reads variant from context and applies appropriate styling.

```rust
impl StackedLayoutVariant {
    fn header_classes(&self) -> &'static str {
        match self {
            Self::Light => "bg-base-100 text-base-content",
            Self::LightBordered => "bg-base-100 text-base-content border-b border-base-300",
            Self::Dark => "bg-neutral text-neutral-content",
            Self::Primary => "bg-primary text-primary-content",
        }
    }
}

#[component]
pub fn StackedLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes**:
- Base: `navbar` (daisyUI navbar)
- Variant: From `header_classes()`

**Implementation Steps**:
1. Read context for variant
2. Apply variant-specific classes
3. Use semantic `<header>` tag with `navbar` class
4. Support `class` prop for customization

---

## Task 4: Implement `StackedLayoutContent`

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Main content area with optional overlap support.

```rust
#[component]
pub fn StackedLayoutContent(
    children: Children,
    #[prop(optional, default = false)]
    overlap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes**:
- Base: `flex-1`
- Overlap: `-mt-8` (or configurable)
- Container: `container mx-auto` (optional)

**Implementation Steps**:
1. Create component with overlap prop
2. Apply negative margin when overlap is true
3. Use semantic `<main>` tag
4. Support `class` prop for customization

---

## Task 5: Implement `StackedLayoutNav` and `StackedLayoutNavItem`

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Navigation components with variant-aware active states.

```rust
#[component]
pub fn StackedLayoutNav(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView

#[component]
pub fn StackedLayoutNavItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes for Nav**:
- Tabs: `menu menu-horizontal gap-1`
- Pills: `menu menu-horizontal gap-2`
- Compact: `menu menu-horizontal menu-sm gap-1`

**CSS Classes for NavItem** (variant-aware):
```rust
fn nav_item_classes(&self, active: bool, nav_style: StackedLayoutNavStyle) -> String {
    // Implementation based on variant and nav_style
}
```

**Implementation Steps**:
1. Create `StackedLayoutNav` with nav style support
2. Create `StackedLayoutNavItem` with active state
3. Implement variant-aware styling
4. Support both `Tabs` and `Pills` styles

---

## Task 6: Implement `StackedLayoutSearch`

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Search input styled for header integration.

```rust
#[component]
pub fn StackedLayoutSearch(
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes**:
- Container: `relative max-w-md w-full`
- Input: `input input-sm w-full` (variant-aware background)

**Implementation Steps**:
1. Create search input with icon
2. Support variant-aware styling (dark/primary variants need different input styles)
3. Support controlled/uncontrolled patterns

---

## Task 7: Implement `StackedLayoutFooter`

**File**: `crates/leptos-daisyui/src/components/layout.rs`

**Description**: Optional footer component.

```rust
#[component]
pub fn StackedLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**CSS Classes**:
- Base: `border-t border-base-300 py-6`
- Variant: Context-aware background

**Implementation Steps**:
1. Create footer component
2. Read variant from context for styling
3. Use semantic `<footer>` tag

---

## Task 8: Update Module Exports

**File**: `crates/leptos-daisyui/src/components/mod.rs`

**Description**: Add exports for all new components.

```rust
// Add to existing layout exports
pub use layout::{
    // ... existing exports ...
    StackedLayout, StackedLayoutVariant, StackedLayoutNavStyle,
    StackedLayoutHeader, StackedLayoutNav, StackedLayoutNavItem,
    StackedLayoutContent, StackedLayoutSearch, StackedLayoutFooter,
};
```

---

## Task 9: Update Prelude Re-exports

**File**: `crates/leptos-daisyui/src/prelude.rs`

**Description**: Add re-exports for convenient importing.

```rust
pub use crate::components::layout::{
    // ... existing exports ...
    StackedLayout, StackedLayoutVariant, StackedLayoutNavStyle,
    StackedLayoutHeader, StackedLayoutNav, StackedLayoutNavItem,
    StackedLayoutContent, StackedLayoutSearch, StackedLayoutFooter,
};
```

---

## Task 10: Create Showcase Examples

**File**: `crates/showcase/src/pages/stacked_layout_page.rs`

**Description**: Implement all 9 Tailwind Plus template examples.

### Example 1: Simple Header with Nav Tabs (Light)
```rust
<section class="space-y-4">
    <h2 class="text-xl font-semibold">"1. Simple Header with Nav Tabs"</h2>
    <div class="border border-base-300 rounded-box overflow-hidden h-[24rem]">
        <StackedLayout variant=StackedLayoutVariant::Light>
            <StackedLayoutHeader>
                <NavbarStart>
                    <span class="text-xl font-bold px-4">"MyApp"</span>
                </NavbarStart>
                <NavbarCenter>
                    <StackedLayoutNav>
                        <StackedLayoutNavItem href="/" active=true>"Dashboard"</StackedLayoutNavItem>
                        <StackedLayoutNavItem href="/team">"Team"</StackedLayoutNavItem>
                        <StackedLayoutNavItem href="/projects">"Projects"</StackedLayoutNavItem>
                    </StackedLayoutNav>
                </NavbarCenter>
            </StackedLayoutHeader>
            <StackedLayoutContent class="p-6">
                <p>"Content here"</p>
            </StackedLayoutContent>
        </StackedLayout>
    </div>
</section>
```

### Examples to Implement:
1. Simple header with nav tabs (light)
2. Header with border and nav tabs (light)
3. Dark header with nav tabs
4. Indigo branded header with nav tabs
5. Dark header with overlap content area
6. Indigo branded header with overlap content area
7. Indigo branded header with search and secondary nav
8. Dark header with compact nav
9. Indigo header with centered search and footer

---

## Task 11: Update Component Manifest

**File**: `components_manifest/layout.yml`

**Description**: Document all new stacked layout components.

```yaml
  - name: StackedLayout
    description: "Full-width page shell with header, content area, and optional footer"
    base_class: "min-h-screen flex flex-col"
    variants:
      - name: variant
        values:
          - Light
          - LightBordered
          - Dark
          - Primary
      - name: nav_style
        values:
          - Tabs
          - Pills
          - Compact
          - None
    props:
      - variant: StackedLayoutVariant
      - nav_style: StackedLayoutNavStyle
      - overlap: bool
    examples:
      - "Light header with tabs"
      - "Dark header with pills"
      - "Primary branded header"
      - "Header with content overlap"

  - name: StackedLayoutHeader
    description: "Header component with variant-aware styling"
    base_class: "navbar"
    examples:
      - "Header with logo and nav"
      - "Header with search"

  - name: StackedLayoutNav
    description: "Navigation container for stacked layouts"
    base_class: "menu menu-horizontal"
    examples:
      - "Tab-style navigation"
      - "Pill-style navigation"

  - name: StackedLayoutNavItem
    description: "Individual navigation item with active state"
    base_class: ""
    props:
      - href: String
      - active: bool
    examples:
      - "Active nav item"
      - "Inactive nav item"

  - name: StackedLayoutContent
    description: "Main content area with optional overlap"
    base_class: "flex-1"
    props:
      - overlap: bool
    examples:
      - "Standard content area"
      - "Overlapping content area"

  - name: StackedLayoutSearch
    description: "Search input for header integration"
    base_class: "input"
    examples:
      - "Header search"

  - name: StackedLayoutFooter
    description: "Footer component for stacked layouts"
    base_class: "border-t"
    examples:
      - "Simple footer"
```

---

## Task 12: Run Build Verification

**Commands**:
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
cargo check -p leptos-daisyui --features ssr
cargo check -p leptos-daisyui --features csr
cargo check -p leptos-daisyui --features hydrate
cd crates/showcase && cargo leptos build
```

**Success Criteria**:
- No formatting errors
- No clippy warnings
- All feature flags compile
- Showcase app builds successfully

---

## Implementation Order Recommendation

For parallel development, tasks can be grouped:

**Group A - Core Components** (Tasks 1-5):
- Can be implemented together as they form the base system
- Required for all examples

**Group B - Enhancement Components** (Tasks 6-7):
- Can be implemented after Group A
- Required for examples 7 and 9

**Group C - Integration** (Tasks 8-12):
- Must be done after components are implemented
- Final verification step

## Notes

- Keep existing `StackedShell` for backward compatibility
- Use existing `Navbar`, `NavbarStart`, `NavbarCenter`, `NavbarEnd` components
- Follow existing patterns from `sidebar_layout.rs` for context usage
- Ensure all components support `class` prop for customization
- Use semantic HTML tags (`<header>`, `<nav>`, `<main>`, `<footer>`)
