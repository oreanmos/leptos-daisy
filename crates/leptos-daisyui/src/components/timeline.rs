//! Timeline component — daisyUI `timeline`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Timeline(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] responsive: bool,
    #[prop(optional)] compact: bool,
    #[prop(optional)] snap: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if responsive {
        m.push("timeline-vertical md:timeline-horizontal");
    } else if vertical {
        m.push("timeline-vertical");
    } else if horizontal {
        m.push("timeline-horizontal");
    }
    if compact {
        m.push("timeline-compact");
    }
    if snap {
        m.push("timeline-snap-icon");
    }
    let cls = class_signal("timeline", &m, class);
    view! { <ul class=cls>{children()}</ul> }
}

#[component]
pub fn TimelineItem(
    children: Children,
    #[prop(optional)] connect_start: bool,
    #[prop(optional)] connect_end: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! {
        <li class=cls>
            {if connect_start { Some(view! { <hr /> }) } else { None }}
            {children()}
            {if connect_end { Some(view! { <hr /> }) } else { None }}
        </li>
    }
}

#[component]
pub fn TimelineStart(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("timeline-start", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn TimelineMiddle(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("timeline-middle", &[], class);
    view! { <div class=cls>{children.map(|c| c())}</div> }
}

#[component]
pub fn TimelineEnd(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("timeline-end", &[], class);
    view! { <div class=cls>{children()}</div> }
}
