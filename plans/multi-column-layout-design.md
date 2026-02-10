# Multi-Column Layout Design Document

## Overview

This document describes the design for adapting Tailwind Plus Elements multi-column layout patterns to daisyUI and integrating them into the leptos-daisyui showcase app.

The user provided HTML from Tailwind Plus Elements showing 4 distinct layout patterns:
1. **Sidebar with secondary column** (left sidebar + main + right sidebar)
2. **Three-column layout with header** (header + left + main + right)
3. **Narrow sidebar with secondary column** (icon-only sidebar + main + right)
4. **Narrow sidebar with header and secondary column** (icon sidebar + header + main + right)

## Analysis of Existing Components

### Current Layout Components (from `layout.rs`)

| Component | Purpose | Status |
|-----------|---------|--------|
| `MultiColumnShell` | Flex container for multi-column layouts | ✅ Exists |
| `MainColumn` | Primary content area with `flex-1` | ✅ Exists |
| `Panel` | Side panel with configurable width | ✅ Exists |
| `PanelWidth` | Enum: ExtraSmall, Small, Medium, Large, ExtraLarge | ✅ Exists |
| `SidebarShell` | Simple flex-based sidebar layout | ✅ Exists |
| `Sidebar` | Sidebar panel with width variants | ✅ Exists |
| `SidebarContent` | Main content area for SidebarShell | ✅ Exists |
| `StackedShell` | Full-width page with navbar + content | ✅ Exists |

### Current Sidebar Components (from `sidebar_layout.rs`)

| Component | Purpose | Status |
|-----------|---------|--------|
| `SidebarLayout` | Drawer-based responsive sidebar | ✅ Exists |
| `SidebarLayoutContent` | Content wrapper for drawer | ✅ Exists |
| `SidebarLayoutSide` | Drawer side panel | ✅ Exists |
| `SidebarLayoutPanel` | The actual sidebar panel | ✅ Exists |
| `SidebarLayoutMain` | Main content with padding | ✅ Exists |
| `SidebarLayoutTopBar` | Top navigation bar | ✅ Exists |

## Gap Analysis: What's Missing

Based on the Tailwind Plus HTML patterns, the following capabilities are needed:

### 1. Right Sidebar/Panel Support
The existing `SidebarLayout` only supports left-side drawers. The Tailwind Plus patterns show:
- Right-side secondary column (`xl:pr-96`)
- Right fixed panel (`fixed inset-y-0 right-0`)

### 2. Combined Layout Patterns
The Tailwind Plus examples combine:
- Fixed sidebar (left) + Main + Right panel
- Narrow icon sidebar + Secondary column + Main
- Header + 3-column layout

### 3. Layout Shell Variants Needed

```
┌─────────────────────────────────────────────────────────────┐
│ Pattern 1: Sidebar with Secondary Column                    │
├────────┬──────────────────────────────┬─────────────────────┤
│        │                              │                     │
│ Sidebar│         Main Content         │   Right Panel       │
│(w-72)  │         (flex-1)             │   (w-96, xl only)   │
│        │                              │                     │
└────────┴──────────────────────────────┴─────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Pattern 2: Three-Column with Header                         │
├─────────────────────────────────────────────────────────────┤
│ Header                                                      │
├────────┬──────────────────────────────┬─────────────────────┤
│ Left   │                              │                     │
│ Panel  │         Main Content         │   Right Panel       │
│(w-64)  │                              │   (w-96)            │
└────────┴──────────────────────────────┴─────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Pattern 3: Narrow Sidebar + Secondary Column                │
├────┬────────┬──────────────────────────────┬────────────────┤
│    │        │                              │                │
│Icon│Secondary│        Main Content         │  Right Panel   │
│Bar │Column  │                              │                │
│(w-20)│(w-96)│                              │                │
└────┴────────┴──────────────────────────────┴────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Pattern 4: Narrow Sidebar + Header + Secondary              │
├────┬────────────────────────────────────────────────────────┤
│    │ Header                                                 │
├────┼────────┬──────────────────────────────┬────────────────┤
│Icon│Secondary│        Main Content         │  Right Panel   │
│Bar │Column  │                              │                │
└────┴────────┴──────────────────────────────┴────────────────┘
```

## Component Architecture

### Option A: Extend Existing Components (Recommended)

Extend the existing `layout.rs` components to support the new patterns:

#### 1. Add `SecondaryColumn` Component

