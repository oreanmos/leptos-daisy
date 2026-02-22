//! Button component — daisyUI `btn`.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::state::State;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[allow(clippy::too_many_arguments)]
fn get_button_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
    state: Option<State>,
    square: bool,
    circle: bool,
    glass: bool,
    wide: bool,
    block: bool,
    no_animation: bool,
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
    if let Some(st) = state {
        let s = st.class("btn");
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
    if glass {
        mods.push("glass".into());
    }
    if wide {
        mods.push("btn-wide".into());
    }
    if block {
        mods.push("btn-block".into());
    }
    if no_animation {
        mods.push("no-animation".into());
    }
    mods
}

/// A daisyUI button.
#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] state: Option<State>,
    #[prop(optional, into)] button_type: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional)] glass: bool,
    #[prop(optional)] wide: bool,
    #[prop(optional)] block: bool,
    #[prop(optional)] no_animation: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mods = get_button_classes(
        color,
        size,
        variant,
        state,
        square,
        circle,
        glass,
        wide,
        block,
        no_animation,
    );

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btn", &refs, class);

    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    view! {
        <button
            class=cls
            type=move || button_type.get().unwrap_or_else(|| "button".to_string())
            id=move || id.get()
            name=move || name.get()
            value=move || value.get()
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
    fn test_button_classes_default() {
        let classes = get_button_classes(
            None, None, None, None, false, false, false, false, false, false,
        );
        assert!(classes.is_empty());
    }

    #[test]
    fn test_button_classes_color() {
        let classes = get_button_classes(
            Some(Color::Primary),
            None,
            None,
            None,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(classes, vec!["btn-primary"]);
    }

    #[test]
    fn test_button_classes_size() {
        let classes = get_button_classes(
            None,
            Some(Size::Large),
            None,
            None,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(classes, vec!["btn-lg"]);
    }

    #[test]
    fn test_button_classes_variant() {
        let classes = get_button_classes(
            None,
            None,
            Some(Variant::Outline),
            None,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(classes, vec!["btn-outline"]);
    }

    #[test]
    fn test_button_classes_state() {
        let classes = get_button_classes(
            None,
            None,
            None,
            Some(State::Active),
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(classes, vec!["btn-active"]);
    }

    #[test]
    fn test_button_classes_booleans() {
        let classes = get_button_classes(
            None, None, None, None, true, // square
            true, // circle
            true, // glass
            true, // wide
            true, // block
            true, // no_animation
        );
        assert_eq!(
            classes,
            vec![
                "btn-square",
                "btn-circle",
                "glass",
                "btn-wide",
                "btn-block",
                "no-animation"
            ]
        );
    }

    #[test]
    fn test_button_classes_mixed() {
        let classes = get_button_classes(
            Some(Color::Secondary),
            Some(Size::Small),
            Some(Variant::Link),
            Some(State::Disabled),
            true, // square
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(
            classes,
            vec![
                "btn-secondary",
                "btn-sm",
                "btn-link",
                "btn-disabled",
                "btn-square"
            ]
        );
    }
}
