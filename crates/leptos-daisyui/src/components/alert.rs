//! Alert component — daisyUI `alert`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

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

fn get_alert_classes(
    color: Option<Color>,
    style: AlertStyle,
    direction: AlertDirection,
) -> Vec<String> {
    let mut mods = Vec::new();
    if let Some(c) = color {
        let s = c.class("alert");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    let style_cls = style.cls();
    if !style_cls.is_empty() {
        mods.push(style_cls.to_string());
    }
    let dir_cls = direction.cls();
    if !dir_cls.is_empty() {
        mods.push(dir_cls.to_string());
    }
    mods
}

/// A daisyUI alert component.
#[component]
pub fn Alert(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional)] style: AlertStyle,
    #[prop(optional)] direction: AlertDirection,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mods = get_alert_classes(color, style, direction);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("alert", &refs, class);
    view! { <div class=cls role="alert">{children()}</div> }.add_any_attr(attrs)
}

/// Alert icon sub-component.
#[component]
pub fn AlertIcon(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("alert-icon", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

/// Alert title sub-component.
#[component]
pub fn AlertTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("alert-title", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

/// Alert content sub-component.
#[component]
pub fn AlertContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("alert-content", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

/// Alert actions sub-component.
#[component]
pub fn AlertActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("alert-actions", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_style_classes() {
        assert_eq!(AlertStyle::Default.cls(), "");
        assert_eq!(AlertStyle::Outline.cls(), "alert-outline");
        assert_eq!(AlertStyle::Dash.cls(), "alert-dash");
        assert_eq!(AlertStyle::Soft.cls(), "alert-soft");
    }

    #[test]
    fn test_alert_direction_classes() {
        assert_eq!(AlertDirection::Default.cls(), "");
        assert_eq!(AlertDirection::Vertical.cls(), "alert-vertical");
        assert_eq!(AlertDirection::Horizontal.cls(), "alert-horizontal");
    }

    #[test]
    fn test_get_alert_classes() {
        // Default
        let classes = get_alert_classes(None, AlertStyle::Default, AlertDirection::Default);
        assert!(classes.is_empty());

        // Color only
        let classes = get_alert_classes(
            Some(Color::Info),
            AlertStyle::Default,
            AlertDirection::Default,
        );
        assert_eq!(classes, vec!["alert-info"]);

        // Style only
        let classes = get_alert_classes(None, AlertStyle::Outline, AlertDirection::Default);
        assert_eq!(classes, vec!["alert-outline"]);

        // Direction only
        let classes = get_alert_classes(None, AlertStyle::Default, AlertDirection::Horizontal);
        assert_eq!(classes, vec!["alert-horizontal"]);

        // Mixed
        let classes = get_alert_classes(
            Some(Color::Success),
            AlertStyle::Dash,
            AlertDirection::Vertical,
        );
        assert_eq!(
            classes,
            vec!["alert-success", "alert-dash", "alert-vertical"]
        );
    }
}
