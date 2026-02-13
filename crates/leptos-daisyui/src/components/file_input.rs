//! FileInput component — daisyUI `file-input`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn FileInput(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] accept: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] ghost: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] required: MaybeProp<bool>,
    #[prop(optional, into)] multiple: MaybeProp<bool>,
    #[prop(optional, into)] on_change: Option<Callback<ev::Event>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("file-input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("file-input"));
    }
    if bordered {
        m.push("file-input-bordered".into());
    }
    if ghost {
        m.push("file-input-ghost".into());
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("file-input", &r, class);

    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <input
            type="file"
            id=move || id.get()
            name=move || name.get()
            class=cls
            accept=move || accept.get()
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().unwrap_or(false)
            required=move || required.get().unwrap_or(false)
            multiple=move || multiple.get().unwrap_or(false)
            on:change=handle_change
        />
    }
    .add_any_attr(attrs)
}
