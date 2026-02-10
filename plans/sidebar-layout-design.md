# Sidebar Layout Component Design

## Overview

This document describes the design for a new comprehensive `SidebarLayout` component system for leptos-daisyui. The design is inspired by Tailwind Plus sidebar application shell patterns and leverages daisyUI's native `drawer` component for mobile responsiveness.

## Design Goals

1. **Use daisyUI primitives** - Leverage `drawer`, `menu`, `navbar` components
2. **Responsive by default** - Mobile-first with desktop enhancements
3. **SSR-safe** - All components work in SSR/CSR/Hydrate modes
4. **Escape hatches** - Support `class` and `attrs` props on all components
5. **Semantic HTML** - Use appropriate tags (`<nav>`, `<aside>`, `<main>`, `<header>`)

---

## Component Hierarchy

```
SidebarLayout (shell component - uses drawer)
├── SidebarLayoutContent (drawer-content wrapper)
│   ├── SidebarLayoutTopBar (optional top navbar)
│   │   ├── SidebarLayoutTopBarStart (mobile menu button, breadcrumbs)
│   │   ├── SidebarLayoutTopBarCenter (search)
│   │   └── SidebarLayoutTopBarEnd (notifications, user dropdown)
│   └── SidebarLayoutMain (main content area)
│
└── SidebarLayoutSide (drawer-side wrapper)
    ├── SidebarLayoutOverlay (drawer-overlay backdrop)
    └── SidebarLayoutPanel (the actual sidebar panel)
        ├── SidebarLayoutHeader (logo area)
        ├── SidebarLayoutNav (navigation - uses menu)
        │   └── SidebarLayoutNavSection (grouped nav items with title)
        │       └── SidebarLayoutNavItem (individual nav link)
        └── SidebarLayoutFooter (user profile / settings at bottom)
```

---

## Component Specifications

### 1. SidebarLayout

The root shell component that wraps the entire layout. Uses daisyUI's `drawer` component internally.

**Props:**

```rust
#[component]
pub fn SidebarLayout(
    children: Children,                    // Should contain SidebarLayoutContent and SidebarLayoutSide
    #[prop(optional)] variant: SidebarLayoutVariant,  // Light, Dark, Primary
    #[prop(optional)] width: SidebarLayoutWidth,      // Narrow, Default, Wide
    #[prop(optional)] open_on_large: bool,            // Always show sidebar on lg+ screens
    #[prop(optional, into)] drawer_id: MaybeProp<String>,  // Custom drawer ID (default: auto-generated)
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Variants:**

```rust
pub enum SidebarLayoutVariant {
    #[default]
    Light,     // bg-base-100 or bg-base-200
    Dark,      // bg-neutral text-neutral-content
    Primary,   // bg-primary text-primary-content
}

pub enum SidebarLayoutWidth {
    Narrow,    // w-64 (16rem / 256px)
    #[default]
    Default,   // w-72 (18rem / 288px)
    Wide,      // w-80 (20rem / 320px)
}
```

**Classes:**

| Variant | Base Classes |
|---------|-------------|
| Light | `drawer` |
| Dark | `drawer` (dark applied to panel) |
| Primary | `drawer` (primary applied to panel) |

**Responsive Behavior:**

- Mobile: Drawer hidden, toggle via hamburger menu
- Desktop (lg:): If `open_on_large` is true, adds `lg:drawer-open`

**Example Output:**

```html
<div class="drawer lg:drawer-open">
  <input id="sidebar-drawer" type="checkbox" class="drawer-toggle" />
  <!-- children -->
