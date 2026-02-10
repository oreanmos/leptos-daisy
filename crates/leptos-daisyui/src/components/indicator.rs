//! Indicator component — daisyUI `indicator`.
//!
//! A corner indicator/badge that can be placed on any element.
//! Useful for showing notifications, status, or counts.

use crate::utils::class::build_class;
use leptos::prelude::*;
use std::sync::Arc;

/// Horizontal position for the indicator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorHorizontal {
    /// Position at the start (left)
    Start,
    /// Position at the center
    Center,
    /// Position at the end (right) - default
    #[default]
    End,
}

impl IndicatorHorizontal {
    fn cls(&self) -> &'static str {
        match self {
            Self::Start => "indicator-start",
            Self::Center => "indicator-center",
            Self::End => "",
        }
    }
}

/// Vertical position for the indicator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorVertical {
    /// Position at the top - default
    #[default]
    Top,
    /// Position at the middle
    Middle,
    /// Position at the bottom
    Bottom,
}

impl IndicatorVertical {
    fn cls(&self) -> &'static str {
        match self {
            Self::Top => "",
            Self::Middle => "indicator-middle",
            Self::Bottom => "indicator-bottom",
        }
    }
}

/// A daisyUI indicator component.
///
/// Wraps content and displays an indicator item in a corner.
///
/// # Props
/// - `horizontal`: Horizontal position (start, center, end)
/// - `vertical`: Vertical position (top, middle, bottom)
/// - `item`: The indicator content (badge, dot, etc.)
/// - `class`: Additional CSS classes
/// - `children`: The content to wrap with the indicator
#[component]
pub fn Indicator(
    children: Children,
    #[prop(optional)] horizontal: IndicatorHorizontal,
    #[prop(optional)] vertical: IndicatorVertical,
    item: Arc<dyn Fn() -> AnyView + Send + Sync>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let hc = horizontal.cls();
    if !hc.is_empty() {
        m.push(hc);
    }
    let vc = vertical.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "indicator",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
            <span class="indicator-item">{item()}</span>
        </div>
    }
}

/// A simple indicator item component for badges.
///
/// Wraps content with the `indicator-item` class.
#[component]
pub fn IndicatorItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "indicator-item",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <span class={cls}>
            {children()}
        </span>
    }
}
