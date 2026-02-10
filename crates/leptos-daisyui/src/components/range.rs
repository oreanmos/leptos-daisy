//! Range component — daisyUI `range`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::prelude::*;

/// A daisyUI range slider component.
#[component]
pub fn Range(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] min: Option<i32>,
    #[prop(optional, into)] max: Option<i32>,
    #[prop(optional, into)] step: Option<i32>,
    #[prop(optional, into)] value: Option<i32>,
    #[prop(optional, into)] class: MaybeProp<String>,
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

    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "range",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <input
            type="range"
            class={cls}
            disabled={disabled}
            min={min}
            max={max}
            step={step}
            value={value}
        />
    }
}
