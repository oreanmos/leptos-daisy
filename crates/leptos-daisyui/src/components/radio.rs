//! Radio component — daisyUI `radio`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::prelude::*;

#[component]
pub fn Radio(
    #[prop(into)] name: String,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("radio");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("radio"));
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "radio",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <input type="radio" name={name} value={value} class={cls} checked={checked} disabled={disabled} /> }
}
