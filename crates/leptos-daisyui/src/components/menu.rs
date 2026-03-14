//! Menu component — daisyUI `menu`.
use crate::utils::class::{class_signal, class_signal_dynamic};
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Menu(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    if let Some(c) = color {
        let s = c.class("menu");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if responsive {
        m.push("menu-vertical lg:menu-horizontal".into());
    } else if horizontal {
        m.push("menu-horizontal".into());
    }
    if let Some(s) = size {
        m.push(s.class("menu"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("menu", &refs, class);
    view! { <ul class=cls role="menu">{children()}</ul> }.add_any_attr(attrs)
}

#[component]
pub fn MenuItem(
    children: Children,
    #[prop(optional, into)] active: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal_dynamic(
        "",
        move || {
            let mut m = Vec::new();
            if active.get().unwrap_or(false) {
                m.push("active".to_string());
            }
            if disabled.get().unwrap_or(false) {
                m.push("disabled".to_string());
            }
            m
        },
        class,
    );
    view! { <li class=cls role="menuitem">{children()}</li> }.add_any_attr(attrs)
}

#[component]
pub fn MenuTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("menu-title", &[], class);
    view! { <li class=cls>{children()}</li> }.add_any_attr(attrs)
}

#[component]
pub fn SubMenu(
    children: Children,
    #[prop(into)] title: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! {
        <li class=cls>
            <details>
                <summary>{title}</summary>
                <ul>{children()}</ul>
            </details>
        </li>
    }
    .add_any_attr(attrs)
}
