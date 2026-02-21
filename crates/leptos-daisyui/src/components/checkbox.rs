//! Checkbox component — daisyUI `checkbox`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("checkbox");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("checkbox"));
    }
    if let Some(v) = variant {
        let s = v.class("checkbox");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("checkbox", &r, class);

    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <input
            type="checkbox"
            id=move || id.get()
            name=move || name.get()
            value=move || value.get()
            class=cls
            aria-label=move || aria_label.get()
            checked=move || checked.get().unwrap_or(false)
            disabled=move || disabled.get().unwrap_or(false)
            on:change=handle_change
        />
    }
    .add_any_attr(attrs)
}
