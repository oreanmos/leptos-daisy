//! IconButton component — a circular/square button for icon-only actions.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

fn get_icon_button_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
    square: bool,
) -> Vec<String> {
    let mut mods = Vec::new();
    // Circle by default, square opt-in
    if square {
        mods.push("btn-square".into());
    } else {
        mods.push("btn-circle".into());
    }
    if let Some(c) = color {
        let s = c.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
    } else {
        // Default to ghost if no color specified
        mods.push("btn-ghost".into());
    }
    if let Some(s) = size {
        mods.push(s.class("btn"));
    }
    if let Some(v) = variant {
        let s = v.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    mods
}

/// A button optimized for icon-only actions (circular by default).
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <IconButton aria_label="Delete" color=Color::Error on_click=move |_| { /* ... */ }>
///         <TrashIcon />
///     </IconButton>
/// }
/// ```
#[component]
pub fn IconButton(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    /// Render as square instead of circle.
    #[prop(optional)]
    square: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mods = get_icon_button_classes(color, size, variant, square);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btn", &refs, class);

    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    view! {
        <button
            type="button"
            class=cls
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            on:click=handle_click
        >
            {children()}
        </button>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_button_classes_default() {
        let classes = get_icon_button_classes(None, None, None, false);
        assert_eq!(classes, vec!["btn-circle", "btn-ghost"]);
    }

    #[test]
    fn test_icon_button_classes_square() {
        let classes = get_icon_button_classes(None, None, None, true);
        assert_eq!(classes, vec!["btn-square", "btn-ghost"]);
    }

    #[test]
    fn test_icon_button_classes_with_color() {
        let classes = get_icon_button_classes(Some(Color::Error), Some(Size::Small), None, false);
        assert_eq!(classes, vec!["btn-circle", "btn-error", "btn-sm"]);
    }
}
