//! Sidebar Layout components — daisyUI `drawer` based responsive sidebar layout.
//!
//! This module provides a comprehensive sidebar layout system inspired by Tailwind Plus
//! sidebar application shell patterns, leveraging daisyUI's native `drawer` component
//! for mobile responsiveness.
//!
//! # Example
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::prelude::*;
//!
//! #[component]
//! pub fn MyApp() -> impl IntoView {
//!     view! {
//!         <SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
//!             <SidebarLayoutContent>
//!                 <SidebarLayoutTopBar>
//!                     <SidebarLayoutTopBarStart>
//!                         <SidebarLayoutMobileMenuButton />
//!                     </SidebarLayoutTopBarStart>
//!                     <SidebarLayoutTopBarCenter>
//!                         <span class="text-lg font-semibold">"Page Title"</span>
//!                     </SidebarLayoutTopBarCenter>
//!                     <SidebarLayoutTopBarEnd>
//!                         <button class="btn btn-ghost btn-circle">
//!                             <span class="icon-notification" />
//!                         </button>
//!                     </SidebarLayoutTopBarEnd>
//!                 </SidebarLayoutTopBar>
//!                 <SidebarLayoutMain>
//!                     <p>"Main content here"</p>
//!                 </SidebarLayoutMain>
//!             </SidebarLayoutContent>
//!             <SidebarLayoutSide>
//!                 <SidebarLayoutOverlay />
//!                 <SidebarLayoutPanel>
//!                     <SidebarLayoutHeader>
//!                         <span class="text-xl font-bold">"My App"</span>
//!                     </SidebarLayoutHeader>
//!                     <SidebarLayoutNav>
//!                         <SidebarLayoutNavSection title="Main">
//!                             <SidebarLayoutNavItem href="/" active=true icon="icon-home">
//!                                 "Home"
//!                             </SidebarLayoutNavItem>
//!                         </SidebarLayoutNavSection>
//!                     </SidebarLayoutNav>
//!                     <SidebarLayoutFooter>
//!                         <div class="flex items-center gap-3">
//!                             <div class="avatar">
//!                                 <div class="w-10 rounded-full">
//!                                     <img src="user.jpg" />
//!                                 </div>
//!                             </div>
//!                             <div>
//!                                 <div class="font-medium">"John Doe"</div>
//!                                 <div class="text-sm opacity-60">"john@example.com"</div>
//!                             </div>
//!                         </div>
//!                     </SidebarLayoutFooter>
//!                 </SidebarLayoutPanel>
//!             </SidebarLayoutSide>
//!         </SidebarLayout>
//!     }
//! }
//! ```

use crate::utils::class::build_class;
use leptos::prelude::*;

// --- Enums ---

/// Sidebar variant determining the color scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarLayoutVariant {
    /// Light sidebar with base-200 background
    #[default]
    Light,
    /// Dark sidebar with neutral background
    Dark,
    /// Primary-colored sidebar
    Primary,
}

impl SidebarLayoutVariant {
    /// Returns the CSS classes for this variant.
    fn panel_classes(&self) -> &'static str {
        match self {
            Self::Light => "bg-base-200 text-base-content",
            Self::Dark => "bg-neutral text-neutral-content",
            Self::Primary => "bg-primary text-primary-content",
        }
    }
}

/// Sidebar width options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarLayoutWidth {
    /// Narrow sidebar (16rem / 256px)
    Narrow,
    /// Default sidebar width (18rem / 288px)
    #[default]
    Default,
    /// Wide sidebar (20rem / 320px)
    Wide,
}

impl SidebarLayoutWidth {
    /// Returns the width CSS class.
    fn width_class(&self) -> &'static str {
        match self {
            Self::Narrow => "w-64",
            Self::Default => "w-72",
            Self::Wide => "w-80",
        }
    }

    /// Returns the padding class for main content to account for sidebar.
    fn padding_class(&self) -> &'static str {
        match self {
            Self::Narrow => "lg:pl-64",
            Self::Default => "lg:pl-72",
            Self::Wide => "lg:pl-80",
        }
    }
}

