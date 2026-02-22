use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn StackedShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("min-h-screen flex flex-col", &[], class);
    view! { <div class=cls>{children()}</div> }
}
