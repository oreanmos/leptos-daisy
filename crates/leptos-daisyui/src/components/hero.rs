//! Hero component — daisyUI `hero`.
//!
//! A hero banner section with background image support and overlay.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI hero component.
///
/// Creates a full-width hero section, typically used at the top of a page.
///
/// # Props
/// - `overlay`: Add an overlay to the hero background
/// - `center`: Center the content both horizontally and vertically
/// - `min_height`: Custom minimum height CSS value (e.g., "500px", "50vh")
/// - `class`: Additional CSS classes
/// - `children`: Hero content
#[component]
pub fn Hero(
    children: Children,
    #[prop(optional)] overlay: bool,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] min_height: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if overlay {
        m.push("hero-overlay");
    }
    if center {
        m.push("place-items-center");
    }

    // Default min-height
    let base = "hero min-h-screen";
    let base_with_height = if let Some(height) = min_height {
        format!("hero min-h-[{}]", height)
    } else {
        base.to_string()
    };

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        &base_with_height,
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
        </div>
    }
}

/// The content container within a hero.
///
/// Centers content and provides text alignment.
#[component]
pub fn HeroContent(
    children: Children,
    #[prop(optional)] center: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if center {
        m.push("text-center");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "hero-content",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
        </div>
    }
}