// --- Context for sharing state between components ---

/// Context provided by SidebarLayout to child components.
#[derive(Clone, Debug)]
struct SidebarLayoutContext {
    variant: SidebarLayoutVariant,
    width: SidebarLayoutWidth,
    drawer_id: String,
}

// --- Core Shell Components ---

/// The root shell component that wraps the entire sidebar layout.
///
/// Uses daisyUI's `drawer` component internally for responsive behavior.
/// On mobile, the sidebar is hidden and can be toggled via a hamburger menu.
/// On desktop (lg+), the sidebar is always visible when `open_on_large` is true.
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
///         <SidebarLayout variant=SidebarLayoutVariant::Dark open_on_large=true>
///             <SidebarLayoutContent>
///                 <SidebarLayoutMain>
///                     <p>"Main content here"</p>
///                 </SidebarLayoutMain>
///             </SidebarLayoutContent>
///             <SidebarLayoutSide>
///                 <SidebarLayoutOverlay />
///                 <SidebarLayoutPanel>
///                     <p>"Sidebar content"</p>
///                 </SidebarLayoutPanel>
///             </SidebarLayoutSide>
///         </SidebarLayout>
///     }
/// }
/// ```
#[component]
pub fn SidebarLayout(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Sidebar variant: Light, Dark, or Primary
    #[prop(optional, default = SidebarLayoutVariant::Light)]
    variant: SidebarLayoutVariant,
    /// Sidebar width: Narrow, Default, or Wide
    #[prop(optional, default = SidebarLayoutWidth::Default)]
    width: SidebarLayoutWidth,
    /// Always show sidebar on lg+ screens
    #[prop(optional, default = true)]
    open_on_large: bool,
    /// ID for the drawer toggle checkbox. Defaults to "sidebar-layout-drawer".
    /// Used by SidebarLayoutMobileMenuButton to toggle the drawer.
    #[prop(optional, into)]
    id: MaybeProp<String>,
) -> impl IntoView {
    // Provide context for child components
    let drawer_id = id
        .get()
        .unwrap_or_else(|| "sidebar-layout-drawer".to_string());
    let ctx = SidebarLayoutContext {
        variant,
        width,
        drawer_id: drawer_id.clone(),
    };
    provide_context(ctx);

    let base_classes = if open_on_large {
        "drawer lg:drawer-open"
    } else {
        "drawer"
    };

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        base_classes,
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            <input type="checkbox" id={drawer_id} class="drawer-toggle" />
            {children()}
        </div>
    }
}

/// Wrapper for the main content area and optional top bar.
///
/// Maps to daisyUI's `drawer-content` class. Should contain
/// `SidebarLayoutTopBar` (optional) and `SidebarLayoutMain`.
#[component]
pub fn SidebarLayoutContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-content flex flex-col min-h-screen",
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

/// The drawer-side wrapper. Contains the overlay and panel.
///
/// Should be placed as a sibling to `SidebarLayoutContent` within
/// `SidebarLayout`. Contains `SidebarLayoutOverlay` and `SidebarLayoutPanel`.
#[component]
pub fn SidebarLayoutSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-side z-40",
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

/// The backdrop overlay for mobile. Clicking it closes the drawer.
///
/// This component renders as a `<label>` with `for` attribute pointing
/// to the drawer toggle checkbox. When clicked, it closes the sidebar
/// on mobile devices.
///
/// # Accessibility
///
/// Includes `aria-label="close sidebar"` for screen readers.
#[component]
pub fn SidebarLayoutOverlay(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    // Get the drawer_id from context
    let ctx = use_context::<SidebarLayoutContext>();
    let drawer_id = ctx
        .map(|c| c.drawer_id)
        .unwrap_or_else(|| "sidebar-layout-drawer".to_string());

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-overlay",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label for={drawer_id} aria-label="close sidebar" class={cls} />
    }
}

