//! FileInput component — daisyUI `file-input`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::prelude::*;

/// A daisyUI file input component.
#[component]
pub fn FileInput(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] ghost: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("file-input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("file-input"));
    }
    if bordered {
        m.push("file-input-bordered".into());
    }
    if ghost {
        m.push("file-input-ghost".into());
    }

    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "file-input",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! { <input type="file" class={cls} disabled={disabled} /> }
}
