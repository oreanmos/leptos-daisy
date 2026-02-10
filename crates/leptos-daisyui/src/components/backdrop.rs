//! Backdrop component — daisyUI backdrop overlays.
//!
//! Used by Modal and Drawer components to provide a clickable overlay
//! that closes the containing component when clicked.
//!
//! # Props
//! - `for_id`: The ID of the checkbox that controls visibility
//! - `class`: Additional CSS classes

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A generic backdrop overlay component.
///
/// Renders a label element that acts as the backdrop. Clicking it
/// closes the modal/drawer by unchecking the controlling checkbox.
#[component]
pub fn Backdrop(
    #[prop(into)] for_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "modal-backdrop",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label for={for_id} class={cls} aria-hidden="true"></label>
    }
}

/// A drawer-specific overlay component.
///
/// Similar to Backdrop but uses `drawer-overlay` class.
/// Clicking the overlay closes the drawer.
#[component]
pub fn DrawerBackdrop(
    #[prop(into)] for_id: String,
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
        <label for={for_id} class={cls} aria-hidden="true"></label>
    }
}
