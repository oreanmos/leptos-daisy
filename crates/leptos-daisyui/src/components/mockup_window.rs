//! Mockup Window component — daisyUI `mockup-window`.
//!
//! OS window mockup for showcasing desktop applications or content.
//! Provides a realistic window frame with title bar and control buttons.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI window mockup component.
///
/// Creates an OS window frame mockup with title bar and control buttons.
/// Useful for showcasing desktop applications or screenshots.
///
/// # Props
/// - `title`: Optional window title to display in the title bar
/// - `class`: Additional CSS classes
/// - `children`: Content to display inside the window frame
#[component]
pub fn MockupWindow(
    children: Children,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mockup-window",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            <div class="mockup-window-toolbar">
                {title.map(|t| view! { <div class="mockup-window-title">{t}</div> })}
            </div>
            <div class="mockup-window-content">
                {children()}
            </div>
        </div>
    }
}
