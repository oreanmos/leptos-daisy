# Multi-Column Layout Implementation Tasks

This document provides a detailed, actionable task list for implementing the multi-column layout components. These tasks can be delegated to implementation agents.

---

## Task 1: Create Component Manifest

**File:** `components_manifest/multi_column_layout.yml`

**Description:** Create a manifest file documenting the new multi-column layout components.

**Content:**
```yaml
# Multi-Column Layout Components Manifest
# Extended layout system for complex dashboard patterns

components:
  - name: SecondaryColumn
    description: "A secondary column between sidebar and main content for navigation or filters"
    base_class: "flex flex-col overflow-y-auto"
    variants:
      - name: width
        values:
          - "w-64"  # Narrow
          - "w-80"  # Default
          - "w-96"  # Wide
      - name: variant
        values:
          - ""           # Default (transparent)
          - "bg-base-100" # Light
          - "bg-base-200" # Muted
          - "bg-neutral"  # Dark
    props:
      - width: SecondaryColumnWidth
      - variant: ColumnVariant
      - bordered: bool
    examples:
      - "Secondary column with channel list"
      - "Secondary column as filter panel"
      - "Dark variant secondary column"

  - name: RightPanel
    description: "Fixed right-side panel for secondary content, visible on xl+ screens"
    base_class: "fixed inset-y-0 right-0 overflow-y-auto"
    variants:
      - name: width
        values:
          - "w-64"  # Narrow
          - "w-80"  # Default
          - "w-96"  # Wide
      - name: breakpoint
        values:
          - "lg:block"  # Large screens
          - "xl:block"  # Extra large (default)
    props:
      - width: RightPanelWidth
      - breakpoint: Breakpoint
      - fixed: bool
    examples:
      - "Right panel with activity feed"
      - "Right panel with details"
      - "Static right panel (not fixed)"

  - name: ColumnVariant
    description: "Background color variants for columns"
    type: enum
    values:
      - Default
      - Light
      - Muted
      - Dark

  - name: SecondaryColumnWidth
    description: "Width options for secondary columns"
    type: enum
    values:
      - Narrow   # w-64
      - Default  # w-80
      - Wide     # w-96

  - name: RightPanelWidth
    description: "Width options for right panels"
    type: enum
    values:
      - Narrow   # w-64
      - Default  # w-80
      - Wide     # w-96

  - name: Breakpoint
    description: "Responsive breakpoints for showing panels"
    type: enum
    values:
      - Large      # lg
      - ExtraLarge # xl
```

**Verification:**
- [ ] File created at `components_manifest/multi_column_layout.yml`
- [ ] YAML is valid
- [ ] All components documented

---

## Task 2: Extend layout.rs with New Components

**File:** `crates/leptos-daisyui/src/components/layout.rs`

**Description:** Add new components and enums to the existing layout.rs file.

### 2.1 Add ColumnVariant Enum

Add after existing enums:

```rust
// --- ColumnVariant ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnVariant {
    /// Default background (transparent/inherit)
    #[default]
    Default,
    /// Light background (bg-base-100)
    Light,
    /// Muted background (bg-base-200)
    Muted,
    /// Dark background (bg-neutral with text-neutral-content)
    Dark,
}

impl ColumnVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Light => "bg-base-100",
            Self::Muted => "bg-base-200",
            Self::Dark => "bg-neutral text-neutral-content",
        }
    }
}
```

### 2.2 Add SecondaryColumnWidth Enum

```rust
// --- SecondaryColumn ---

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

impl SecondaryColumnWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-64",
            Self::Default => "w-80",
            Self::Wide => "w-96",
        }
    }
}
```

### 2.3 Add SecondaryColumn Component

Add after the `Panel` component:

