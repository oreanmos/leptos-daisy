//! Tooltip component — daisyUI `tooltip`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::prelude::*;

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

#[component]
pub fn Tooltip(
    children: Children,
    #[prop(into)] tip: String,
    #[prop(optional)] position: TooltipPosition,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional)] open: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    m.push(position.cls().to_string());
    if let Some(c) = color {
        let s = c.class("tooltip");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if open {
        m.push("tooltip-open".into());
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("tooltip", &refs, class);
    view! { <div class=cls data-tip=tip>{children()}</div> }
}
