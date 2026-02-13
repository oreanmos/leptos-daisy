//! Pagination component — daisyUI uses `join` for pagination.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Pagination(
    children: Children,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(s) = size {
        m.push(s.class("btn"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("join", &refs, class);
    view! {
        <nav aria-label=move || aria_label.get().unwrap_or_else(|| "Pagination".to_string())>
            <div class=cls>{children()}</div>
        </nav>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn PaginationItem(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["join-item", "btn"];
    if active {
        m.push("btn-active");
    }
    let cls = class_signal("", &m, class);
    view! {
        <button class=cls type="button" disabled=disabled
            aria-current={if active { Some("page") } else { None }}
        >{children()}</button>
    }
    .add_any_attr(attrs)
}
