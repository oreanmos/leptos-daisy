//! Progress component — daisyUI `progress`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] value: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("progress");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("progress"));
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "progress",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    let max_val = max.unwrap_or(100.0);
    let val_str = value.map(|v| v.to_string());
    let max_str = max_val.to_string();
    view! { <progress class={cls} value={val_str} max={max_str}></progress> }
}