</div>
```

---

### 2. SidebarLayoutContent

Wrapper for the main content area and optional top bar. Maps to `drawer-content`.

**Props:**

```rust
#[component]
pub fn SidebarLayoutContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
drawer-content flex flex-col min-h-screen
```

---

### 3. SidebarLayoutTopBar

Optional top navigation bar. Can be shown on mobile only, desktop only, or both.

**Props:**

```rust
#[component]
pub fn SidebarLayoutTopBar(
    children: Children,
    #[prop(optional)] sticky: bool,              // Sticky positioning
    #[prop(optional)] show_on_mobile: bool,      // Default: true
    #[prop(optional)] show_on_desktop: bool,     // Default: true
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
navbar bg-base-100 border-b border-base-300 w-full
```

- With `sticky`: `sticky top-0 z-30`

---

### 4. SidebarLayoutTopBarStart

Left section of the top bar. Contains hamburger menu on mobile.

**Props:**

```rust
#[component]
pub fn SidebarLayoutTopBarStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
navbar-start
```

---

### 5. SidebarLayoutTopBarCenter

Center section of the top bar. Typically used for search.

**Props:**

```rust
#[component]
pub fn SidebarLayoutTopBarCenter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
navbar-center
```

---

### 6. SidebarLayoutTopBarEnd

Right section of the top bar. Contains notifications, user dropdown.

**Props:**

```rust
#[component]
pub fn SidebarLayoutTopBarEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
navbar-end
```

---

### 7. SidebarLayoutMain

The main content area. Applies proper padding to account for fixed sidebar.

**Props:**

```rust
#[component]
pub fn SidebarLayoutMain(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
flex-1 overflow-y-auto
```

- Desktop padding accounts for sidebar width via `lg:pl-72` (or appropriate width)

---

### 8. SidebarLayoutSide

The drawer-side wrapper. Contains the overlay and panel.

**Props:**

```rust
#[component]
pub fn SidebarLayoutSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
drawer-side z-40
```

---

### 9. SidebarLayoutOverlay

The backdrop overlay for mobile. Clicking it closes the drawer.

**Props:**

```rust
#[component]
pub fn SidebarLayoutOverlay(
    #[prop(optional, into)] drawer_id: MaybeProp<String>,  // Links to drawer toggle
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
drawer-overlay
```

**Example Output:**

```html
<label for="sidebar-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
```

---

### 10. SidebarLayoutPanel

The actual sidebar panel containing navigation.

**Props:**

```rust
#[component]
pub fn SidebarLayoutPanel(
    children: Children,
    #[prop(optional)] variant: SidebarLayoutVariant,  // Inherited from parent or override
    #[prop(optional)] width: SidebarLayoutWidth,      // Inherited from parent or override
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes by Variant:**

| Variant | Classes |
|---------|---------|
| Light | `bg-base-200 min-h-full` |
| Dark | `bg-neutral text-neutral-content min-h-full` |
| Primary | `bg-primary text-primary-content min-h-full` |

**Width Classes:**

| Width | Class |
|-------|-------|
| Narrow | `w-64` |
| Default | `w-72` |
| Wide | `w-80` |

---

### 11. SidebarLayoutHeader

Logo/branding area at the top of the sidebar.

**Props:**

```rust
#[component]
pub fn SidebarLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
p-4 flex items-center gap-2 border-b border-base-300/10
```

---

### 12. SidebarLayoutNav

Navigation container. Uses daisyUI `menu` internally.

**Props:**

```rust
#[component]
pub fn SidebarLayoutNav(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
menu p-4 flex-1 overflow-y-auto
```

---

### 13. SidebarLayoutNavSection

A grouped section of navigation items with an optional title.

**Props:**

```rust
#[component]
pub fn SidebarLayoutNavSection(
    children: Children,
    #[prop(optional, into)] title: MaybeProp<String>,  // Section title
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
mb-4
```

**Title Classes:**

```
menu-title text-xs uppercase opacity-60 px-2 mb-2
```

---

### 14. SidebarLayoutNavItem

Individual navigation item.

**Props:**

```rust
#[component]
pub fn SidebarLayoutNavItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,   // Link URL
    #[prop(optional)] active: bool,                     // Is this the current page?
    #[prop(optional)] disabled: bool,                   // Is this item disabled?
    #[prop(optional, into)] icon: MaybeProp<String>,    // Optional icon SVG or class
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

| State | Classes |
|-------|---------|
| Default | `<a>` inside `<li>` |
| Active | `menu-active` on `<a>` |
| Disabled | `menu-disabled` on `<li>` |

**Example Output:**

```html
<li>
  <a href="/dashboard" class="menu-active">
    <svg>...</svg>
    Dashboard
  </a>
</li>
```

---

### 15. SidebarLayoutFooter

Footer area at the bottom of the sidebar. Typically contains user profile or settings.

**Props:**

```rust
#[component]
pub fn SidebarLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
p-4 border-t border-base-300/10 mt-auto
```

---

## Mobile Menu Button Component

A convenience component for the hamburger menu button.

**Props:**

```rust
#[component]
pub fn SidebarLayoutMenuButton(
    #[prop(optional, into)] drawer_id: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

**Classes:**

```
btn btn-square btn-ghost lg:hidden
```

**Example Output:**

```html
<label for="sidebar-drawer" aria-label="open sidebar" class="btn btn-square btn-ghost lg:hidden">
  <svg>...</svg>
</label>
```

---

## Complete Usage Example

### Basic Sidebar Layout

```rust
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <SidebarLayout 
            variant=SidebarLayoutVariant::Light
            width=SidebarLayoutWidth::Default
            open_on_large=true
        >
            // Main content area
            <SidebarLayoutContent>
                // Top bar (mobile + desktop)
                <SidebarLayoutTopBar>
                    <SidebarLayoutTopBarStart>
                        <SidebarLayoutMenuButton />
                        <span class="text-lg font-bold px-4">"MyApp"</span>
                    </SidebarLayoutTopBarStart>
                    <SidebarLayoutTopBarCenter>
                        <input type="text" placeholder="Search..." class="input input-bordered w-full max-w-xs" />
                    </SidebarLayoutTopBarCenter>
                    <SidebarLayoutTopBarEnd>
                        <button class="btn btn-ghost btn-circle">
                            <svg>/* bell icon */</svg>
                        </button>
                        <div class="dropdown dropdown-end">
                            <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                                <div class="w-10 rounded-full">
                                    <img src="avatar.jpg" />
                                </div>
                            </label>
                        </div>
                    </SidebarLayoutTopBarEnd>
                </SidebarLayoutTopBar>
                
                // Main content
                <SidebarLayoutMain>
                    <div class="p-6">
                        <h1 class="text-2xl font-bold">"Dashboard"</h1>
                        // Page content here
                    </div>
                </SidebarLayoutMain>
            </SidebarLayoutContent>
            
            // Sidebar
            <SidebarLayoutSide>
                <SidebarLayoutOverlay />
                <SidebarLayoutPanel variant=SidebarLayoutVariant::Light>
                    <SidebarLayoutHeader>
                        <svg>/* logo */</svg>
                        <span class="font-bold text-lg">"MyApp"</span>
                    </SidebarLayoutHeader>
                    
                    <SidebarLayoutNav>
                        <SidebarLayoutNavSection title="Navigation">
                            <SidebarLayoutNavItem href="/dashboard" active=true>
                                "Dashboard"
                            </SidebarLayoutNavItem>
                            <SidebarLayoutNavItem href="/projects">
                                "Projects"
                            </SidebarLayoutNavItem>
                            <SidebarLayoutNavItem href="/calendar">
                                "Calendar"
                            </SidebarLayoutNavItem>
                        </SidebarLayoutNavSection>
                        
                        <SidebarLayoutNavSection title="Settings">
                            <SidebarLayoutNavItem href="/settings">
                                "Settings"
                            </SidebarLayoutNavItem>
                            <SidebarLayoutNavItem href="/help">
                                "Help"
                            </SidebarLayoutNavItem>
                        </SidebarLayoutNavSection>
                    </SidebarLayoutNav>
                    
                    <SidebarLayoutFooter>
                        <div class="flex items-center gap-3">
                            <div class="avatar">
                                <div class="w-10 rounded-full">
                                    <img src="avatar.jpg" />
                                </div>
                            </div>
                            <div class="flex-1">
                                <div class="font-semibold">"John Doe"</div>
                                <div class="text-sm opacity-60">"john@example.com"</div>
                            </div>
                        </div>
                    </SidebarLayoutFooter>
                </SidebarLayoutPanel>
            </SidebarLayoutSide>
        </SidebarLayout>
    }
}
```

### Dark Sidebar Variant

```rust
<SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
    // ... same structure, sidebar will have dark theme
</SidebarLayout>
```

### Primary Sidebar with Icons

```rust
<SidebarLayoutNav>
    <SidebarLayoutNavSection>
        <SidebarLayoutNavItem href="/dashboard" active=true>
            <svg>/* home icon */</svg>
            "Dashboard"
        </SidebarLayoutNavItem>
        <SidebarLayoutNavItem href="/analytics">
            <svg>/* chart icon */</svg>
            "Analytics"
        </SidebarLayoutNavItem>
    </SidebarLayoutNavSection>
</SidebarLayoutNav>
```

---

## Mapping to Tailwind Plus Patterns

The design maps to the 8 Tailwind Plus sidebar variants as follows:

| Tailwind Plus Pattern | Implementation |
|----------------------|----------------|
| **Mobile Sidebar (Dialog)** | daisyUI `drawer` with `drawer-toggle` checkbox |
| **Desktop Sidebar (Fixed)** | `lg:drawer-open` class on drawer wrapper |
| **Light Sidebar** | `SidebarLayoutVariant::Light` with `bg-base-200` |
| **Dark Sidebar** | `SidebarLayoutVariant::Dark` with `bg-neutral` |
| **Indigo/Primary Sidebar** | `SidebarLayoutVariant::Primary` with `bg-primary` |
| **Top Bar (Mobile)** | `SidebarLayoutTopBar` with `show_on_desktop=false` |
| **Top Bar (Desktop)** | `SidebarLayoutTopBar` with full navbar content |
| **Main Content Padding** | `SidebarLayoutMain` with `lg:pl-72` |

### Responsive Behavior

```
Mobile (< lg):
┌─────────────────────┐
│ TopBar (hamburger)  │
├─────────────────────┤
│                     │
│    Main Content     │
│                     │
└─────────────────────┘
Sidebar: Hidden (drawer closed)

Desktop (>= lg):
┌────────┬────────────┐
│        │ TopBar     │
│ Side   ├────────────┤
│ bar    │            │
│        │ Main       │
│        │ Content    │
└────────┴────────────┘
Sidebar: Always visible (lg:drawer-open)
```

---

## Implementation Notes

### State Management

The drawer state is managed by the browser via the checkbox input. No JavaScript state is required for basic open/close functionality.

For programmatic control (e.g., close drawer after navigation), a Tier B helper can be created:

```rust
// Tier B - feature-gated for hydrate/csr
#[cfg(feature = "hydrate")]
pub fn use_sidebar_drawer(drawer_id: &str) -> SidebarDrawerControl {
    // Provides methods to open/close/toggle the drawer
}
```

### Accessibility

- All interactive elements have proper `aria-label` attributes
- The drawer overlay has `aria-label="close sidebar"`
- The menu button has `aria-label="open sidebar"`
- Disabled items use `aria-disabled="true"` for links
- Focus management is handled by the browser's native dialog/focus behavior

### SSR Considerations

- The drawer checkbox is unchecked by default (sidebar closed)
- On desktop with `lg:drawer-open`, CSS handles showing the sidebar
- No hydration errors since the checkbox state is not managed by Leptos

---

## File Structure

```
crates/leptos-daisyui/src/components/
├── sidebar_layout.rs          // All sidebar layout components
└── mod.rs                     // Add exports

components_manifest/
└── sidebar_layout.yml         // Component manifest
```

---

## Export Summary

```rust
// In mod.rs
pub use sidebar_layout::{
    SidebarLayout, SidebarLayoutVariant, SidebarLayoutWidth,
    SidebarLayoutContent,
    SidebarLayoutTopBar, SidebarLayoutTopBarStart, SidebarLayoutTopBarCenter, SidebarLayoutTopBarEnd,
    SidebarLayoutMain,
    SidebarLayoutSide, SidebarLayoutOverlay, SidebarLayoutPanel,
    SidebarLayoutHeader, SidebarLayoutNav, SidebarLayoutNavSection, SidebarLayoutNavItem,
    SidebarLayoutFooter,
    SidebarLayoutMenuButton,
};
```

---

## Questions for Clarification

1. **Collapsible sidebar on desktop?** Should we support a collapsed/icon-only state on desktop (like the Tailwind Plus examples with `is-drawer-close:w-14`)?

2. **Multiple sidebars?** Should we support right-side sidebars via a `position` prop?

3. **Nested navigation?** Should `SidebarLayoutNavItem` support nested children (submenus)?

4. **Context/Provider?** Should we provide a context provider for sharing sidebar state (open/closed) across components?

---

## Next Steps

1. Review and approve this design document
2. Create `components_manifest/sidebar_layout.yml`
3. Implement components in `sidebar_layout.rs`
4. Add exports to `mod.rs` and `prelude.rs`
5. Create demo page in showcase app
6. Verify with `cargo leptos build`
