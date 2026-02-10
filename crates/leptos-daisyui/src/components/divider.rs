//! Divider component — daisyUI `divider`.
//!
//! A visual divider with optional text content.
//! Can be horizontal or vertical.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// Orientation for the divider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal divider (default)
    #[default]
    Horizontal,
    /// Vertical divider
    Vertical,
}

impl DividerOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Horizontal => "divider-horizontal",
            Self::Vertical => "divider-vertical",
        }
    }
}

/// Color variants for the divider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerColor {
    /// Default color
    #[default]
    Default,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Neutral color
    Neutral,
    /// Info color
    Info,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
}

impl DividerColor {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "divider-primary",
            Self::Secondary => "divider-secondary",
            Self::Accent => "divider-accent",
            Self::Neutral => "divider-neutral",
            Self::Info => "divider-info",
            Self::Success => "divider-success",
            Self::Warning => "divider-warning",
            Self::Error => "divider-error",
        }
    }
}

/// A daisyUI divider component.
///
/// # Props
/// - `orientation`: Horizontal or vertical orientation
/// - `color`: Color variant for the divider
/// - `text`: Optional text to display in the divider
/// - `class`: Additional CSS classes
/// - `children`: Optional child content
#[component]
pub fn Divider(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional)] color: DividerColor,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    m.push(orientation.cls());
    let cc = color.cls();
    if !cc.is_empty() {
        m.push(cc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "divider",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {text.map(|t| view! { <span>{t}</span> })}
            {children.map(|c| c())}
        </div>
    }
}
