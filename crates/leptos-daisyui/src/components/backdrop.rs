//! Backdrop component — daisyUI backdrop overlays.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Backdrop(
    #[prop(into)] for_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("modal-backdrop", &[], class);
    view! { <label for=for_id class=cls aria-hidden="true"></label> }
}

#[component]
pub fn DrawerBackdrop(
    #[prop(into)] for_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-overlay", &[], class);
    view! { <label for=for_id class=cls aria-hidden="true"></label> }
}
