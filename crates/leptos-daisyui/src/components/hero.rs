//! Hero component — daisyUI `hero`.
use crate::utils::class::{build_class, class_signal};
use leptos::prelude::*;

#[component]
pub fn Hero(
    children: Children,
    #[prop(optional)] overlay: bool,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] min_height: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if overlay {
        m.push("hero-overlay");
    }
    if center {
        m.push("place-items-center");
    }
    let base = if let Some(height) = min_height {
        format!("hero min-h-[{}]", height)
    } else {
        "hero min-h-screen".to_string()
    };
    let refs = m;
    let static_cls = build_class(&base, &refs, None);
    let cls = move || match class.get() {
        Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
        _ => static_cls.clone(),
    };
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn HeroContent(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("text-center");
    }
    let cls = class_signal("hero-content", &m, class);
    view! { <div class=cls>{children()}</div> }
}