```rust
/// A secondary column that appears between a sidebar and main content.
/// Used in narrow sidebar layouts where there's an additional navigation
/// or filter column between the icon sidebar and main content.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::prelude::*;
///
/// #[component]
/// pub fn MyLayout() -> impl IntoView {
///     view! {
///         <div class="flex min-h-screen">
///             <Sidebar width=SidebarWidth::Narrow>
///                 // Icon navigation
///             </Sidebar>
///             <SecondaryColumn variant=ColumnVariant::Muted bordered=true>
///                 <h3>"Channels"</h3>
///                 <ul class="menu">
///                     <li><a>"# general"</a></li>
///                     <li><a>"# random"</a></li>
///                 </ul>
///             </SecondaryColumn>
///             <MainColumn>
///                 // Main content
///             </MainColumn>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn SecondaryColumn(
    children: Children,
    /// Width of the secondary column
    #[prop(optional)] width: SecondaryColumnWidth,
    /// Background color variant
    #[prop(optional)] variant: ColumnVariant,
    /// Add border on the right side
    #[prop(optional)] bordered: bool,
    /// Additional classes
    #[prop(optional, into)] class: MaybeProp<String>,
    /// HTML attributes
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    m.push("flex-col");
    m.push("shrink-0");
    m.push("overflow-y-auto");
    m.push(width.cls());
    
    let variant_class = variant.cls();
    if !variant_class.is_empty() {
        m.push(variant_class);
    }
    
    if bordered {
        m.push("border-r");
        m.push("border-base-300");
    }
    
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex",
        &m,
        if uc.is_empty() { None } else { Some(uc.as_str()) },
    );
    
    view! { <aside class={cls} {..attrs}>{children()}</aside> }
}
```

### 2.4 Add RightPanelWidth Enum

```rust
// --- RightPanel ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelWidth {
    /// Narrow panel (w-64 / 16rem)
    Narrow,
    /// Default panel (w-80 / 20rem)
    #[default]
    Default,
    /// Wide panel (w-96 / 24rem)
    Wide,
}

impl RightPanelWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-64",
            Self::Default => "w-80",
            Self::Wide => "w-96",
        }
    }
}
```

### 2.5 Add Breakpoint Enum

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Breakpoint {
    /// Show on lg screens and up (1024px+)
    Large,
    /// Show on xl screens and up (1280px+)
    #[default]
    ExtraLarge,
}

