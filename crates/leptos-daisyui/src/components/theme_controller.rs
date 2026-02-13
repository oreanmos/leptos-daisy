//! ThemeController component — daisyUI `theme-controller`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

/// Input type for ThemeController.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeControllerType {
    #[default]
    Checkbox,
    Radio,
}

impl ThemeControllerType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
        }
    }
}

/// A DaisyUI theme controller input for CSS-only theme switching.
#[component]
pub fn ThemeController(
    #[prop(into)] value: String,
    #[prop(optional)] input_type: ThemeControllerType,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
) -> impl IntoView {
    let cls = class_signal("theme-controller", &[], class);
    view! {
        <input
            type=input_type.as_str()
            id=move || id.get()
            name=move || name.get()
            value=value
            class=cls
            aria-label=move || aria_label.get()
            checked=move || checked.get().unwrap_or(false)
            disabled=move || disabled.get().unwrap_or(false)
        />
    }
}
