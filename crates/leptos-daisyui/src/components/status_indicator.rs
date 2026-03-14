//! StatusIndicator component — a badge with an optional colored dot.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

fn get_status_classes(color: Option<Color>, size: Option<Size>) -> Vec<String> {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("badge");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("badge"));
    }
    m
}

fn dot_color_class(color: Option<Color>) -> &'static str {
    match color {
        Some(Color::Primary) => "bg-primary",
        Some(Color::Secondary) => "bg-secondary",
        Some(Color::Accent) => "bg-accent",
        Some(Color::Info) => "bg-info",
        Some(Color::Success) => "bg-success",
        Some(Color::Warning) => "bg-warning",
        Some(Color::Error) => "bg-error",
        Some(Color::Neutral) => "bg-neutral",
        _ => "bg-base-content/40",
    }
}

/// A badge-style status indicator with an optional colored dot.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <StatusIndicator label="Active" color=Color::Success dot=true />
///     <StatusIndicator label="Maintenance" color=Color::Warning />
/// }
/// ```
#[component]
pub fn StatusIndicator(
    /// The status label text.
    #[prop(into)]
    label: String,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    /// Show a colored dot circle before the label.
    #[prop(optional)]
    dot: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mods = get_status_classes(color, size);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("badge gap-1.5", &refs, class);

    let dot_cls = format!(
        "inline-block w-2 h-2 rounded-full {}",
        dot_color_class(color)
    );

    view! {
        <span class=cls>
            {dot.then(|| view! {
                <span class=dot_cls.clone()></span>
            })}
            {label}
        </span>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_classes_default() {
        let classes = get_status_classes(None, None);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_status_classes_color_size() {
        let classes = get_status_classes(Some(Color::Success), Some(Size::Small));
        assert_eq!(classes, vec!["badge-success", "badge-sm"]);
    }

    #[test]
    fn test_dot_color_class() {
        assert_eq!(dot_color_class(Some(Color::Success)), "bg-success");
        assert_eq!(dot_color_class(Some(Color::Error)), "bg-error");
        assert_eq!(dot_color_class(None), "bg-base-content/40");
    }
}
