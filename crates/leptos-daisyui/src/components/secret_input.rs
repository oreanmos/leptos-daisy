//! SecretInput component — password input with visibility toggle.

use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

fn get_input_classes(color: Option<Color>, size: Option<Size>) -> Vec<String> {
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
    m
}

fn get_btn_classes(size: Option<Size>) -> Vec<String> {
    let mut m = vec!["btn-square".to_string()];
    if let Some(s) = size {
        m.push(s.class("btn"));
    }
    m
}

/// A password input with a toggle button to show/hide the value.
///
/// Renders a `join` group containing a text input and a toggle button
/// with eye/eye-off icons.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <SecretInput
///         placeholder="Enter password"
///         on_input=move |ev: ev::Event| { /* extract value */ }
///     />
/// }
/// ```
#[component]
pub fn SecretInput(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] on_input: Option<Callback<ev::Event>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let visible = RwSignal::new(false);

    let input_mods = get_input_classes(color, size);
    let mut input_cls_parts = vec!["input".to_string(), "input-bordered".to_string(), "join-item".to_string(), "flex-1".to_string()];
    input_cls_parts.extend(input_mods);
    let input_base: String = input_cls_parts.join(" ");

    let btn_mods = get_btn_classes(size);
    let mut btn_cls_parts = vec!["btn".to_string(), "join-item".to_string()];
    btn_cls_parts.extend(btn_mods);
    let btn_cls: String = btn_cls_parts.join(" ");

    let handle_input = move |ev: ev::Event| {
        if let Some(cb) = on_input {
            cb.run(ev);
        }
    };

    let input_cls = move || {
        let base = &input_base;
        match class.get() {
            Some(extra) => format!("{base} {extra}"),
            None => base.clone(),
        }
    };

    view! {
        <div class="join w-full">
            {move || {
                let input_type = if visible.get() { "text" } else { "password" };
                view! {
                    <input
                        type=input_type
                        class=input_cls.clone()
                        placeholder=move || placeholder.get().unwrap_or_default()
                        name=move || name.get()
                        disabled=move || disabled.get().unwrap_or(false)
                        prop:value=move || value.get().unwrap_or_default()
                        on:input=handle_input
                    />
                }
            }}
            <button
                type="button"
                class=btn_cls
                on:click=move |_| visible.update(|v| *v = !*v)
            >
                {move || if visible.get() {
                    // Eye-slash icon (visible — click to hide)
                    view! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88" />
                        </svg>
                    }.into_any()
                } else {
                    // Eye icon (hidden — click to show)
                    view! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    }.into_any()
                }}
            </button>
        </div>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_classes_default() {
        let classes = get_input_classes(None, None);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_input_classes_color_size() {
        let classes = get_input_classes(Some(Color::Primary), Some(Size::Small));
        assert_eq!(classes, vec!["input-primary", "input-sm"]);
    }

    #[test]
    fn test_btn_classes_default() {
        let classes = get_btn_classes(None);
        assert_eq!(classes, vec!["btn-square"]);
    }

    #[test]
    fn test_btn_classes_size() {
        let classes = get_btn_classes(Some(Size::Large));
        assert_eq!(classes, vec!["btn-square", "btn-lg"]);
    }
}