impl Breakpoint {
    fn cls(&self) -> &'static str {
        match self {
            Self::Large => "lg:block",
            Self::ExtraLarge => "xl:block",
        }
    }
    
    fn hidden_cls(&self) -> &'static str {
        match self {
            Self::Large => "hidden lg:block",
            Self::ExtraLarge => "hidden xl:block",
        }
    }
}
```

### 2.6 Add RightPanel Component

```rust
/// A fixed right-side panel for secondary content, details, or activity feeds.
/// Hidden by default on smaller screens, visible at the specified breakpoint.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::prelude::*;
///
/// #[component]
/// pub fn MyLayout() -> impl IntoView {
///     view! {
///         <div class="flex min-h-screen">
///             <MainColumn class="xl:pr-80">
///                 // Main content
///             </MainColumn>
///             <RightPanel>
///                 <h3>"Activity"</h3>
///                 <ul>
///                     <li>"User signed up"</li>
///                     <li>"New comment"</li>
///                 </ul>
///             </RightPanel>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn RightPanel(
    children: Children,
    /// Width of the right panel
    #[prop(optional)] width: RightPanelWidth,
    /// Breakpoint at which panel becomes visible
    #[prop(optional)] breakpoint: Breakpoint,
    /// Use fixed positioning (default: true)
    #[prop(optional, default = true)]
    fixed: bool,
    /// Background color variant
    #[prop(optional)] variant: ColumnVariant,
    /// Add border on the left side
    #[prop(optional)] bordered: bool,
    /// Additional classes
    #[prop(optional, into)] class: MaybeProp<String>,
    /// HTML attributes
    #[prop(attrs)] attrs: Vec<Attribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    m.push(breakpoint.hidden_cls());
    m.push(width.cls());
    m.push("overflow-y-auto");
    
    if fixed {
        m.push("fixed");
        m.push("inset-y-0");
        m.push("right-0");
    } else {
        m.push("shrink-0");
    }
    
    let variant_class = variant.cls();
    if !variant_class.is_empty() {
        m.push(variant_class);
    }
    
    if bordered {
        m.push("border-l");
        m.push("border-base-300");
    }
    
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &m,
        if uc.is_empty() { None } else { Some(uc.as_str()) },
    );
    
    view! { <aside class={cls} {..attrs}>{children()}</aside> }
}
```

**Verification:**
- [ ] All enums added
- [ ] All components added
- [ ] Components follow existing patterns
- [ ] Doc comments included

---

## Task 3: Update mod.rs Exports

**File:** `crates/leptos-daisyui/src/components/mod.rs`

**Description:** Add exports for new components and enums.

**Changes:**

Add to the layout import section:

```rust
pub use layout::{
    Container, ContainerSize, Divider, DividerColor, DividerOrientation, Footer, Grid, GridCols,
    Hero, HeroContent, Join, MainColumn, MultiColumnShell, PageHeader, PageHeaderActions,
    PageHeaderTitle, Panel, PanelWidth, Sidebar, SidebarContent, SidebarShell, SidebarWidth, Stack,
    StackGap, StackedShell,
    // New exports:
    SecondaryColumn, SecondaryColumnWidth,
    RightPanel, RightPanelWidth,
    ColumnVariant, Breakpoint,
};
```

**Verification:**
- [ ] New exports added
- [ ] No duplicate exports
- [ ] Alphabetical order maintained

---

## Task 4: Update prelude.rs Exports

**File:** `crates/leptos-daisyui/src/prelude.rs`

**Description:** Add exports for new components and enums to prelude.

**Changes:**

Update the layout import:

```rust
pub use crate::components::layout::{
    Container, ContainerSize, Divider, DividerColor, DividerOrientation, Footer, Grid, GridCols,
    Hero, HeroContent, Join, MainColumn, MultiColumnShell, PageHeader, PageHeaderActions,
    PageHeaderTitle, Panel, PanelWidth, Sidebar, SidebarContent, SidebarShell, SidebarWidth, Stack,
    StackGap, StackedShell,
    // New exports:
    SecondaryColumn, SecondaryColumnWidth,
    RightPanel, RightPanelWidth,
    ColumnVariant, Breakpoint,
};
```

**Verification:**
- [ ] New exports added
- [ ] Consistent with mod.rs

---

## Task 5: Update MultiColumnPage Showcase

**File:** `crates/showcase/src/pages/multi_column_page.rs`

**Description:** Add new layout examples demonstrating the 4 Tailwind Plus patterns.

### 5.1 Add Pattern 3: Narrow Sidebar with Secondary Column

Add after existing sections:

```rust
// --- Pattern 3: Narrow Sidebar + Secondary Column ---
<section class="space-y-4">
    <h2 class="text-xl font-semibold">"Narrow Sidebar + Secondary Column"</h2>
    <p class="text-sm text-base-content/60">
        "Icon-only sidebar with a secondary navigation column. "
        "Common in chat applications (like Slack)."
    </p>
    <div class="border border-base-300 rounded-box overflow-hidden">
        <div class="flex h-[24rem]">
            // Narrow icon sidebar
            <aside class="w-16 flex flex-col bg-neutral text-neutral-content shrink-0">
                <div class="h-16 flex items-center justify-center border-b border-neutral-content/10">
                    <span class="text-xl">🌼</span>
                </div>
                <nav class="flex-1 py-4 space-y-2">
                    <a href="#" class="flex justify-center p-2 bg-white/10 rounded-lg mx-2">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                        </svg>
                    </a>
                    <a href="#" class="flex justify-center p-2 hover:bg-white/5 rounded-lg mx-2">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                        </svg>
                    </a>
                </nav>
            </aside>
            
            // Secondary column
            <SecondaryColumn variant=ColumnVariant::Muted bordered=true class="p-4">
                <h3 class="font-bold mb-4">"Channels"</h3>
                <ul class="menu menu-sm gap-1">
                    <li><a class="active">"# general"</a></li>
                    <li><a>"# random"</a></li>
                    <li><a>"# engineering"</a></li>
                    <li><a>"# design"</a></li>
                </ul>
            </SecondaryColumn>
            
            // Main content
            <MainColumn class="p-6">
                <h3 class="text-xl font-bold mb-4">"# general"</h3>
                <div class="space-y-4">
                    <div class="chat chat-start">
                        <div class="chat-bubble chat-bubble-primary">"Hey team!"</div>
                    </div>
                    <div class="chat chat-end">
                        <div class="chat-bubble">"Hello! 👋"</div>
                    </div>
                </div>
            </MainColumn>
        </div>
    </div>
