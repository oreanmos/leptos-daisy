//! Stat component — daisyUI `stat`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Stats(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if responsive {
        m.push("stats-vertical md:stats-horizontal");
    } else if vertical {
        m.push("stats-vertical");
    } else if horizontal {
        m.push("stats-horizontal");
    }
    let cls = class_signal("stats", &m, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn Stat(children: Children, #[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let cls = class_signal("stat", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StatTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stat-title", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StatValue(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stat-value", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StatDesc(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stat-desc", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StatFigure(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stat-figure", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn StatActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stat-actions", &[], class);
    view! { <div class=cls>{children()}</div> }
}
