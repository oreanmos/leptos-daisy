//! Badge component — daisyUI `badge`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::prelude::*;

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional)] outline: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("badge");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("badge"));
    }
    if let Some(v) = variant {
        let s = v.class("badge");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if outline {
        m.push("badge-outline".into());
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("badge", &r, class);
    view! { <span class=cls>{children()}</span> }
}
