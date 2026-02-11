//! Diff component — daisyUI `diff` for before/after comparison.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Diff(children: Children, #[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let cls = class_signal("diff", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DiffItem1(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("diff-item-1", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DiffItem2(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("diff-item-2", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DiffResizer(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let cls = class_signal("diff-resizer", &[], class);
    view! { <div class=cls></div> }
}
