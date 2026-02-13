//! Range component — daisyUI `range`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Range(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] min: MaybeProp<f64>,
    #[prop(optional, into)] max: MaybeProp<f64>,
    #[prop(optional, into)] value: MaybeProp<f64>,
    #[prop(optional, into)] step: MaybeProp<f64>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_input: Option<Callback<ev::Event>>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("range");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("range"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("range", &refs, class);

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
        <input
            type="range"
            id=move || id.get()
            name=move || name.get()
            min=move || min.get().map(|v| v.to_string())
            max=move || max.get().map(|v| v.to_string())
            value=move || value.get().map(|v| v.to_string())
            step=move || step.get().map(|v| v.to_string())
            class=cls
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            on:input=handle_input
            on:change=handle_change
        />
    }
    .add_any_attr(attrs)
}
