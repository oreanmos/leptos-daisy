//! Filter component — daisyUI `filter`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI filter component for radio button groups with reset functionality.
#[component]
pub fn Filter(
    children: Children,
    #[prop(optional, into)] reset_label: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "filter",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
            {reset_label.as_ref().map(|l| view! { <input type="reset" class="filter-reset" value={l.clone()} /> })}
        </div>
    }
}
