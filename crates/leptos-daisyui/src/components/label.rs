//! Label component — daisyUI `label`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI label for form inputs with optional floating style.
#[component]
pub fn Label(
    children: Children,
    #[prop(optional)] floating: bool,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    if floating {
        m.push("floating-label".into());
    }

    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "label",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label class={cls} for={for_id}>
            {children()}
        </label>
    }
}