</section>
```

### 5.2 Add Pattern 4: Full Dashboard with Header

```rust
// --- Pattern 4: Full Dashboard Layout ---
<section class="space-y-4">
    <h2 class="text-xl font-semibold">"Full Dashboard Layout"</h2>
    <p class="text-sm text-base-content/60">
        "Complete layout with header, narrow sidebar, secondary column, "
        "main content, and right panel."
    </p>
    <div class="border border-base-300 rounded-box overflow-hidden">
        <div class="flex flex-col h-[28rem]">
            // Header
            <header class="h-16 bg-base-100 border-b border-base-300 flex items-center px-4 gap-4 shrink-0">
                <button class="btn btn-ghost btn-square btn-sm lg:hidden">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                </button>
                <span class="font-bold text-lg">"Dashboard"</span>
                <div class="flex-1"></div>
                <button class="btn btn-ghost btn-circle btn-sm">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                    </svg>
                </button>
                <div class="avatar">
                    <div class="w-8 rounded-full">
                        <div class="bg-primary text-primary-content flex items-center justify-center h-full text-sm font-bold">TC</div>
                    </div>
                </div>
            </header>
            
            <div class="flex flex-1 overflow-hidden">
                // Narrow sidebar
                <aside class="w-16 flex flex-col bg-neutral text-neutral-content shrink-0">
                    <nav class="flex-1 py-4 space-y-2">
                        <a href="#" class="flex justify-center p-2 bg-white/10 rounded-lg mx-2">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                            </svg>
                        </a>
                    </nav>
                </aside>
                
                // Secondary column
                <SecondaryColumn variant=ColumnVariant::Muted bordered=true class="p-4 hidden lg:flex">
                    <h3 class="font-bold mb-4">"Projects"</h3>
                    <ul class="menu menu-sm gap-1">
                        <li><a class="active">"Website Redesign"</a></li>
                        <li><a>"Mobile App"</a></li>
                        <li><a>"API Development"</a></li>
                    </ul>
                </SecondaryColumn>
                
                // Main content
                <MainColumn class="p-6 xl:pr-80">
                    <h3 class="text-2xl font-bold mb-4">"Website Redesign"</h3>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Analytics"</div>
                        <div class="bg-base-200 rounded-box p-4 h-24 flex items-center justify-center">"Tasks"</div>
                    </div>
                </MainColumn>
                
                // Right panel (hidden on smaller screens)
                <RightPanel breakpoint=Breakpoint::ExtraLarge variant=ColumnVariant::Muted bordered=true class="p-4">
                    <h4 class="font-bold mb-3">"Activity"</h4>
                    <ul class="space-y-3 text-sm">
                        <li class="flex items-center gap-2">
                            <div class="badge badge-xs badge-success"></div>
                            <span>"Task completed"</span>
                        </li>
                        <li class="flex items-center gap-2">
                            <div class="badge badge-xs badge-info"></div>
                            <span>"New comment"</span>
                        </li>
                    </ul>
                </RightPanel>
            </div>
        </div>
    </div>
