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

use crate::utils::class::{build_class, class_signal};
use leptos::prelude::*;
use std::sync::Arc;

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

    /// Returns the CSS classes for navigation items (text, hover, active).
    fn nav_item_classes(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Light => ("", "hover:bg-base-100/30", "bg-base-100/50 font-medium"),
            Self::Dark => (
                "text-neutral-content/80",
                "hover:bg-neutral-content/10",
                "bg-neutral-content/20 font-medium",
            ),
            Self::Primary => (
                "text-primary-content/80",
                "hover:bg-primary-content/10",
                "bg-primary-content/20 font-medium",
            ),
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
    drawer_id: Arc<str>,
}

// --- Core Shell Components ---

/// The root shell component that wraps the entire sidebar layout.
///
/// Uses daisyUI's `drawer` component internally for responsive behavior.
/// On mobile, the sidebar is hidden and can be toggled via a hamburger menu.
/// On desktop (lg+), the sidebar is always visible when `open_on_large` is true.
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
    #[prop(optional, into)]
    id: MaybeProp<String>,
) -> impl IntoView {
    let drawer_id = id
        .get()
        .unwrap_or_else(|| "sidebar-layout-drawer".to_string());
    let ctx = SidebarLayoutContext {
        variant,
        width,
        drawer_id: Arc::from(drawer_id.as_str()),
    };
    provide_context(ctx);

    let base = if open_on_large {
        "drawer lg:drawer-open"
    } else {
        "drawer"
    };
    let cls = class_signal(base, &[], class);

    view! {
        <div class=cls>
            <input type="checkbox" id=drawer_id class="drawer-toggle" />
            {children()}
        </div>
    }
}

/// Wrapper for the main content area and optional top bar.
///
/// Maps to daisyUI's `drawer-content` class.
#[component]
pub fn SidebarLayoutContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-content flex flex-col min-h-screen", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// The drawer-side wrapper. Contains the overlay and panel.
#[component]
pub fn SidebarLayoutSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-side z-40", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// The backdrop overlay for mobile. Clicking it closes the drawer.
#[component]
pub fn SidebarLayoutOverlay(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let ctx = use_context::<SidebarLayoutContext>();
    let drawer_id = ctx
        .map(|c| c.drawer_id)
        .unwrap_or_else(|| Arc::from("sidebar-layout-drawer"));
    let cls = class_signal("drawer-overlay", &[], class);
    view! { <label for=Oco::Counted(drawer_id) aria-label="close sidebar" class=cls /> }
}

/// The actual sidebar panel containing navigation.
///
/// Inherits variant and width from the parent `SidebarLayout` context,
/// but can be overridden via props.
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
    let ctx = use_context::<SidebarLayoutContext>();
    let inherited_variant = ctx.as_ref().map(|c| c.variant).unwrap_or_default();
    let inherited_width = ctx.as_ref().map(|c| c.width).unwrap_or_default();
    let cls = move || {
        let resolved_variant = variant.get().unwrap_or(inherited_variant);
        let resolved_width = width.get().unwrap_or(inherited_width);
        let static_cls = build_class(
            resolved_variant.panel_classes(),
            &[
                resolved_width.width_class(),
                "min-h-full",
                "flex",
                "flex-col",
            ],
            None,
        );
        match class.get() {
            Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
            _ => static_cls,
        }
    };
    view! { <aside class=cls>{children()}</aside> }
}

/// The main content area within the layout.
///
/// Applies proper padding to account for the fixed sidebar on desktop.
#[component]
pub fn SidebarLayoutMain(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<SidebarLayoutContext>();
    let padding = ctx.map(|c| c.width.padding_class()).unwrap_or("lg:pl-72");
    let cls = class_signal("flex-1 overflow-y-auto", &[padding], class);
    view! { <main class=cls>{children()}</main> }
}

// --- Top Bar Components ---

/// Top bar wrapper that appears above the main content area.
#[component]
pub fn SidebarLayoutTopBar(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Show on mobile devices
    #[prop(optional, default = true)]
    mobile: bool,
    /// Show on desktop (lg+)
    #[prop(optional, default = true)]
    desktop: bool,
) -> impl IntoView {
    let vis = match (mobile, desktop) {
        (true, true) => "",
        (true, false) => "lg:hidden",
        (false, true) => "hidden lg:flex",
        (false, false) => "hidden",
    };
    let mut m: Vec<&str> = Vec::new();
    if !vis.is_empty() {
        m.push(vis);
    }
    let cls = class_signal(
        "sticky top-0 z-30 flex h-16 shrink-0 items-center gap-x-4 border-b px-4 shadow-sm sm:gap-x-6 sm:px-6 bg-base-100 border-base-300",
        &m,
        class,
    );
    view! { <header class=cls>{children()}</header> }
}

