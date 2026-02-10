//! Join component — daisyUI `join`.
//!
//! Groups buttons, inputs, and other elements together with no gap.
//! Replaces the deprecated button-group.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI join component.
///
/// Groups child elements together without gaps between them.
///
/// # Props
/// - `vertical`: Use vertical layout instead of horizontal
/// - `horizontal`: Use horizontal layout (default)
/// - `responsive`: Use responsive layout (horizontal on desktop, vertical on mobile)
/// - `class`: Additional CSS classes
/// - `children`: Child elements to join
#[component]
pub fn Join(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();

    if responsive {
        m.push("join-vertical md:join-horizontal");
    } else if vertical {
        m.push("join-vertical");
    } else if horizontal {
        // Default is horizontal, no extra class needed
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "join",
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

/// Marks a child element as a join item.
///
/// Apply this class to direct children of a Join component.
#[component]
pub fn JoinItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "join-item",
        &[],
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
