//! Stat component — daisyUI `stats` + `stat` + parts.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Stats container component.
#[component]
pub fn Stats(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    if vertical {
        mods.push("stats-vertical");
    } else {
        mods.push("stats-horizontal");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stats",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Individual stat item.
#[component]
pub fn Stat(children: Children, #[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Stat title.
#[component]
pub fn StatTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat-title",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Stat value (the main number/text).
#[component]
pub fn StatValue(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat-value",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Stat description.
#[component]
pub fn StatDesc(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat-desc",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Stat figure (icon/image).
#[component]
pub fn StatFigure(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat-figure",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Stat actions container.
#[component]
pub fn StatActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stat-actions",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}
