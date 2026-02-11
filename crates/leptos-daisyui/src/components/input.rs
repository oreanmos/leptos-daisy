//! Input component — daisyUI `input`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] input_type: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] required: MaybeProp<bool>,
    #[prop(optional, into)] readonly: MaybeProp<bool>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("input"));
    }
    if let Some(v) = variant {
        let s = v.class("input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("input", &r, class);
    view! {
        <input
            type=move || input_type.get().unwrap_or_else(|| "text".to_string())
            id=move || id.get()
            name=move || name.get()
            prop:value=move || value.get().unwrap_or_default()
            class=cls
            placeholder=move || placeholder.get()
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            required=move || required.get().unwrap_or(false)
            readonly=move || readonly.get().unwrap_or(false)
        />
    }
}
