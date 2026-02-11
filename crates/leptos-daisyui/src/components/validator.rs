//! Validator component — daisyUI `validator`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Validator(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("validator", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ValidatorLabel(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("validator-label", &[], class);
    view! { <span class=cls>{children()}</span> }
}
