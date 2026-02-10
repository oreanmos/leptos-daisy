# Stacked Layout Component Design Document

## Overview

This document describes the design for adapting Tailwind Plus Elements "stacked layout" templates to daisyUI and integrating them into the leptos-daisyui showcase app.

The user provided 9 HTML stacked layout examples from Tailwind Plus:
1. Simple header with nav tabs (light)
2. Header with border and nav tabs (light)
3. Dark header with nav tabs
4. Indigo branded header with nav tabs
5. Dark header with overlap content area
6. Indigo branded header with overlap content area
7. Indigo branded header with search and secondary nav
8. Dark header with compact nav
9. Indigo header with centered search and footer

## Key Patterns Analysis

### Common Structure
All stacked layouts share this structure:
```
┌─────────────────────────────────────────┐
│ Header                                  │
│ ┌─────────────────────────────────────┐ │
│ │ Logo │ Nav │ Actions               │ │
│ └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│ Main Content Area                       │
│                                         │
│                                         │
│                                         │
├─────────────────────────────────────────┤
│ Footer (optional)                       │
└─────────────────────────────────────────┘
```

### Header Variants

| Variant | Background | Text Color | Border |
|---------|------------|------------|--------|
| Light | `bg-base-100` | `text-base-content` | None or `border-b border-base-300` |
| Dark | `bg-neutral` | `text-neutral-content` | None |
| Branded (Primary) | `bg-primary` | `text-primary-content` | None |

### Navigation Patterns

1. **Horizontal Tabs**: `menu menu-horizontal` with `bg-tabs` classes
2. **Pills**: Rounded nav items with active state highlighting
3. **Compact**: Minimal padding, icon-only on mobile
4. **With Search**: Search input in navbar-center
5. **Secondary Nav**: Additional nav below main header

### Content Overlap Pattern
Some layouts feature a negative margin content area that overlaps the header:
```
┌─────────────────────────────────────────┐
│ Header (dark/primary background)        │
│                                         │
├─────────────────────────────────────────┤ ← Header ends
│                                         │
│   ┌─────────────────────────────────┐   │
│   │ Content Card (overlaps header)  │   │
│   │ margin-top: -4rem               │   │
│   └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

## Existing Components Analysis

### Current StackedShell (layout.rs)
```rust
#[component]
pub fn StackedShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```
- Basic flex column container with `min-h-screen`
- No built-in header or content structure

### Current PageHeader (layout.rs)
```rust
#[component]
pub fn PageHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView

#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView

#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

### Current Navbar (navbar.rs)
```rust
#[component]
pub fn Navbar(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView

#[component]
pub fn NavbarStart(...)
#[component]
pub fn NavbarCenter(...)
#[component]
pub fn NavbarEnd(...)
```

## Component Architecture

### New Components to Create

#### 1. `StackedLayout` (Enhanced Shell)

