//! Drawer component — daisyUI `drawer`.
//!
//! A side drawer navigation that slides in from the left or right.
//! Uses a checkbox hack for open/close state (SSR-safe).

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI drawer component.
///
/// The drawer consists of a content area and a side panel that slides in.
/// The open/close state is controlled via a checkbox input (SSR-safe).
///
/// # Props
/// - `end`: If true, drawer slides in from the right side
/// - `id`: Unique identifier for the toggle control checkbox
/// - `class`: Additional CSS classes
/// - `children`: Child content (should include DrawerContent and DrawerSide)
#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional)] end: bool,
    #[prop(into)] id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if end {
        m.push("drawer-end");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            <input id={id.clone()} type="checkbox" class="drawer-toggle" />
            {children()}
        </div>
    }
}

/// The main content area of a drawer.
///
/// This is the area that remains visible when the drawer is closed.
#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-content",
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

/// The side panel of a drawer.
///
/// This is the sliding panel that appears when the drawer is open.
/// Contains the drawer overlay and the actual side content.
#[component]
pub fn DrawerSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-side",
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

/// The overlay element for a drawer.
///
/// Clicking the overlay closes the drawer. This is a label that targets
/// the drawer's checkbox input.
#[component]
pub fn DrawerOverlay(
    #[prop(into)] drawer_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "drawer-overlay",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label for={drawer_id} class={cls} aria-hidden="true"></label>
    }
}
