//! Input component — daisyUI `input`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] placeholder: Option<String>,
    /// The input id attribute
    #[prop(optional, into)]
    id: Option<String>,
    /// The input type attribute
    #[prop(optional, into)]
    input_type: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
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
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "input",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    view! { <input type={input_type} id={id} class={cls} placeholder={placeholder} disabled={disabled} /> }
}
