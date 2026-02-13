//! Navbar component — daisyUI `navbar`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Navbar(
    children: Children,
    #[prop(optional)] glass: bool,
    #[prop(optional)] rounded: bool,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    if glass {
        m.push("glass".into());
    }
    if rounded {
        m.push("rounded-box".into());
    }
    if let Some(c) = color {
        let s = c.class("navbar");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("navbar", &refs, class);
    view! { <nav class=cls>{children()}</nav> }.add_any_attr(attrs)
}

#[component]
pub fn NavbarStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("navbar-start", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn NavbarCenter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("navbar-center", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn NavbarEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("navbar-end", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}