/// The actual sidebar panel containing navigation.
///
/// This is the visible sidebar that appears on desktop or slides out
/// on mobile. It inherits variant and width from the parent `SidebarLayout`
/// context, but can be overridden via props.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutPanel>
///         <SidebarLayoutHeader>
///             <span>"My App"</span>
///         </SidebarLayoutHeader>
///         <SidebarLayoutNav>
///             // Navigation items
///         </SidebarLayoutNav>
///     </SidebarLayoutPanel>
/// }
/// ```
#[component]
pub fn SidebarLayoutPanel(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Override variant from parent context
    #[prop(optional, into)]
    variant: MaybeProp<SidebarLayoutVariant>,
    /// Override width from parent context
    #[prop(optional, into)]
    width: MaybeProp<SidebarLayoutWidth>,
) -> impl IntoView {
    // Get context from parent SidebarLayout, or use defaults
    let ctx = use_context::<SidebarLayoutContext>();
    let resolved_variant = variant
        .get()
        .unwrap_or_else(|| ctx.as_ref().map(|c| c.variant).unwrap_or_default());
    let resolved_width = width
        .get()
        .unwrap_or_else(|| ctx.as_ref().map(|c| c.width).unwrap_or_default());

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        resolved_variant.panel_classes(),
        &[
            resolved_width.width_class(),
            "min-h-full",
            "flex",
            "flex-col",
        ],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <aside class={cls}>
            {children()}
        </aside>
    }
}

