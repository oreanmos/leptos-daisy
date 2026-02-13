//! Select component — daisyUI `select`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Select(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("select");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("select"));
    }
    if let Some(v) = variant {
        let s = v.class("select");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("select", &refs, class);

    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <select
            id=move || id.get()
            name=move || name.get()
            class=cls
            prop:value=move || value.get().unwrap_or_default()
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            on:change=handle_change
        >
            {children()}
        </select>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn SelectOption(
    children: Children,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] selected: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <option
            value=move || value.get()
            disabled=move || disabled.get().unwrap_or(false)
            selected=move || selected.get().unwrap_or(false)
        >
            {children()}
        </option>
    }
}
