//! Tooltip component — daisyUI `tooltip`.
//!
//! A hover tooltip that displays additional information when hovering over an element.
//!
//! # Props
//! - `text`: The tooltip content text
//! - `position`: Tooltip position (Top, Bottom, Left, Right)
//! - `color`: Optional color variant for the tooltip
//! - `open`: If true, tooltip is always visible
//! - `class`: Additional CSS classes
//! - `children`: The element that triggers the tooltip on hover

use crate::utils::class::build_class;
use crate::variants::color::Color;
use leptos::prelude::*;

/// Tooltip position variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPosition {
    fn cls(&self) -> &'static str {
        match self {
            Self::Top => "tooltip-top",
            Self::Bottom => "tooltip-bottom",
            Self::Left => "tooltip-left",
            Self::Right => "tooltip-right",
        }
    }
}

/// A daisyUI tooltip component.
///
/// Wraps children with a tooltip container that displays text on hover.
#[component]
pub fn Tooltip(
    children: Children,
    #[prop(into)] text: String,
    #[prop(optional)] position: TooltipPosition,
    #[prop(optional)] color: Option<Color>,
    #[prop(optional)] open: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec![position.cls()];

    if open {
        m.push("tooltip-open");
    }

    // Add color class if specified
    let color_class = color.map(|c| c.class("tooltip"));
    if let Some(ref cc) = color_class
        && !cc.is_empty()
    {
        m.push(cc.as_str());
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "tooltip",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls} data-tip={text}>
            {children()}
        </div>
    }
}
