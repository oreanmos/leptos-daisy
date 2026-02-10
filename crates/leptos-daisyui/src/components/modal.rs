//! Modal component — daisyUI `modal`.
use crate::utils::class::build_class;
use leptos::prelude::*;

#[component]
pub fn Modal(
    children: Children,
    #[prop(optional)] open: bool,
    #[prop(optional)] bottom: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if bottom {
        m.push("modal-bottom");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "modal",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <dialog class={cls} open={open}>{children()}</dialog> }
}

#[component]
pub fn ModalBox(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "modal-box",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn ModalActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "modal-action",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn ModalTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "text-lg font-bold",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <h3 class={cls}>{children()}</h3> }
}

#[component]
pub fn ModalCloseButton(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <button class={cls} type="button" aria-label="Close">"✕"</button> }
}

#[component]
pub fn ModalBackdrop(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
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
    view! { <form method="dialog" class={cls}><button>"close"</button></form> }
}