/// Start section of the top bar.
#[component]
pub fn SidebarLayoutTopBarStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex items-center gap-x-4", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Center section of the top bar.
#[component]
pub fn SidebarLayoutTopBarCenter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex flex-1 items-center justify-center", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// End section of the top bar.
#[component]
pub fn SidebarLayoutTopBarEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex items-center gap-x-4", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Mobile menu toggle button (hamburger icon).
#[component]
pub fn SidebarLayoutMobileMenuButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// ID of the drawer checkbox to toggle
    #[prop(optional, into)]
    drawer_id: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<SidebarLayoutContext>();
    let inherited_id = ctx
        .as_ref()
        .map(|c| c.drawer_id.clone())
        .unwrap_or_else(|| Arc::from("sidebar-layout-drawer"));
    let target_id = move || {
        if let Some(id) = drawer_id.get() {
            Oco::Owned(id)
        } else {
            Oco::Counted(inherited_id.clone())
        }
    };
    let cls = class_signal("btn btn-ghost btn-square lg:hidden", &[], class);

    view! {
        <label for=target_id class=cls aria-label="open sidebar">
            <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M4 6h16M4 12h16M4 18h16" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" />
            </svg>
        </label>
    }
}

// --- Sidebar Internal Components ---

/// Header area for logo/branding within the sidebar.
#[component]
pub fn SidebarLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex h-16 shrink-0 items-center px-6", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Navigation wrapper within the sidebar.
#[component]
pub fn SidebarLayoutNav(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "flex flex-1 flex-col gap-y-7 overflow-y-auto px-6 py-4",
        &[],
        class,
    );
    view! { <nav class=cls>{children()}</nav> }
}

/// Grouped navigation items with an optional title.
#[component]
pub fn SidebarLayoutNavSection(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] title: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex flex-col gap-y-2", &[], class);
    view! {
        <div class=cls>
            {move || {
                let title_text = title.get().unwrap_or_default();
                if title_text.is_empty() {
                    None
                } else {
                    Some(view! {
                        <div class="text-xs font-semibold uppercase tracking-wider opacity-60 px-2 mb-2">
                            {title_text}
                        </div>
                    })
                }
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
#[component]
pub fn SidebarLayoutNavItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] active: MaybeProp<bool>,
    /// CSS class list applied to an icon span. Prefer this over raw HTML.
    #[prop(optional, into)]
    icon: MaybeProp<String>,
    /// Raw SVG/HTML inserted via `inner_html`. Treat as trusted input only.
    #[prop(optional, into)]
    unsafe_icon_html: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SidebarLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();

    let (text_class, hover_class, active_class) = variant.nav_item_classes();

    let cls = move || {
        let mut m: Vec<&str> = vec![text_class, hover_class];
        if active.get().unwrap_or(false) {
            m.push(active_class);
        }
        let static_cls = build_class("flex items-center gap-3 rounded-lg px-3 py-2", &m, None);
        match class.get() {
            Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
            _ => static_cls,
        }
    };

    view! {
        <li>
            <a
                href=move || href.get().unwrap_or_else(|| "#".to_string())
                class=cls
                aria-current=move || if active.get().unwrap_or(false) { Some("page") } else { None }
            >
                {move || {
                    let raw_icon_html = unsafe_icon_html.get().unwrap_or_default();
                    if !raw_icon_html.is_empty() {
                        return Some(view! { <span class="w-5 h-5" inner_html=raw_icon_html /> }.into_any());
                    }

                    let icon_val = icon.get().unwrap_or_default();
                    if icon_val.is_empty() || icon_val.contains('<') || icon_val.contains('>') {
                        None
                    } else {
                        Some(view! { <span class={format!("w-5 h-5 {}", icon_val)} /> }.into_any())
                    }
                }}
                {children()}
            </a>
        </li>
    }
}

/// Footer area for user profile/settings within the sidebar.
#[component]
pub fn SidebarLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<SidebarLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let border = match variant {
        SidebarLayoutVariant::Light => "border-base-content/10",
        SidebarLayoutVariant::Dark => "border-neutral-content/10",
        SidebarLayoutVariant::Primary => "border-primary-content/10",
    };
    let cls = class_signal("mt-auto border-t px-6 py-4", &[border], class);
    view! { <div class=cls>{children()}</div> }
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
    fn test_nav_item_classes() {
        assert_eq!(
            SidebarLayoutVariant::Light.nav_item_classes(),
            ("", "hover:bg-base-100/30", "bg-base-100/50 font-medium")
        );
        assert_eq!(
            SidebarLayoutVariant::Dark.nav_item_classes(),
            (
                "text-neutral-content/80",
                "hover:bg-neutral-content/10",
                "bg-neutral-content/20 font-medium",
            )
        );
        assert_eq!(
            SidebarLayoutVariant::Primary.nav_item_classes(),
            (
                "text-primary-content/80",
                "hover:bg-primary-content/10",
                "bg-primary-content/20 font-medium",
            )
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
