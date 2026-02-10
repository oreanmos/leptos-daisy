//! Layout components — daisyUI `container`, `divider`, `stack`, `join`, `grid`, `hero`, `footer`.
use crate::utils::class::build_class;
use leptos::prelude::*;

// --- Container ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContainerSize {
    #[default]
    Default,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl ContainerSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Small => "max-w-sm",
            Self::Medium => "max-w-md",
            Self::Large => "max-w-lg",
            Self::ExtraLarge => "max-w-xl",
        }
    }
}

#[component]
pub fn Container(
    children: Children,
    #[prop(optional)] size: ContainerSize,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let sc = size.cls();
    if !sc.is_empty() {
        m.push(sc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "container mx-auto",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- Divider ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl DividerOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Horizontal => "divider-horizontal",
            Self::Vertical => "divider-vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}
impl DividerColor {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "divider-primary",
            Self::Secondary => "divider-secondary",
            Self::Accent => "divider-accent",
            Self::Neutral => "divider-neutral",
            Self::Info => "divider-info",
            Self::Success => "divider-success",
            Self::Warning => "divider-warning",
            Self::Error => "divider-error",
        }
    }
}

#[component]
pub fn Divider(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional)] color: DividerColor,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    m.push(orientation.cls());
    let cc = color.cls();
    if !cc.is_empty() {
        m.push(cc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "divider",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! {
        <div class={cls}>
            {children.map(|c| c())}
        </div>
    }
}

// --- Stack ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackGap {
    #[default]
    Default,
    None,
    Small,
    Medium,
    Large,
}
impl StackGap {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::None => "gap-0",
            Self::Small => "gap-2",
            Self::Medium => "gap-4",
            Self::Large => "gap-8",
        }
    }
}

#[component]
pub fn Stack(
    children: Children,
    #[prop(optional)] gap: StackGap,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let gc = gap.cls();
    if !gc.is_empty() {
        m.push(gc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stack",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- Join ---

#[component]
pub fn Join(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if vertical {
        m.push("join-vertical");
    } else {
        m.push("join-horizontal");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "join",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- Grid ---

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridCols {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
impl GridCols {
    fn cls(&self) -> &'static str {
        match self {
            Self::One => "grid-cols-1",
            Self::Two => "grid-cols-2",
            Self::Three => "grid-cols-3",
            Self::Four => "grid-cols-4",
            Self::Five => "grid-cols-5",
            Self::Six => "grid-cols-6",
        }
    }
}

#[component]
pub fn Grid(
    children: Children,
    #[prop(optional)] cols: GridCols,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "grid",
        &[cols.cls(), "gap-4"],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- Hero ---

#[component]
pub fn Hero(children: Children, #[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "hero",
        &["min-h-screen"],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn HeroContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "hero-content",
        &["text-center"],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- Footer ---

#[component]
pub fn Footer(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["bg-neutral", "text-neutral-content", "p-10"];
    if center {
        m.push("footer-center");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "footer",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <footer class={cls}>{children()}</footer> }
}

// --- StackedShell ---
// A full-width page shell with a navbar on top, optional page header, and content area below.
// Inspired by Tailwind Plus "stacked" application shell layouts.

#[component]
pub fn StackedShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "min-h-screen flex flex-col",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- SidebarShell ---
// A layout with a fixed/responsive sidebar and main content area.
// The sidebar can be narrow (icon-only) or wide.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarWidth {
    /// Narrow / icon-only sidebar (w-16)
    Narrow,
    /// Default sidebar width (w-64)
    #[default]
    Default,
    /// Wide sidebar (w-80)
    Wide,
}
impl SidebarWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-16",
            Self::Default => "w-64",
            Self::Wide => "w-80",
        }
    }
}

#[component]
pub fn SidebarShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex min-h-screen",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// The sidebar panel within a `SidebarShell`.
#[component]
pub fn Sidebar(
    children: Children,
    #[prop(optional)] width: SidebarWidth,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex flex-col shrink-0 bg-base-200 overflow-y-auto",
        &[width.cls()],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <aside class={cls}>{children()}</aside> }
}

/// The main content area within a `SidebarShell`.
#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex-1 overflow-y-auto",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <main class={cls}>{children()}</main> }
}

// --- MultiColumnShell ---
// A layout with a main content column flanked by optional left/right panels.

#[component]
pub fn MultiColumnShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex flex-1",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// The primary content column within a `MultiColumnShell`.
#[component]
pub fn MainColumn(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex-1 min-w-0",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <main class={cls}>{children()}</main> }
}

// --- PageHeader ---
// A reusable page header section with title and optional actions.

#[component]
pub fn PageHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "border-b border-base-300 bg-base-100 px-4 py-4 sm:px-6 lg:px-8",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <header class={cls}>{children()}</header> }
}

/// Title text inside a `PageHeader`.
#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "text-2xl font-bold leading-7",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <h1 class={cls}>{children()}</h1> }
}

/// Action buttons area inside a `PageHeader`.
#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex items-center gap-2",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

