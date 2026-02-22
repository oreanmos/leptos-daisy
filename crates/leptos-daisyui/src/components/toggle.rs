//! Toggle component — daisyUI `toggle`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Toggle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let modifiers = get_toggle_modifiers(color, size, variant);
    let refs: Vec<&str> = modifiers.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("toggle", &refs, class);

    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <input
            type="checkbox"
            id=move || id.get()
            name=move || name.get()
            value=move || value.get()
            class=cls
            aria-label=move || aria_label.get()
            checked=move || checked.get().unwrap_or(false)
            disabled=move || disabled.get().unwrap_or(false)
            on:change=handle_change
        />
    }
    .add_any_attr(attrs)
}

fn get_toggle_modifiers(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
) -> Vec<String> {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("toggle");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("toggle"));
    }
    if let Some(v) = variant {
        let s = v.class("toggle");
        if !s.is_empty() {
            m.push(s);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variants::color::Color;
    use crate::variants::size::Size;
    use crate::variants::variant::Variant;

    #[test]
    fn test_toggle_modifiers_defaults() {
        let modifiers = get_toggle_modifiers(None, None, None);
        assert!(modifiers.is_empty());
    }

    #[test]
    fn test_toggle_modifiers_color() {
        let modifiers = get_toggle_modifiers(Some(Color::Primary), None, None);
        assert_eq!(modifiers, vec!["toggle-primary"]);
    }

    #[test]
    fn test_toggle_modifiers_size() {
        let modifiers = get_toggle_modifiers(None, Some(Size::Large), None);
        assert_eq!(modifiers, vec!["toggle-lg"]);
    }

    #[test]
    fn test_toggle_modifiers_variant() {
        let modifiers = get_toggle_modifiers(None, None, Some(Variant::Outline));
        assert_eq!(modifiers, vec!["toggle-outline"]);
    }

    #[test]
    fn test_toggle_modifiers_combined() {
        let modifiers = get_toggle_modifiers(Some(Color::Secondary), Some(Size::Small), Some(Variant::Ghost));
        // Order matters in implementation: color -> size -> variant
        assert_eq!(modifiers, vec!["toggle-secondary", "toggle-sm", "toggle-ghost"]);
    }
}
