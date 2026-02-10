//! Fieldset component — daisyUI `fieldset`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI fieldset for grouping form elements with an optional legend.
#[component]
pub fn Fieldset(
    children: Children,
    #[prop(optional, into)] legend: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "fieldset",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <fieldset class={cls}>
            {legend.as_ref().map(|l| view! { <legend class="fieldset-legend">{l.clone()}</legend> })}
            {children()}
        </fieldset>
    }
}
