//! MockupCode component — daisyUI `mockup-code`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn MockupCode(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mockup-code", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn MockupCodeLine(
    children: Children,
    #[prop(optional, into)] prefix: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! { <pre class=cls data-prefix=prefix><code>{children()}</code></pre> }
}
