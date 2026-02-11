//! Stack component — daisyUI `stack`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Stack(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("stack", &[], class);
    view! { <div class=cls>{children()}</div> }
}
