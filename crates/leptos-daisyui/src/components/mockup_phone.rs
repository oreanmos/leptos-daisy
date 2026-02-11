//! MockupPhone component — daisyUI `mockup-phone`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn MockupPhone(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mockup-phone", &[], class);
    view! {
        <div class=cls>
            <div class="camera"></div>
            <div class="display">{children()}</div>
        </div>
    }
}
