use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn PageHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "border-b border-base-300 bg-base-100 px-4 py-4 sm:px-6 lg:px-8",
        &[],
        class,
    );
    view! { <header class=cls>{children()}</header> }
}

#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("text-2xl font-bold leading-7", &[], class);
    view! { <h1 class=cls>{children()}</h1> }
}

#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex items-center gap-2", &[], class);
    view! { <div class=cls>{children()}</div> }
}
