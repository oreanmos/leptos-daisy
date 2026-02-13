//! Textarea component — daisyUI `textarea`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Textarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] rows: MaybeProp<u32>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] readonly: MaybeProp<bool>,
    #[prop(optional, into)] required: MaybeProp<bool>,
    #[prop(optional, into)] on_input: Option<Callback<ev::Event>>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("textarea");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("textarea"));
    }
    if let Some(v) = variant {
        let s = v.class("textarea");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("textarea", &refs, class);

    let handle_input = move |ev: ev::Event| {
        if let Some(cb) = on_input {
            cb.run(ev);
        }
    };
    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <textarea
            id=move || id.get()
            name=move || name.get()
            class=cls
            placeholder=move || placeholder.get()
            prop:value=move || value.get().unwrap_or_default()
            rows=move || rows.get().map(|v| v.to_string())
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            readonly=move || readonly.get().unwrap_or(false)
            required=move || required.get().unwrap_or(false)
            on:input=handle_input
            on:change=handle_change
        ></textarea>
    }
    .add_any_attr(attrs)
}
