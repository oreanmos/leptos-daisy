//! Alert component — daisyUI `alert`.
use crate::utils::class::build_class;
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
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "alert",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls} role="alert">{children()}</div> }
}
