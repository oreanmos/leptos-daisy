//! Alert component — daisyUI `alert`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}
impl AlertVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Info => "alert-info",
            Self::Success => "alert-success",
            Self::Warning => "alert-warning",
            Self::Error => "alert-error",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertStyle {
    #[default]
    Default,
    Outline,
    Dash,
    Soft,
}
impl AlertStyle {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Outline => "alert-outline",
            Self::Dash => "alert-dash",
            Self::Soft => "alert-soft",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDirection {
    #[default]
    Default,
    Vertical,
    Horizontal,
}
impl AlertDirection {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Vertical => "alert-vertical",
            Self::Horizontal => "alert-horizontal",
        }
    }
}

/// A daisyUI alert component.
#[component]
pub fn Alert(
    children: Children,
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional)] style: AlertStyle,
    #[prop(optional)] direction: AlertDirection,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [variant.cls(), style.cls(), direction.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let cls = class_signal("alert", &mods, class);
    view! { <div class=cls role="alert">{children()}</div> }
}

/// Alert icon sub-component.
#[component]
pub fn AlertIcon(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("alert-icon", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Alert title sub-component.
#[component]
pub fn AlertTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("alert-title", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Alert content sub-component.
#[component]
pub fn AlertContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("alert-content", &[], class);
    view! { <div class=cls>{children()}</div> }
}

/// Alert actions sub-component.
#[component]
pub fn AlertActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("alert-actions", &[], class);
    view! { <div class=cls>{children()}</div> }
}
