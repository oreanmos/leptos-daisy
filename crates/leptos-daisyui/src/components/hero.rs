//! Hero component — daisyUI `hero`.
use crate::utils::class::{build_class, class_signal};
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Hero(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional, default = true)] full_screen: bool,
    #[prop(optional, into)] min_height: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("place-items-center");
    }
    let base = if let Some(height) = min_height {
        format!("hero min-h-[{}]", height)
    } else if full_screen {
        "hero min-h-screen".to_string()
    } else {
        "hero".to_string()
    };
    let refs = m;
    let static_cls = build_class(&base, &refs, None);
    let cls = move || match class.get() {
        Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
        _ => static_cls.clone(),
    };
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn HeroContent(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("text-center");
    }
    let cls = class_signal("hero-content", &m, class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn HeroOverlay(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("hero-overlay", &[], class);
    view! { <div class=cls></div> }.add_any_attr(attrs)
}