```rust
/// A secondary column that appears between a sidebar and main content.
/// Used in narrow sidebar layouts where there's an additional navigation
/// or filter column.
#[component]
pub fn SecondaryColumn(
    children: Children,
    #[prop(optional)] width: SecondaryColumnWidth,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

**Width Variants:**
- `Narrow` - w-64 (16rem)
- `Default` - w-80 (20rem) 
- `Wide` - w-96 (24rem)

#### 2. Add `RightPanel` Component

```rust
/// A fixed right-side panel that appears on xl+ screens.
/// Used for secondary content, details, or activity feeds.
#[component]
pub fn RightPanel(
    children: Children,
    #[prop(optional)] width: RightPanelWidth,
    #[prop(optional)] breakpoint: Breakpoint, // xl, lg
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

#### 3. Add `ThreeColumnShell` Component

```rust
/// A layout shell supporting left sidebar, main content, and right panel.
/// Combines SidebarShell with MultiColumnShell capabilities.
#[component]
pub fn ThreeColumnShell(
    children: Children, // Should contain Sidebar, MainColumn, RightPanel
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

#### 4. Extend `SidebarShell` with Header Support

```rust
/// Add header slot support to SidebarShell
#[component]
pub fn SidebarShell(
    children: Children,
    #[prop(optional)] has_header: bool, // Adjust layout for header
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

### Option B: Create New Composite Components

Create higher-level components that compose existing ones:

```rust
/// Pre-configured layout for Pattern 1: Sidebar + Secondary Column
#[component]
pub fn SidebarWithSecondaryLayout(
    sidebar: Children,
    secondary: Children,
    main: Children,
    #[prop(optional)] right_panel: Option<Children>,
) -> impl IntoView

/// Pre-configured layout for Pattern 3: Narrow Sidebar + Secondary
#[component]
pub fn NarrowSidebarLayout(
    icon_sidebar: Children,
    secondary: Children,
    main: Children,
    #[prop(optional)] right_panel: Option<Children>,
) -> impl IntoView
```

## Tailwind to daisyUI Class Mapping

### Background Colors

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `bg-white` | `bg-base-100` |
| `bg-gray-900` | `bg-neutral` or `bg-base-300` |
| `bg-gray-50` | `bg-base-200` |
| `dark:bg-gray-900` | `dark:bg-neutral` |

### Border Colors

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `border-gray-200` | `border-base-300` |
| `border-white/10` | `border-base-content/10` |
| `dark:border-white/10` | `dark:border-base-content/10` |

### Text Colors

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `text-gray-900` | `text-base-content` |
| `text-gray-700` | `text-base-content/70` |
| `text-gray-400` | `text-base-content/50` |
| `text-white` | `text-neutral-content` |

### Interactive States

| Tailwind Plus | daisyUI Equivalent |
|---------------|-------------------|
| `hover:bg-gray-50` | `hover:bg-base-200` |
| `hover:bg-gray-800` | `hover:bg-neutral-focus` |
| `bg-gray-50` (active) | `bg-base-200` |
| `bg-gray-800` (active) | `bg-neutral` |

## Props Design

### SecondaryColumn

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SecondaryColumnWidth {
    /// Narrow secondary column (w-64 / 16rem)
    Narrow,
    /// Default width (w-80 / 20rem)
    #[default]
    Default,
    /// Wide secondary column (w-96 / 24rem)
    Wide,
}

#[component]
pub fn SecondaryColumn(
    children: Children,
    /// Width of the secondary column
    #[prop(optional)] width: SecondaryColumnWidth,
    /// Add border on the right side
    #[prop(optional)] bordered: bool,
    /// Background color variant
    #[prop(optional)] variant: ColumnVariant,
    /// Additional classes
    #[prop(optional, into)] class: MaybeProp<String>,
    /// HTML attributes
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

### RightPanel

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelWidth {
    /// Narrow panel (w-64)
    Narrow,
    /// Default panel (w-80)
    #[default]
    Default,
    /// Wide panel (w-96)
    Wide,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Breakpoint {
    /// Show on lg screens and up
    Large,
    /// Show on xl screens and up (default)
    #[default]
    ExtraLarge,
}

#[component]
pub fn RightPanel(
    children: Children,
    /// Width of the right panel
    #[prop(optional)] width: RightPanelWidth,
    /// Breakpoint at which panel becomes visible
    #[prop(optional)] breakpoint: Breakpoint,
    /// Position: fixed or static
    #[prop(optional)] fixed: bool,
    /// Additional classes
    #[prop(optional, into)] class: MaybeProp<String>,
    /// HTML attributes
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView
```

### ColumnVariant

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnVariant {
    /// Default background (transparent/inherit)
    #[default]
    Default,
    /// Light background (bg-base-100)
    Light,
    /// Muted background (bg-base-200)
    Muted,
    /// Dark background (bg-neutral)
    Dark,
}
```

## Showcase Page Structure

The `MultiColumnPage` should demonstrate:

1. **Basic Two-Column** (already exists)
   - Main + Right Panel

2. **Three-Column Layout** (already exists)
   - Left Panel + Main + Right Panel

3. **Sidebar with Secondary Column** (NEW)
   - Fixed sidebar + Secondary column + Main

4. **Narrow Sidebar Layout** (NEW)
   - Icon-only sidebar + Main + Right Panel

5. **Full Dashboard Layout** (NEW)
   - Header + Narrow Sidebar + Secondary + Main

## Implementation Plan

### Phase 1: Extend layout.rs

1. Add `SecondaryColumn` component
2. Add `RightPanel` component  
3. Add `ColumnVariant` enum
4. Add `SecondaryColumnWidth` enum
5. Update exports in `mod.rs`
6. Update exports in `prelude.rs`

### Phase 2: Update Showcase Page

1. Add new layout examples to `multi_column_page.rs`
2. Create visual representations of each pattern
3. Add code examples in mockup-code blocks

### Phase 3: Update Manifest

1. Update `components_manifest/layout.yml` with new components

### Phase 4: Verify Build

1. Run `cargo leptos build` in demo app
2. Run `cargo fmt --all`
3. Run `cargo clippy --all-targets --all-features -D warnings`

## Component Hierarchy

```
ThreeColumnShell (new)
├── Sidebar (existing) / IconSidebar (new)
├── SecondaryColumn (new) [optional]
├── MainColumn (existing)
└── RightPanel (new) [optional]

SidebarWithHeaderShell (new)
├── Header (existing Navbar or new PageHeader)
└── ThreeColumnShell
    ├── Sidebar
    ├── MainColumn
    └── RightPanel
```

## Usage Examples

### Pattern 1: Sidebar with Secondary Column

```rust
view! {
    <div class="flex min-h-screen">
        <Sidebar width=SidebarWidth::Default>
            // Navigation
        </Sidebar>
        <div class="flex flex-1">
            <SecondaryColumn width=SecondaryColumnWidth::Wide>
                // Channel list, filters, etc.
            </SecondaryColumn>
            <MainColumn>
                // Main content
            </MainColumn>
        </div>
    </div>
}
```

### Pattern 3: Narrow Sidebar with Secondary

```rust
view! {
    <div class="flex min-h-screen">
        <Sidebar width=SidebarWidth::Narrow>
            // Icon-only navigation
        </Sidebar>
        <SecondaryColumn>
            // Detailed navigation
        </SecondaryColumn>
        <MainColumn>
            // Main content
        </MainColumn>
        <RightPanel breakpoint=Breakpoint::ExtraLarge>
            // Activity feed, details
        </RightPanel>
    </div>
}
```

### Pattern 4: With Header

```rust
view! {
    <div class="flex flex-col min-h-screen">
        <Navbar>
            // Top navigation
        </Navbar>
        <div class="flex flex-1">
            <Sidebar width=SidebarWidth::Narrow>
                // Icon sidebar
            </Sidebar>
            <SecondaryColumn>
                // Secondary nav
            </SecondaryColumn>
            <MainColumn>
                // Content
            </MainColumn>
        </div>
    </div>
}
```

## Questions for Clarification

1. **Should we support right-side drawers?** The existing `SidebarLayout` uses daisyUI's drawer which supports `drawer-end` for right-side drawers.

2. **Collapsible secondary column?** Should the secondary column be collapsible on smaller screens?

3. **Responsive behavior for 3-column layouts?** On smaller screens, should left/secondary columns collapse to drawers or stack?

4. **Sticky headers within columns?** Should columns support sticky headers/footers?

## Next Steps

1. Review and approve this design document
2. Implement new components in `layout.rs`
3. Update showcase page with new examples
4. Update component manifest
5. Verify builds pass
