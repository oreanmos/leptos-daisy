//! Navbar component — daisyUI `navbar`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Navbar(
    children: Children,
    #[prop(optional)] glass: bool,
    #[prop(optional)] rounded: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if glass {
        m.push("glass");
    }
    if rounded {
        m.push("rounded-box");
    }
    let cls = class_signal("navbar", &m, class);
    view! { <nav class=cls>{children()}</nav> }
}

#[component]
pub fn NavbarStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("navbar-start", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn NavbarCenter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("navbar-center", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn NavbarEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("navbar-end", &[], class);
    view! { <div class=cls>{children()}</div> }
}
