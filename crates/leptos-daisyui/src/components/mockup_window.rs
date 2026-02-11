//! MockupWindow component — daisyUI `mockup-window`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn MockupWindow(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mockup-window", &[], class);
    view! { <div class=cls>{children()}</div> }
}
