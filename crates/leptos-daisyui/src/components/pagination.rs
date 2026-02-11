//! Pagination component — daisyUI uses `join` for pagination.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::prelude::*;

#[component]
pub fn Pagination(
    children: Children,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(s) = size {
        m.push(s.class("btn"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("join", &refs, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn PaginationItem(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["join-item", "btn"];
    if active {
        m.push("btn-active");
    }
    let cls = class_signal("", &m, class);
    view! { <button class=cls type="button" disabled=disabled>{children()}</button> }
}
