//! Progress component — daisyUI `progress`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] value: MaybeProp<f64>,
    #[prop(optional, into)] max: MaybeProp<f64>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("progress");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("progress", &refs, class);
    view! {
        <progress
            class=cls
            value=move || value.get().map(|v| v.to_string())
            max=move || max.get().unwrap_or(100.0).to_string()
        ></progress>
    }
    .add_any_attr(attrs)
}
