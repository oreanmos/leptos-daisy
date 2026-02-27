//! LinkButton component — an `<a>` element styled as a daisyUI button.

use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

fn get_link_button_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
    wide: bool,
    block: bool,
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
    if wide {
        mods.push("btn-wide".into());
    }
    if block {
        mods.push("btn-block".into());
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
    /// Open link in a new tab with `target="_blank"` and `rel="noopener noreferrer"`.
    #[prop(optional)]
    external: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] wide: bool,
    #[prop(optional)] block: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let mods = get_link_button_classes(color, size, variant, wide, block, disabled);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("btn", &refs, class);

    let target = external.then_some("_blank");
    let rel = external.then_some("noopener noreferrer");

    view! {
        <a
            class=cls
            href=href
            target=target
            rel=rel
            aria-label=move || aria_label.get()
            tabindex=disabled.then_some("-1")
            aria-disabled=disabled.then_some("true")
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
        let classes = get_link_button_classes(None, None, None, false, false, false);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_link_button_classes_full() {
        let classes = get_link_button_classes(
            Some(Color::Primary),
            Some(Size::Large),
            Some(Variant::Outline),
            true,
            true,
            true,
        );
        assert_eq!(
            classes,
            vec!["btn-primary", "btn-lg", "btn-outline", "btn-wide", "btn-block", "btn-disabled"]
        );
    }
}
