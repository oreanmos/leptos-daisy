//! Join component — daisyUI `join`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Join(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if responsive {
        m.push("join-vertical md:join-horizontal");
    } else if vertical {
        m.push("join-vertical");
    } else if horizontal {
        m.push("join-horizontal");
    }
    let cls = class_signal("join", &m, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn JoinItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("join-item", &[], class);
    view! { <div class=cls>{children()}</div> }
}
