//! Label component — daisyUI `label`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Label(
    children: Children,
    #[prop(optional)] floating: bool,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if floating {
        m.push("floating-label");
    }
    let cls = class_signal("label", &m, class);
    view! { <label class=cls for=for_id>{children()}</label> }
}
