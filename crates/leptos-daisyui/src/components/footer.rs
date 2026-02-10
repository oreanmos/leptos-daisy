//! Footer component — daisyUI `footer`.
//!
//! A page footer with various layout options.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI footer component.
///
/// # Props
/// - `center`: Center the footer content
/// - `horizontal`: Use horizontal layout
/// - `vertical`: Use vertical layout
/// - `class`: Additional CSS classes
/// - `children`: Footer content
#[component]
pub fn Footer(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("footer-center");
    }
    if horizontal {
        m.push("footer-horizontal");
    }
    if vertical {
        m.push("footer-vertical");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "footer",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <footer class={cls}>
            {children()}
        </footer>
    }
}

/// A title/heading within a footer section.
#[component]
pub fn FooterTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "footer-title",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <h6 class={cls}>
            {children()}
        </h6>
    }
}
