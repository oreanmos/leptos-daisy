//! LinkButton component — an `<a>` element styled as a daisyUI button.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::state::State;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[allow(clippy::too_many_arguments)]
fn get_link_button_classes(
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
    disabled: bool,
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
    if disabled {
        mods.push("btn-disabled".into());
    }
    mods
}

/// An anchor element styled as a daisyUI button.
///
/// Use this instead of raw `<a class="btn ...">` for consistent styling.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <LinkButton href="/settings" color=Color::Primary>"Settings"</LinkButton>
///     <LinkButton href="https://example.com" external=true>"Visit Site"</LinkButton>
/// }
/// ```
#[component]
pub fn LinkButton(
    #[prop(into)] href: String,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] state: Option<State>,
    /// Open link in a new tab with `target="_blank"` and `rel="noopener noreferrer"`.
    #[prop(optional)]
    external: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional)] glass: bool,
    #[prop(optional)] wide: bool,
    #[prop(optional)] block: bool,
    #[prop(optional)] no_animation: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mods = get_link_button_classes(
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
        disabled,
    );
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btn", &refs, class);

    let target = external.then_some("_blank");
    let rel = external.then_some("noopener noreferrer");

    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    view! {
        <a
            class=cls
            href=href
            target=target
            rel=rel
            aria-label=move || aria_label.get()
            tabindex=disabled.then_some("-1")
            aria-disabled=disabled.then_some("true")
            on:click=handle_click
        >
            {children()}
        </a>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_button_classes_default() {
        let classes = get_link_button_classes(
            None, None, None, None, false, false, false, false, false, false, false,
        );
        assert!(classes.is_empty());
    }

    #[test]
    fn test_link_button_classes_full() {
        let classes = get_link_button_classes(
            Some(Color::Primary),
            Some(Size::Large),
            Some(Variant::Outline),
            Some(State::Active),
            true, // square
            true, // circle
            true, // glass
            true, // wide
            true, // block
            true, // no_animation
            true, // disabled
        );
        assert_eq!(
            classes,
            vec![
                "btn-primary",
                "btn-lg",
                "btn-outline",
                "btn-active",
                "btn-square",
                "btn-circle",
                "glass",
                "btn-wide",
                "btn-block",
                "no-animation",
                "btn-disabled",
            ]
        );
    }

    #[test]
    fn test_link_button_classes_square_ghost() {
        let classes = get_link_button_classes(
            None,
            None,
            Some(Variant::Ghost),
            None,
            true, // square
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert_eq!(classes, vec!["btn-ghost", "btn-square"]);
    }
}
