//! Input component — daisyUI `input`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

fn get_input_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
) -> Vec<String> {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("input"));
    }
    if let Some(v) = variant {
        let s = v.class("input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    m
}

#[component]
pub fn Input(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] input_type: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] required: MaybeProp<bool>,
    #[prop(optional, into)] readonly: MaybeProp<bool>,
    #[prop(optional, into)] autofocus: MaybeProp<bool>,
    #[prop(optional, into)] on_input: Option<Callback<ev::Event>>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_keydown: Option<Callback<ev::KeyboardEvent>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let m = get_input_classes(color, size, variant);
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("input", &r, class);

    let handle_input = move |ev: ev::Event| {
        if let Some(cb) = on_input {
            cb.run(ev);
        }
    };
    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };
    let handle_focus = move |ev: ev::FocusEvent| {
        if let Some(cb) = on_focus {
            cb.run(ev);
        }
    };
    let handle_blur = move |ev: ev::FocusEvent| {
        if let Some(cb) = on_blur {
            cb.run(ev);
        }
    };
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if let Some(cb) = on_keydown {
            cb.run(ev);
        }
    };

    view! {
        <input
            type=move || input_type.get().unwrap_or_else(|| "text".to_string())
            id=move || id.get()
            name=move || name.get()
            prop:value=move || value.get().unwrap_or_default()
            class=cls
            placeholder=move || placeholder.get()
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            required=move || required.get().unwrap_or(false)
            readonly=move || readonly.get().unwrap_or(false)
            autofocus=move || autofocus.get().unwrap_or(false)
            on:input=handle_input
            on:change=handle_change
            on:focus=handle_focus
            on:blur=handle_blur
            on:keydown=handle_keydown
        />
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_defaults() {
        let classes = get_input_classes(None, None, None);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_input_color() {
        let classes = get_input_classes(Some(Color::Primary), None, None);
        assert_eq!(classes, vec!["input-primary"]);
    }

    #[test]
    fn test_input_size() {
        let classes = get_input_classes(None, Some(Size::Large), None);
        assert_eq!(classes, vec!["input-lg"]);
    }

    #[test]
    fn test_input_variant() {
        let classes = get_input_classes(None, None, Some(Variant::Ghost));
        assert_eq!(classes, vec!["input-ghost"]);
    }

    #[test]
    fn test_input_combinations() {
        let classes = get_input_classes(
            Some(Color::Secondary),
            Some(Size::Small),
            Some(Variant::Outline),
        );
        assert_eq!(
            classes,
            vec!["input-secondary", "input-sm", "input-outline"]
        );
    }
}
