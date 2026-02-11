//! List component — daisyUI `list`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

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
    let cls = class_signal("list", &mods, class);
    view! { <ul class=cls>{children()}</ul> }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! { <li class=cls>{children()}</li> }
}

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
    let cls = class_signal("list-col", &mods, class);
    view! { <div class=cls>{children()}</div> }
}