A comprehensive stacked layout shell with built-in header support and variants.

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackedLayoutVariant {
    /// Light background with default text
    #[default]
    Light,
    /// Light with bottom border
    LightBordered,
    /// Dark neutral background
    Dark,
    /// Primary/branded background
    Primary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackedLayoutNavStyle {
    /// Simple horizontal menu
    #[default]
    Tabs,
    /// Rounded pill-style navigation
    Pills,
    /// Compact minimal navigation
    Compact,
    /// No built-in nav (custom)
    None,
}

#[component]
pub fn StackedLayout(
    children: Children,
    /// Layout variant: Light, LightBordered, Dark, Primary
    #[prop(optional, default = StackedLayoutVariant::Light)]
    variant: StackedLayoutVariant,
    /// Navigation style: Tabs, Pills, Compact, None
    #[prop(optional, default = StackedLayoutNavStyle::Tabs)]
    nav_style: StackedLayoutNavStyle,
    /// Enable content overlap effect (negative margin)
    #[prop(optional, default = false)]
    overlap: bool,
    /// Show footer
    #[prop(optional, default = false)]
    footer: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

#### 2. `StackedLayoutHeader`

The header component with variant-aware styling.

```rust
#[component]
pub fn StackedLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Context Integration**: Reads `StackedLayoutVariant` from parent context to apply correct background/text colors.

#### 3. `StackedLayoutNav`

Navigation container with style variants.

```rust
#[component]
pub fn StackedLayoutNav(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

#### 4. `StackedLayoutNavItem`

Individual navigation item with active state.

```rust
#[component]
pub fn StackedLayoutNavItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

#### 5. `StackedLayoutContent`

Main content area with optional overlap support.

```rust
#[component]
pub fn StackedLayoutContent(
    children: Children,
    /// Apply negative margin to overlap header
    #[prop(optional, default = false)]
    overlap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

#### 6. `StackedLayoutFooter`

Optional footer component.

```rust
#[component]
pub fn StackedLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

#### 7. `StackedLayoutSearch`

Search input component for header integration.

```rust
#[component]
pub fn StackedLayoutSearch(
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

### Component Hierarchy

```
StackedLayout (root shell)
├── StackedLayoutHeader
│   ├── NavbarStart (logo)
│   ├── NavbarCenter (nav/search)
│   │   └── StackedLayoutNav
│   │       └── StackedLayoutNavItem[]
│   └── NavbarEnd (actions)
├── StackedLayoutContent
│   └── PageHeader (optional)
│       ├── PageHeaderTitle
│       └── PageHeaderActions
│   └── main content
└── StackedLayoutFooter (optional)
```

## Class Mapping: Tailwind Plus → daisyUI

### Header Classes

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `bg-white` | `bg-base-100` |
| `bg-gray-900` | `bg-neutral` |
| `bg-indigo-600` | `bg-primary` |
| `border-b border-gray-200` | `border-b border-base-300` |
| `shadow-sm` | `shadow-sm` (same) |

### Navigation Classes

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `flex space-x-8` | `menu menu-horizontal gap-8` |
| `border-b-2 border-indigo-500` | `menu-active` or `border-b-2 border-primary` |
| `text-sm font-medium` | `text-sm font-medium` (same) |
| `rounded-md px-3 py-2` | `rounded-lg px-3 py-2` (same) |
| `bg-indigo-700` (active) | `bg-primary-focus` or `active` |

### Content Overlap

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `-mt-8` | `-mt-8` (same) |
| `max-w-7xl mx-auto` | `container mx-auto max-w-7xl` |
| `bg-white rounded-lg shadow` | `bg-base-100 rounded-box shadow-lg` |

### Footer

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `bg-white border-t` | `bg-base-100 border-t border-base-300` |
| `bg-gray-800` | `bg-neutral` |

## Implementation Strategy

### Phase 1: Core Components
1. Create `StackedLayout` shell with variant support
2. Create `StackedLayoutHeader` with context-aware styling
3. Create `StackedLayoutContent` with overlap support

### Phase 2: Navigation Components
4. Create `StackedLayoutNav` and `StackedLayoutNavItem`
5. Add active state styling for each variant

### Phase 3: Enhanced Features
6. Create `StackedLayoutSearch` component
7. Create `StackedLayoutFooter` component
8. Add secondary nav support

### Phase 4: Showcase Integration
9. Update `stacked_layout_page.rs` with all 9 examples
10. Add navigation route if needed
11. Update manifest

## Usage Examples

### Example 1: Simple Light Header with Tabs
```rust
view! {
    <StackedLayout variant=StackedLayoutVariant::Light>
        <StackedLayoutHeader>
            <NavbarStart>
                <span class="text-xl font-bold">"MyApp"</span>
            </NavbarStart>
            <NavbarCenter>
                <StackedLayoutNav>
                    <StackedLayoutNavItem href="/" active=true>"Dashboard"</StackedLayoutNavItem>
                    <StackedLayoutNavItem href="/team">"Team"</StackedLayoutNavItem>
                    <StackedLayoutNavItem href="/projects">"Projects"</StackedLayoutNavItem>
                </StackedLayoutNav>
            </NavbarCenter>
        </StackedLayoutHeader>
        <StackedLayoutContent>
            <p>"Main content here"</p>
        </StackedLayoutContent>
    </StackedLayout>
}
```

### Example 2: Dark Header with Overlap
```rust
view! {
    <StackedLayout variant=StackedLayoutVariant::Dark overlap=true>
        <StackedLayoutHeader>
            <NavbarStart>
                <span class="text-xl font-bold">"DarkApp"</span>
            </NavbarStart>
            <NavbarCenter>
                <StackedLayoutNav>
                    <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
                    <StackedLayoutNavItem href="/about">"About"</StackedLayoutNavItem>
                </StackedLayoutNav>
            </NavbarCenter>
        </StackedLayoutHeader>
        <StackedLayoutContent overlap=true>
            <div class="bg-base-100 rounded-box shadow-lg p-6 -mt-8">
                <h1>"Overlapping Content"</h1>
            </div>
        </StackedLayoutContent>
    </StackedLayout>
}
```

### Example 3: Branded Header with Search
```rust
view! {
    <StackedLayout variant=StackedLayoutVariant::Primary>
        <StackedLayoutHeader>
            <NavbarStart>
                <span class="text-xl font-bold">"Brand"</span>
            </NavbarStart>
            <NavbarCenter>
                <StackedLayoutSearch placeholder="Search..." />
            </NavbarCenter>
            <NavbarEnd>
                <button class="btn btn-ghost btn-circle">
                    <span>"🔔"</span>
                </button>
            </NavbarEnd>
        </StackedLayoutHeader>
        <StackedLayoutContent>
            <p>"Content here"</p>
        </StackedLayoutContent>
    </StackedLayout>
}
```

## Responsive Behavior

### Mobile (< md)
- Navigation collapses to hamburger menu
- Search moves to dropdown or hides
- Logo only (no text)
- Compact padding

### Tablet (md - lg)
- Horizontal nav visible
- Reduced spacing
- Full logo

### Desktop (lg+)
- Full horizontal navigation
- Search visible in header
- Maximum spacing

## Accessibility Requirements

1. **Semantic HTML**: Use `<header>`, `<nav>`, `<main>`, `<footer>` tags
2. **ARIA Labels**: Navigation should have `aria-label="Main navigation"`
3. **Focus Management**: Visible focus states for all interactive elements
4. **Color Contrast**: Ensure text meets WCAG 4.5:1 ratio on all variants
5. **Keyboard Navigation**: Tab order follows visual layout

## Files to Modify

### New Files
- None (add to existing `layout.rs` or create `stacked_layout.rs`)

### Modified Files
1. `crates/leptos-daisyui/src/components/layout.rs` - Add new components
2. `crates/leptos-daisyui/src/components/mod.rs` - Export new components
3. `crates/leptos-daisyui/src/prelude.rs` - Re-export for convenience
4. `crates/showcase/src/pages/stacked_layout_page.rs` - Add all examples
5. `components_manifest/layout.yml` - Document new components

## CSS Classes Reference

### Variant Classes

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
    
    fn nav_item_classes(&self, active: bool) -> &'static str {
        match (self, active) {
            (Self::Light, true) => "border-b-2 border-primary font-medium",
            (Self::Light, false) => "text-base-content/70 hover:text-base-content",
            (Self::LightBordered, true) => "border-b-2 border-primary font-medium",
            (Self::LightBordered, false) => "text-base-content/70 hover:text-base-content",
            (Self::Dark, true) => "bg-neutral-content/20 font-medium",
            (Self::Dark, false) => "text-neutral-content/70 hover:bg-neutral-content/10",
            (Self::Primary, true) => "bg-primary-content/20 font-medium",
            (Self::Primary, false) => "text-primary-content/80 hover:bg-primary-content/10",
        }
    }
}
```

## Migration Path

The existing `StackedShell` component should remain for backward compatibility. The new `StackedLayout` components provide a more structured, feature-rich alternative.

```rust
// Old (still works)
<StackedShell>
    <div class="navbar">...</div>
    <div class="flex-1">...</div>
</StackedShell>

// New (recommended)
<StackedLayout variant=StackedLayoutVariant::Dark>
    <StackedLayoutHeader>...</StackedLayoutHeader>
    <StackedLayoutContent>...</StackedLayoutContent>
</StackedLayout>
```
