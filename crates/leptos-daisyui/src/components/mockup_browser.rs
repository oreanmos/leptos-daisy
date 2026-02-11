//! MockupBrowser component — daisyUI `mockup-browser`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn MockupBrowser(
    children: Children,
    #[prop(optional, into)] url: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mockup-browser", &[], class);
    view! {
        <div class=cls>
            {url.map(|u| view! { <div class="mockup-browser-toolbar"><div class="input">{u}</div></div> })}
            {children()}
        </div>
    }
}
