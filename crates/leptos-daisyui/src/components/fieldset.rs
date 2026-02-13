//! Fieldset component — daisyUI `fieldset`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Fieldset(
    children: Children,
    #[prop(optional, into)] legend: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("fieldset", &[], class);
    view! {
        <fieldset class=cls>
            {legend.as_ref().map(|l| view! { <legend class="fieldset-legend">{l.clone()}</legend> })}
            {children()}
        </fieldset>
    }.add_any_attr(attrs)
}
