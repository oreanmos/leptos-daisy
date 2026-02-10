//! Mockup Phone component — daisyUI `mockup-phone`.
//!
//! Phone frame mockup for showcasing mobile designs or screenshots.
//! Provides a realistic phone frame with camera notch and borders.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI phone mockup component.
///
/// Creates a phone frame mockup for showcasing mobile designs.
/// The children are rendered inside the phone screen area.
///
/// # Props
/// - `class`: Additional CSS classes
/// - `children`: Content to display inside the phone screen
#[component]
pub fn MockupPhone(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mockup-phone",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            <div class="mockup-phone-camera"></div>
            <div class="mockup-phone-display">
                {children()}
            </div>
        </div>
    }
}
