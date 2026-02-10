//! Timeline component — daisyUI `timeline` + `timeline-item` + parts.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Timeline orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl TimelineOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Vertical => "timeline-vertical",
            Self::Horizontal => "timeline-horizontal",
        }
    }
}

/// Timeline container component.
#[component]
pub fn Timeline(
    children: Children,
    #[prop(optional)] orientation: TimelineOrientation,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [orientation.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "timeline",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <ul class={cls}>{children()}</ul> }
}

/// Timeline item component.
#[component]
pub fn TimelineItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "timeline-item",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <li class={cls}>{children()}</li> }
}

/// Timeline start content (left/top side).
#[component]
pub fn TimelineStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "timeline-start",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Timeline middle (icon/dot).
#[component]
pub fn TimelineMiddle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "timeline-middle",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Timeline end content (right/bottom side).
#[component]
pub fn TimelineEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "timeline-end",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Timeline box (content container).
#[component]
pub fn TimelineBox(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], None);
    let final_cls = format!("{} {}", cls, uc);
    view! { <div class={final_cls}>{children()}</div> }
}
