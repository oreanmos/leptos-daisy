//! Dropdown component — daisyUI `dropdown`.
use crate::utils::class::build_class;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
    Start,
    End,
}
impl DropdownPosition {
    fn cls(&self) -> &'static str {
        match self {
            Self::Bottom => "dropdown-bottom",
            Self::Top => "dropdown-top",
            Self::Left => "dropdown-left",
            Self::Right => "dropdown-right",
            Self::Start => "dropdown-start",
            Self::End => "dropdown-end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownHover {
    #[default]
    Click,
    Hover,
}
impl DropdownHover {
    fn cls(&self) -> &'static str {
        match self {
            Self::Click => "",
            Self::Hover => "dropdown-hover",
        }
    }
}

#[component]
pub fn Dropdown(
    children: Children,
    #[prop(optional)] position: DropdownPosition,
    #[prop(optional)] hover: DropdownHover,
    #[prop(optional)] open: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec![position.cls()];
    let hc = hover.cls();
    if !hc.is_empty() {
        m.push(hc);
    }
    if open {
        m.push("dropdown-open");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "dropdown",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn DropdownTrigger(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = if uc.is_empty() { String::new() } else { uc };
    view! { <div tabindex="0" role="button" class={cls}>{children()}</div> }
}

#[component]
pub fn DropdownContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "dropdown-content",
        &[
            "menu",
            "bg-base-100",
            "rounded-box",
            "z-1",
            "w-52",
            "p-2",
            "shadow",
        ],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <ul tabindex="0" class={cls}>{children()}</ul> }
}

#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if active {
        m.push("active");
    }
    if disabled {
        m.push("disabled");
    }
    let uc = class.get().unwrap_or_default();
    if !uc.is_empty() {
        m.push("");
    }
    let cls = m.join(" ");
    view! { <li class={cls}><a>{children()}</a></li> }
}
