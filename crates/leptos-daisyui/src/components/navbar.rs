//! Navbar component — daisyUI `navbar`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;
use std::borrow::Cow;

#[component]
pub fn Navbar(
    children: Children,
    #[prop(optional)] glass: bool,
    #[prop(optional)] rounded: bool,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<Cow<'static, str>> = Vec::new();
    if glass {
        m.push(Cow::Borrowed("glass"));
    }
    if rounded {
        m.push(Cow::Borrowed("rounded-box"));
    }
    if let Some(c) = color {
        let s = c.class("navbar");
        if !s.is_empty() {
            m.push(Cow::Owned(s));
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_ref()).collect();
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
