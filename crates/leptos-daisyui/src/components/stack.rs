//! Stack component — daisyUI `stack`.
//!
//! A CSS grid layout for stacking elements (typically images).
//! Creates an overlapping stack effect.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A daisyUI stack component.
///
/// Creates a stacked layout where children overlap each other.
/// Commonly used for image stacks or card overlays.
///
/// # Props
/// - `class`: Additional CSS classes
/// - `children`: Elements to stack (typically images)
#[component]
pub fn Stack(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "stack",
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
        </div>
    }
}
