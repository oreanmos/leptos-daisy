//! LabelButton component — a `<label>` element styled as a daisyUI button.
//!
//! Useful for drawer toggles and file upload triggers where the `for` attribute
//! on a `<label>` drives the interaction.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

fn get_label_button_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
    square: bool,
    circle: bool,
) -> Vec<String> {
    let mut mods = Vec::new();
    if let Some(c) = color {
        let s = c.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
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
    if square {
        mods.push("btn-square".into());
    }
    if circle {
        mods.push("btn-circle".into());
    }
    mods
}

/// A `<label>` element styled as a daisyUI button.
///
/// Use this for drawer toggles (`<label for="drawer-id">`) and file upload
/// triggers where clicking a label activates an associated input.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <LabelButton for_id="my-drawer" variant=Variant::Ghost square=true>
///         <HamburgerIcon />
///     </LabelButton>
/// }
/// ```
#[component]
pub fn LabelButton(
    #[prop(into)] for_id: String,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mods = get_label_button_classes(color, size, variant, square, circle);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btn", &refs, class);

    view! {
        <label
            class=cls
            for=for_id
            aria-label=move || aria_label.get()
        >
            {children()}
        </label>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_button_classes_default() {
        let classes = get_label_button_classes(None, None, None, false, false);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_label_button_classes_ghost_square() {
        let classes = get_label_button_classes(None, None, Some(Variant::Ghost), true, false);
        assert_eq!(classes, vec!["btn-ghost", "btn-square"]);
    }

    #[test]
    fn test_label_button_classes_full() {
        let classes = get_label_button_classes(
            Some(Color::Primary),
            Some(Size::Small),
            Some(Variant::Outline),
            false,
            true,
        );
        assert_eq!(
            classes,
            vec!["btn-primary", "btn-sm", "btn-outline", "btn-circle"]
        );
    }
}
