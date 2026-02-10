//! Link component — daisyUI `link`.

use crate::utils::class::build_class;
use crate::variants::color::Color;
use leptos::prelude::*;

/// A daisyUI styled link component.
#[component]
pub fn Link(
    children: Children,
    /// The href attribute for the link.
    #[prop(into)]
    href: String,
    /// Optional color variant.
    #[prop(optional, into)]
    color: Option<Color>,
    /// Whether to apply the hover effect.
    #[prop(optional)]
    hover: bool,
    /// Whether this is an external link (adds target="_blank" and rel="noopener noreferrer").
    #[prop(optional)]
    external: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<String> = Vec::new();

    if hover {
        mods.push("link-hover".to_string());
    }

    if let Some(c) = color {
        let s = c.class("link");
        if !s.is_empty() {
            mods.push(s);
        }
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "link",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <a
            class={cls}
            href={href}
            target={if external { Some("_blank") } else { None }}
            rel={if external { Some("noopener noreferrer") } else { None }}
        >
            {children()}
        </a>
    }
}
