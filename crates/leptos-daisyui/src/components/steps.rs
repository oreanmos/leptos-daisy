//! Steps component — daisyUI `steps`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Steps(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if responsive {
        m.push("steps-vertical md:steps-horizontal");
    } else if vertical {
        m.push("steps-vertical");
    } else if horizontal {
        m.push("steps-horizontal");
    }
    let cls = class_signal("steps", &m, class);
    view! { <ul class=cls>{children()}</ul> }.add_any_attr(attrs)
}

#[component]
pub fn Step(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] data_content: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("step");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("step", &refs, class);
    view! {
        <li class=cls data-content=data_content
            aria-current={if active { Some("step") } else { None }}
        >{children()}</li>
    }
    .add_any_attr(attrs)
}
