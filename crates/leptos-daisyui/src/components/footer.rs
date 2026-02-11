//! Footer component — daisyUI `footer`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Footer(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("footer-center");
    }
    if horizontal {
        m.push("footer-horizontal");
    }
    if vertical {
        m.push("footer-vertical");
    }
    let cls = class_signal("footer", &m, class);
    view! { <footer class=cls>{children()}</footer> }
}

#[component]
pub fn FooterTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("footer-title", &[], class);
    view! { <h6 class=cls>{children()}</h6> }
}