// --- StackedLayout Components ---
// Full-width page shell with header, navigation, content area, and optional footer.
// Inspired by Tailwind Plus "stacked" application shell layouts.

/// Variant determining the color scheme of the stacked layout.
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

impl StackedLayoutVariant {
    /// Returns the CSS classes for the header background and text.
    fn header_classes(&self) -> &'static str {
        match self {
            Self::Light => "bg-base-100 text-base-content",
            Self::LightBordered => "bg-base-100 text-base-content border-b border-base-300",
            Self::Dark => "bg-neutral text-neutral-content",
            Self::Primary => "bg-primary text-primary-content",
        }
    }

    /// Returns the CSS classes for the nav item active state.
    fn nav_active_classes(&self) -> &'static str {
        match self {
            Self::Light | Self::LightBordered => "border-b-2 border-primary text-base-content",
            Self::Dark => "border-b-2 border-neutral-content text-neutral-content",
            Self::Primary => "border-b-2 border-primary-content text-primary-content",
        }
    }

    /// Returns the CSS classes for inactive nav items.
    fn nav_inactive_classes(&self) -> &'static str {
        match self {
            Self::Light | Self::LightBordered => "text-base-content/70 hover:text-base-content",
            Self::Dark => "text-neutral-content/70 hover:text-neutral-content",
            Self::Primary => "text-primary-content/70 hover:text-primary-content",
        }
    }

    /// Returns the CSS classes for search input styling.
    fn search_classes(&self) -> &'static str {
        match self {
            Self::Light | Self::LightBordered => "input-bordered bg-base-100 text-base-content",
            Self::Dark => {
                "bg-neutral-700 text-neutral-content border-neutral-600 placeholder-neutral-content/50"
            }
            Self::Primary => {
                "bg-primary-focus text-primary-content border-primary-content/30 placeholder-primary-content/50"
            }
        }
    }
}

/// Navigation style variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavStyle {
    /// Standard horizontal tabs
    #[default]
    Tabs,
    /// Rounded pill-style navigation
    Pills,
    /// Compact minimal navigation
    Compact,
}

impl NavStyle {
    /// Returns the CSS classes for the nav container.
    fn container_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "menu menu-horizontal gap-1",
            Self::Pills => "flex items-center gap-1",
            Self::Compact => "flex items-center gap-0.5",
        }
    }

    /// Returns the CSS classes for nav items.
    fn item_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "px-3 py-2 text-sm font-medium",
            Self::Pills => "px-3 py-2 text-sm font-medium rounded-lg",
            Self::Compact => "px-2 py-1.5 text-sm",
        }
    }

    /// Returns additional classes for active items.
    fn active_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "",
            Self::Pills => "bg-white/20",
            Self::Compact => "bg-white/10 rounded",
        }
    }
}

/// Context provided by StackedLayout to child components.
#[derive(Clone, Debug)]
struct StackedLayoutContext {
    variant: StackedLayoutVariant,
}

/// Root shell component for stacked layouts.
///
/// Provides a full-width page shell with variant-aware header styling.
/// Uses context to propagate the variant to child components.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::prelude::*;
///
/// #[component]
/// pub fn MyApp() -> impl IntoView {
///     view! {
///         <StackedLayout variant=StackedLayoutVariant::Dark>
///             <StackedLayoutHeader>
///                 <NavbarStart>
///                     <span class="text-xl font-bold">"MyApp"</span>
///                 </NavbarStart>
///                 <NavbarCenter>
///                     <StackedLayoutNav>
///                         <StackedLayoutNavItem href="/" active=true>"Home"</StackedLayoutNavItem>
///                     </StackedLayoutNav>
///                 </NavbarCenter>
///             </StackedLayoutHeader>
///             <StackedLayoutContent>
///                 <p>"Main content here"</p>
///             </StackedLayoutContent>
///         </StackedLayout>
///     }
/// }
/// ```
#[component]
pub fn StackedLayout(
    children: Children,
    /// Layout variant: Light, LightBordered, Dark, or Primary
    #[prop(optional, default = StackedLayoutVariant::Light)]
    variant: StackedLayoutVariant,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Provide context for child components
    let ctx = StackedLayoutContext { variant };
    provide_context(ctx);

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "min-h-screen flex flex-col",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
        </div>
    }
}

/// Context-aware header with variant styling.
///
/// Reads the variant from context to apply correct background/text colors.
/// Should contain NavbarStart, NavbarCenter, and/or NavbarEnd children.
#[component]
pub fn StackedLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Get variant from context
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let variant_classes = variant.header_classes();

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "navbar",
        &[variant_classes],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <header class={cls}>
            {children()}
        </header>
    }
}

/// Navigation container with style variants.
///
/// Renders as a horizontal menu with configurable styling.
#[component]
pub fn StackedLayoutNav(
    children: Children,
    /// Navigation style: Tabs, Pills, or Compact
    #[prop(optional, default = NavStyle::Tabs)]
    style: NavStyle,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let style_classes = style.container_classes();

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        style_classes,
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <nav class={cls}>
            {children()}
        </nav>
    }
}

