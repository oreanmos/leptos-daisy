//! Link component — daisyUI `link`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::prelude::*;

#[component]
pub fn Link(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional)] hover: bool,
    #[prop(optional)] external: bool,
    #[prop(optional)] disabled: bool,
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
    let cls = class_signal("link", &refs, class);

    view! {
        <a
            class=cls
            id=move || id.get()
            href=move || {
                if disabled {
                    None
                } else {
                    Some(href.get().unwrap_or_else(|| "#".to_string()))
                }
            }
            aria-label=move || aria_label.get()
            aria-disabled=disabled
            tabindex=if disabled { -1 } else { 0 }
            target={if external && !disabled { Some("_blank") } else { None }}
            rel={if external && !disabled { Some("noopener noreferrer") } else { None }}
        >
            {children()}
        </a>
    }
}
