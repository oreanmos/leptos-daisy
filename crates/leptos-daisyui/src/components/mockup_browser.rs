//! Mockup Browser component — daisyUI `mockup-browser`.
//!
//! Browser window mockup for showcasing web designs or screenshots.
//! Provides a realistic browser frame with toolbar and URL bar.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI browser mockup component.
///
/// Creates a browser window frame mockup with toolbar and URL bar.
/// Useful for showcasing web designs or screenshots.
///
/// # Props
/// - `url`: Optional URL to display in the address bar
/// - `class`: Additional CSS classes
/// - `children`: Content to display inside the browser frame
#[component]
pub fn MockupBrowser(
    children: Children,
    #[prop(optional, into)] url: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mockup-browser",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let url_display = url.unwrap_or_else(|| "https://example.com".to_string());

    view! {
        <div class={cls}>
            <div class="mockup-browser-toolbar">
                <div class="input">{url_display}</div>
            </div>
            {children()}
        </div>
    }
}