</section>
```

### 5.3 Update Usage Section

Update the usage code example at the bottom:

```rust
// --- Usage ---
<section class="space-y-4">
    <h2 class="text-xl font-semibold">"Usage"</h2>
    <div class="space-y-4">
        <h3 class="text-lg font-medium">"Basic Multi-Column"</h3>
        <div class="mockup-code text-sm">
            <pre data-prefix="1"><code>"<MultiColumnShell>"</code></pre>
            <pre data-prefix="2"><code>"  <Panel width=PanelWidth::Small class=\"border-r\">"</code></pre>
            <pre data-prefix="3"><code>"    // left panel"</code></pre>
            <pre data-prefix="4"><code>"  </Panel>"</code></pre>
            <pre data-prefix="5"><code>"  <MainColumn>"</code></pre>
            <pre data-prefix="6"><code>"    // main content"</code></pre>
            <pre data-prefix="7"><code>"  </MainColumn>"</code></pre>
            <pre data-prefix="8"><code>"</MultiColumnShell>"</code></pre>
        </div>
        
        <h3 class="text-lg font-medium">"With Secondary Column"</h3>
        <div class="mockup-code text-sm">
            <pre data-prefix="1"><code>"<div class=\"flex min-h-screen\">"</code></pre>
            <pre data-prefix="2"><code>"  <Sidebar width=SidebarWidth::Narrow>"</code></pre>
            <pre data-prefix="3"><code>"    // Icon navigation"</code></pre>
            <pre data-prefix="4"><code>"  </Sidebar>"</code></pre>
            <pre data-prefix="5"><code>"  <SecondaryColumn variant=ColumnVariant::Muted>"</code></pre>
            <pre data-prefix="6"><code>"    // Secondary nav"</code></pre>
            <pre data-prefix="7"><code>"  </SecondaryColumn>"</code></pre>
            <pre data-prefix="8"><code>"  <MainColumn>"</code></pre>
            <pre data-prefix="9"><code>"    // Main content"</code></pre>
            <pre data-prefix="10"><code>"  </MainColumn>"</code></pre>
            <pre data-prefix="11"><code>"</div>"</code></pre>
        </div>
        
        <h3 class="text-lg font-medium">"With Right Panel"</h3>
        <div class="mockup-code text-sm">
            <pre data-prefix="1"><code>"<div class=\"flex min-h-screen\">"</code></pre>
            <pre data-prefix="2"><code>"  <MainColumn class=\"xl:pr-80\">"</code></pre>
            <pre data-prefix="3"><code>"    // Main content with right padding"</code></pre>
            <pre data-prefix="4"><code>"  </MainColumn>"</code></pre>
            <pre data-prefix="5"><code>"  <RightPanel breakpoint=Breakpoint::ExtraLarge>"</code></pre>
            <pre data-prefix="6"><code>"    // Activity feed (xl+ only)"</code></pre>
            <pre data-prefix="7"><code>"  </RightPanel>"</code></pre>
            <pre data-prefix="8"><code>"</div>"</code></pre>
        </div>
    </div>
</section>
```

**Verification:**
- [ ] All 4 patterns demonstrated
- [ ] New components used in examples
- [ ] Code examples updated
- [ ] Page compiles without errors

---

## Task 6: Verify Builds

**Commands to run:**

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -D warnings

# Check different feature combinations
cargo check -p leptos-daisyui --features ssr
cargo check -p leptos-daisyui --features csr
cargo check -p leptos-daisyui --features hydrate

# Build showcase app
cd crates/showcase && cargo leptos build
```

**Verification:**
- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] SSR check passes
- [ ] CSR check passes
- [ ] Hydrate check passes
- [ ] `cargo leptos build` passes

---

## Summary of Changes

| File | Change Type | Description |
|------|-------------|-------------|
| `components_manifest/multi_column_layout.yml` | Create | New manifest for multi-column components |
| `crates/leptos-daisyui/src/components/layout.rs` | Extend | Add SecondaryColumn, RightPanel, enums |
| `crates/leptos-daisyui/src/components/mod.rs` | Update | Add new exports |
| `crates/leptos-daisyui/src/prelude.rs` | Update | Add new exports |
| `crates/showcase/src/pages/multi_column_page.rs` | Update | Add new layout examples |

## Components Summary

| Component | Purpose | Props |
|-----------|---------|-------|
| `SecondaryColumn` | Column between sidebar and main | width, variant, bordered |
| `RightPanel` | Fixed right-side panel | width, breakpoint, fixed, variant, bordered |
| `ColumnVariant` | Background color enum | Default, Light, Muted, Dark |
| `SecondaryColumnWidth` | Width enum | Narrow, Default, Wide |
| `RightPanelWidth` | Width enum | Narrow, Default, Wide |
| `Breakpoint` | Responsive breakpoint enum | Large, ExtraLarge |
