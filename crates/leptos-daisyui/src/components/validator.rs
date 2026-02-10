//! Validator component — daisyUI `validator`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Validation state for the validator component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValidatorState {
    #[default]
    None,
    Error,
    Success,
}

impl ValidatorState {
    /// Returns the CSS class suffix for the validator state.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Error => "validator-error",
            Self::Success => "validator-success",
        }
    }
}

/// A daisyUI validator component for form validation styling.
#[component]
pub fn Validator(
    children: Children,
    #[prop(optional)] state: ValidatorState,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    let s = state.as_str();
    if !s.is_empty() {
        m.push(s.into());
    }

    let r: Vec<&str> = m.iter().map(|s: &String| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "validator",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
            {hint.as_ref().map(|h| view! { <div class="validator-hint">{h.clone()}</div> })}
        </div>
    }
}
