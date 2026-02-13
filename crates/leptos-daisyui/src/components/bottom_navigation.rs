//! Bottom Navigation component — daisyUI `btm-nav`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// A DaisyUI bottom navigation bar component for mobile-style navigation.
#[component]
pub fn BottomNavigation(
    children: Children,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(s) = size {
        m.push(s.class("btm-nav"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btm-nav", &refs, class);
    view! {
        <nav class=cls aria-label=move || aria_label.get().unwrap_or_else(|| "Bottom navigation".to_string())>
            {children()}
        </nav>
    }.add_any_attr(attrs)
}

/// A label sub-component for bottom navigation items.
#[component]
pub fn BottomNavigationLabel(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("btm-nav-label", &[], class);
    view! { <span class=cls>{children()}</span> }.add_any_attr(attrs)
}
