//! Diff component — daisyUI `diff` for before/after comparison.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Diff component for before/after image comparison.
#[component]
pub fn Diff(children: Children, #[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "diff",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// First diff item (before image).
#[component]
pub fn DiffItem1(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "diff-item-1",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Second diff item (after image).
#[component]
pub fn DiffItem2(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "diff-item-2",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Diff resizer handle.
#[component]
pub fn DiffResizer(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "diff-resizer",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}></div> }
}
