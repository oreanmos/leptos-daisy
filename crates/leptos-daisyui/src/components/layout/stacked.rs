use crate::utils::class::{build_class, class_signal};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackedLayoutVariant {
    #[default]
    Light,
    LightBordered,
    Dark,
    Primary,
}
impl StackedLayoutVariant {
    fn header_classes(&self) -> &'static str {
        match self {
            Self::Light => "bg-base-100 text-base-content",
            Self::LightBordered => "bg-base-100 text-base-content border-b border-base-300",
            Self::Dark => "bg-neutral text-neutral-content",
            Self::Primary => "bg-primary text-primary-content",
        }
    }
    fn nav_active_classes(&self) -> &'static str {
        match self {
            Self::Light | Self::LightBordered => "border-b-2 border-primary text-base-content",
            Self::Dark => "border-b-2 border-neutral-content text-neutral-content",
            Self::Primary => "border-b-2 border-primary-content text-primary-content",
        }
    }
    fn nav_inactive_classes(&self) -> &'static str {
        match self {
            Self::Light | Self::LightBordered => "text-base-content/70 hover:text-base-content",
            Self::Dark => "text-neutral-content/70 hover:text-neutral-content",
            Self::Primary => "text-primary-content/70 hover:text-primary-content",
        }
    }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavStyle {
    #[default]
    Tabs,
    Pills,
    Compact,
}
impl NavStyle {
    fn container_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "menu menu-horizontal gap-1",
            Self::Pills => "flex items-center gap-1",
            Self::Compact => "flex items-center gap-0.5",
        }
    }
    fn item_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "px-3 py-2 text-sm font-medium",
            Self::Pills => "px-3 py-2 text-sm font-medium rounded-lg",
            Self::Compact => "px-2 py-1.5 text-sm",
        }
    }
    fn active_classes(&self) -> &'static str {
        match self {
            Self::Tabs => "",
            Self::Pills => "bg-white/20",
            Self::Compact => "bg-white/10 rounded",
        }
    }
}

#[derive(Clone, Debug)]
struct StackedLayoutContext {
    variant: StackedLayoutVariant,
}

#[derive(Clone, Copy, Debug)]
struct NavStyleContext {
    style: NavStyle,
}

#[component]
pub fn StackedLayout(
    children: Children,
    #[prop(optional, default = StackedLayoutVariant::Light)] variant: StackedLayoutVariant,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let ctx = StackedLayoutContext { variant };
    provide_context(ctx);
    let cls = class_signal("min-h-screen flex flex-col", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StackedLayoutHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let vc = variant.header_classes();
    let cls = class_signal("navbar", &[vc], class);
    view! { <header class=cls>{children()}</header> }
}

#[component]
pub fn StackedLayoutNav(
    children: Children,
    #[prop(optional, default = NavStyle::Tabs)] style: NavStyle,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    provide_context(NavStyleContext { style });
    let sc = style.container_classes();
    let static_cls = build_class(sc, &[], None);
    let cls = move || match class.get() {
        Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
        _ => static_cls.clone(),
    };
    view! { <nav class=cls>{children()}</nav> }
}

#[component]
pub fn StackedLayoutNavItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] active: MaybeProp<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let nav_style = use_context::<NavStyleContext>()
        .map(|c| c.style)
        .unwrap_or_default();
    let cls = move || {
        let is_active = active.get().unwrap_or(false);
        let item_base = nav_style.item_classes();
        let active_extra = if is_active {
            nav_style.active_classes()
        } else {
            ""
        };
        let state_classes = if is_active {
            variant.nav_active_classes()
        } else {
            variant.nav_inactive_classes()
        };

        let mut m: Vec<&str> = vec![item_base];
        if !active_extra.is_empty() {
            m.push(active_extra);
        }
        m.push(state_classes);
        let static_cls = build_class("", &m, None);
        match class.get() {
            Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
            _ => static_cls,
        }
    };

    view! {
        <a href=move || href.get().unwrap_or_else(|| "#".to_string()) class=cls>
            {children()}
        </a>
    }
}

#[component]
pub fn StackedLayoutContent(
    children: Children,
    #[prop(optional)] overlap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["flex-1"];
    if overlap {
        m.push("-mt-16");
    }
    let cls = class_signal("", &m, class);
    view! { <main class=cls>{children()}</main> }
}

#[component]
pub fn StackedLayoutSearch(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    let ctx = use_context::<StackedLayoutContext>();
    let variant = ctx.map(|c| c.variant).unwrap_or_default();
    let sc = variant.search_classes();
    let cls = class_signal("input input-sm w-full max-w-xs", &[sc], class);
    view! {
        <input
            type="text"
            class=cls
            placeholder=move || placeholder.get().unwrap_or_else(|| "Search...".to_string())
        />
    }
}

#[component]
pub fn StackedLayoutFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "footer footer-center p-4 bg-base-100 text-base-content border-t border-base-300",
        &[],
        class,
    );
    view! { <footer class=cls>{children()}</footer> }
}