/// The main content area within the layout.
///
/// Applies proper padding to account for the fixed sidebar on desktop.
/// The padding is automatically set based on the sidebar width from
/// the parent `SidebarLayout` context.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutMain>
///         <div class="p-6">
///             <h1>"Page Title"</h1>
///             // Page content
///         </div>
///     </SidebarLayoutMain>
/// }
/// ```
#[component]
pub fn SidebarLayoutMain(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Get context from parent SidebarLayout for width
    let ctx = use_context::<SidebarLayoutContext>();
    let padding_class = ctx.map(|c| c.width.padding_class()).unwrap_or("lg:pl-72");

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex-1 overflow-y-auto",
        &[padding_class],
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

// --- Top Bar Components ---

/// Top bar wrapper that appears above the main content area.
///
/// Contains the mobile menu button, breadcrumbs, search, and user actions.
/// Can be configured to show/hide on mobile or desktop independently.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutTopBar>
///         <SidebarLayoutTopBarStart>
///             <SidebarLayoutMobileMenuButton />
///         </SidebarLayoutTopBarStart>
///         <SidebarLayoutTopBarCenter>
///             <span class="text-lg font-semibold">"Page Title"</span>
///         </SidebarLayoutTopBarCenter>
///         <SidebarLayoutTopBarEnd>
///             <button class="btn btn-ghost btn-circle">
///                 <span class="icon-notification" />
///             </button>
///         </SidebarLayoutTopBarEnd>
///     </SidebarLayoutTopBar>
/// }
/// ```
#[component]
pub fn SidebarLayoutTopBar(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Show on mobile devices. If false, adds `hidden lg:flex`.
    #[prop(optional, default = true)]
    mobile: bool,
    /// Show on desktop (lg+). If false, adds `lg:hidden`.
    #[prop(optional, default = true)]
    desktop: bool,
) -> impl IntoView {
    // Build visibility classes based on props
    let visibility_classes = match (mobile, desktop) {
        (true, true) => "",                // Show everywhere
        (true, false) => "lg:hidden",      // Hide on desktop
        (false, true) => "hidden lg:flex", // Hide on mobile, show on desktop
        (false, false) => "hidden",        // Hide everywhere (unusual but supported)
    };

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "sticky top-0 z-30 flex h-16 shrink-0 items-center gap-x-4 border-b px-4 shadow-sm sm:gap-x-6 sm:px-6 bg-base-100 border-base-300",
        &[visibility_classes],
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

/// Start section of the top bar.
///
/// Typically contains the mobile menu button and breadcrumbs.
/// Uses flexbox with items centered and gap spacing.
#[component]
pub fn SidebarLayoutTopBarStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex items-center gap-x-4",
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

/// Center section of the top bar.
///
/// Typically contains search input or page title.
/// Uses flexbox with items centered and takes remaining space (flex-1).
#[component]
pub fn SidebarLayoutTopBarCenter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex flex-1 items-center justify-center",
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

/// End section of the top bar.
///
/// Typically contains notifications, user dropdown, and other actions.
/// Uses flexbox with items centered and gap spacing.
#[component]
pub fn SidebarLayoutTopBarEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex items-center gap-x-4",
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

/// Mobile menu toggle button (hamburger icon).
///
/// This button toggles the sidebar drawer on mobile devices.
/// It uses a `<label>` with `for` attribute targeting the drawer's checkbox input.
///
/// # Props
///
/// - `drawer_id`: The ID of the drawer checkbox to toggle. Defaults to "sidebar-layout-drawer".
///   This should match the `id` prop passed to `SidebarLayout`.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutMobileMenuButton drawer_id="my-custom-drawer" />
/// }
/// ```
#[component]
pub fn SidebarLayoutMobileMenuButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// ID of the drawer checkbox to toggle. Defaults to "sidebar-layout-drawer".
    #[prop(optional, into)]
    drawer_id: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "btn btn-ghost btn-square lg:hidden",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    // Get drawer_id from context if not provided
    let ctx = use_context::<SidebarLayoutContext>();
    let target_id = drawer_id
        .get()
        .or_else(|| ctx.map(|c| c.drawer_id))
        .unwrap_or_else(|| "sidebar-layout-drawer".to_string());

    view! {
        <label for={target_id} class={cls} aria-label="open sidebar">
            <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M4 6h16M4 12h16M4 18h16" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" />
            </svg>
        </label>
    }
}

// --- Sidebar Internal Components ---

/// Header area for logo/branding within the sidebar.
///
/// Typically placed at the top of `SidebarLayoutPanel` and contains
/// the application logo or brand name.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutPanel>
///         <SidebarLayoutHeader>
///             <span class="text-xl font-bold">"My App"</span>
///         </SidebarLayoutHeader>
///     </SidebarLayoutPanel>
/// }
/// ```
#[component]
pub fn SidebarLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex h-16 shrink-0 items-center px-6",
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

/// Navigation wrapper within the sidebar.
///
/// Uses daisyUI menu classes internally. Contains navigation sections
/// or individual navigation items.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutNav>
///         <SidebarLayoutNavSection title="Main">
///             <SidebarLayoutNavItem href="/" active=true>
///                 "Home"
///             </SidebarLayoutNavItem>
///         </SidebarLayoutNavSection>
///     </SidebarLayoutNav>
/// }
/// ```
#[component]
pub fn SidebarLayoutNav(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex flex-1 flex-col gap-y-7 overflow-y-auto px-6 py-4",
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

/// Grouped navigation items with an optional title.
///
/// Used to organize navigation items into logical groups within
/// the sidebar navigation.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutNavSection title="Settings">
///         <SidebarLayoutNavItem href="/profile">
///             "Profile"
///         </SidebarLayoutNavItem>
///         <SidebarLayoutNavItem href="/security">
///             "Security"
///         </SidebarLayoutNavItem>
///     </SidebarLayoutNavSection>
/// }
/// ```
#[component]
pub fn SidebarLayoutNavSection(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] title: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "flex flex-col gap-y-2",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let title_text = title.get().unwrap_or_default();

    view! {
        <div class={cls}>
            {if !title_text.is_empty() {
                view! {
                    <div class="text-xs font-semibold uppercase tracking-wider opacity-60 px-2 mb-2">
                        {title_text}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
            <ul class="menu menu-sm gap-1 p-0">
                {children()}
            </ul>
        </div>
    }
}

/// Individual navigation item within the sidebar.
///
/// Supports variant-aware styling based on the parent `SidebarLayout` variant.
/// The active state is highlighted appropriately for the current variant.
///
/// # Icons
///
/// The `icon` prop accepts either:
/// - A daisyUI icon class string (e.g., `"icon-home"`)
/// - An SVG string starting with `<svg` (rendered as raw HTML)
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutNavItem href="/dashboard" active=true icon="icon-home">
///         "Dashboard"
///     </SidebarLayoutNavItem>
/// }
/// ```
#[component]
pub fn SidebarLayoutNavItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] icon: MaybeProp<String>,
) -> impl IntoView {
    // Get context for variant-aware styling
    let ctx = use_context::<SidebarLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();

    // Build variant-aware classes for the link
    let (text_class, hover_class, active_class) = match variant {
        SidebarLayoutVariant::Light => ("", "hover:bg-base-100/30", "bg-base-100/50 font-medium"),
        SidebarLayoutVariant::Dark => (
            "text-neutral-content/80",
            "hover:bg-neutral-content/10",
            "bg-neutral-content/20 font-medium",
        ),
        SidebarLayoutVariant::Primary => (
            "text-primary-content/80",
            "hover:bg-primary-content/10",
            "bg-primary-content/20 font-medium",
        ),
    };

    let uc = class.get().unwrap_or_default();
    let link_classes = build_class(
        "flex items-center gap-3 rounded-lg px-3 py-2",
        &[text_class, hover_class],
        if active {
            Some(active_class)
        } else if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let href_val = href.get().unwrap_or_else(|| "#".to_string());
    let icon_val = icon.get().unwrap_or_default();

    // Determine if icon is SVG or class
    let icon_html = if icon_val.starts_with("<svg") {
        view! {
            <span class="w-5 h-5" inner_html={icon_val} />
        }
        .into_any()
    } else if !icon_val.is_empty() {
        view! {
            <span class={format!("w-5 h-5 {}", icon_val)} />
        }
        .into_any()
    } else {
        ().into_any()
    };

    view! {
        <li>
            <a href={href_val} class={link_classes}>
                {icon_html}
                {children()}
            </a>
        </li>
    }
}

/// Footer area for user profile/settings within the sidebar.
///
/// Typically placed at the bottom of `SidebarLayoutPanel` and contains
/// user information, settings link, or logout button.
///
/// # Example
///
/// ```rust
/// view! {
///     <SidebarLayoutPanel>
///         // ... header and nav ...
///         <SidebarLayoutFooter>
///             <div class="flex items-center gap-3">
///                 <div class="avatar">
///                     <div class="w-10 rounded-full">
///                         <img src="user.jpg" />
///                     </div>
///                 </div>
///                 <div>
///                     <div class="font-medium">"John Doe"</div>
///                     <div class="text-sm opacity-60">"john@example.com"</div>
///                 </div>
///             </div>
///         </SidebarLayoutFooter>
///     </SidebarLayoutPanel>
/// }
/// ```
#[component]
pub fn SidebarLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    // Get context for variant-aware border color
    let ctx = use_context::<SidebarLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();

    let border_class = match variant {
        SidebarLayoutVariant::Light => "border-base-content/10",
        SidebarLayoutVariant::Dark => "border-neutral-content/10",
        SidebarLayoutVariant::Primary => "border-primary-content/10",
    };

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mt-auto border-t px-6 py-4",
        &[border_class],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_classes() {
        assert_eq!(
            SidebarLayoutVariant::Light.panel_classes(),
            "bg-base-200 text-base-content"
        );
        assert_eq!(
            SidebarLayoutVariant::Dark.panel_classes(),
            "bg-neutral text-neutral-content"
        );
        assert_eq!(
            SidebarLayoutVariant::Primary.panel_classes(),
            "bg-primary text-primary-content"
        );
    }

    #[test]
    fn test_width_classes() {
        assert_eq!(SidebarLayoutWidth::Narrow.width_class(), "w-64");
        assert_eq!(SidebarLayoutWidth::Default.width_class(), "w-72");
        assert_eq!(SidebarLayoutWidth::Wide.width_class(), "w-80");
    }

    #[test]
    fn test_padding_classes() {
        assert_eq!(SidebarLayoutWidth::Narrow.padding_class(), "lg:pl-64");
        assert_eq!(SidebarLayoutWidth::Default.padding_class(), "lg:pl-72");
        assert_eq!(SidebarLayoutWidth::Wide.padding_class(), "lg:pl-80");
    }
}
