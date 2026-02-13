//! MockupWindow component — daisyUI `mockup-window`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn MockupWindow(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("mockup-window", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}
