//! Filter component — daisyUI `filter`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Filter(
    children: Children,
    #[prop(optional, into)] reset_label: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("filter", &[], class);
    view! {
        <div class=cls>
            {children()}
            {reset_label.as_ref().map(|l| view! { <input type="reset" class="filter-reset" value={l.clone()} /> })}
        </div>
    }.add_any_attr(attrs)
}