/// Navigation item with active state support.
///
/// Reads the variant from context for proper active state styling.
#[component]
pub fn StackedLayoutNavItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Get variant from context
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();

    // Get nav style from context (default to Tabs if not found)
    let nav_style = use_context::<NavStyleContext>()
        .map(|c| c.style)
        .unwrap_or_default();

    let style = nav_style;
    let item_base = style.item_classes();
    let active_extra = if active { style.active_classes() } else { "" };
    let state_classes = if active {
        variant.nav_active_classes()
    } else {
        variant.nav_inactive_classes()
    };

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &[item_base, active_extra, state_classes],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let href_val = href.get().unwrap_or_default();

    view! {
        <a href={href_val} class={cls}>
            {children()}
        </a>
    }
}

/// Context for sharing nav style between StackedLayoutNav and StackedLayoutNavItem.
#[derive(Clone, Copy, Debug)]
struct NavStyleContext {
    style: NavStyle,
}

/// Main content area with optional overlap support.
///
/// When `overlap` is true, applies a negative margin to create an overlap effect
/// with the header above.
#[component]
pub fn StackedLayoutContent(
    children: Children,
    /// Apply negative margin to overlap header
    #[prop(optional)]
    overlap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["flex-1"];
    if overlap {
        m.push("-mt-16");
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <main class={cls}>
            {children()}
        </main>
    }
}

/// Header-integrated search input.
///
/// Reads the variant from context for proper styling.
#[component]
pub fn StackedLayoutSearch(
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Get variant from context
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let search_classes = variant.search_classes();

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "input input-sm w-full max-w-xs",
        &[search_classes],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let placeholder_val = placeholder.get().unwrap_or_else(|| "Search...".to_string());

    view! {
        <input
            type="text"
            class={cls}
            placeholder={placeholder_val}
        />
    }
}

/// Optional footer component.
#[component]
pub fn StackedLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "footer footer-center p-4 bg-base-100 text-base-content border-t border-base-300",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <footer class={cls}>
            {children()}
        </footer>
    }
}

// --- Panel ---
// A generic panel/column container for multi-column layouts.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelWidth {
    /// Extra-small panel (w-48)
    ExtraSmall,
    /// Small panel (w-56)
    Small,
    /// Medium / default panel (w-64)
    #[default]
    Medium,
    /// Large panel (w-80)
    Large,
    /// Extra-large panel (w-96)
    ExtraLarge,
}
impl PanelWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "w-48",
            Self::Small => "w-56",
            Self::Medium => "w-64",
            Self::Large => "w-80",
            Self::ExtraLarge => "w-96",
        }
    }
}

#[component]
pub fn Panel(
    children: Children,
    #[prop(optional)] width: PanelWidth,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "shrink-0 overflow-y-auto border-base-300",
        &[width.cls()],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <aside class={cls}>{children()}</aside> }
}

// --- SecondaryColumn ---
// A column between sidebar and main content for navigation or filters.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SecondaryColumnWidth {
    /// Narrow secondary column (w-64)
    Narrow,
    /// Default secondary column width (w-80)
    #[default]
    Default,
    /// Wide secondary column (w-96)
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnVariant {
    /// Default (transparent background)
    #[default]
    Default,
    /// Light background (bg-base-100)
    Light,
    /// Muted background (bg-base-200)
    Muted,
    /// Dark background (bg-neutral)
    Dark,
}
impl ColumnVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Light => "bg-base-100",
            Self::Muted => "bg-base-200",
            Self::Dark => "bg-neutral",
        }
    }
}

#[component]
pub fn SecondaryColumn(
    children: Children,
    #[prop(optional)] width: SecondaryColumnWidth,
    #[prop(optional)] variant: ColumnVariant,
    #[prop(optional)] bordered: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["flex flex-col overflow-y-auto", width.cls()];
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    if bordered {
        m.push("border-r border-base-300");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <aside class={cls}>{children()}</aside> }
}

// --- RightPanel ---
// A fixed right-side panel for secondary content, visible on xl+ screens.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelWidth {
    /// Narrow right panel (w-64)
    Narrow,
    /// Default right panel width (w-80)
    #[default]
    Default,
    /// Wide right panel (w-96)
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Breakpoint {
    /// Large screens (lg)
    Large,
    /// Extra large screens (xl) - default
    #[default]
    ExtraLarge,
}
impl Breakpoint {
    fn cls(&self) -> &'static str {
        match self {
            Self::Large => "hidden lg:block",
            Self::ExtraLarge => "hidden xl:block",
        }
    }
}

#[component]
pub fn RightPanel(
    children: Children,
    #[prop(optional)] width: RightPanelWidth,
    #[prop(optional)] breakpoint: Breakpoint,
    #[prop(optional)] fixed: bool,
    #[prop(optional)] variant: ColumnVariant,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["overflow-y-auto", width.cls(), breakpoint.cls()];
    if fixed {
        m.push("fixed inset-y-0 right-0");
    }
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <aside class={cls}>{children()}</aside> }
}
