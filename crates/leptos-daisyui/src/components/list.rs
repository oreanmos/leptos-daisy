//! List component — daisyUI `list` + `list-row`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// List container component.
#[component]
pub fn List(
    children: Children,
    #[prop(optional)] row: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    if row {
        mods.push("list-row");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "list",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <ul class={cls}>{children()}</ul> }
}

/// List item component.
#[component]
pub fn ListItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <li class={cls}>{children()}</li> }
}

/// List column that grows to fill space.
#[component]
pub fn ListCol(
    children: Children,
    #[prop(optional)] grow: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    if grow {
        mods.push("list-col-grow");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "list-col",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}
