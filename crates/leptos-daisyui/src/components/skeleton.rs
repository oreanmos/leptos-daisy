//! Skeleton component — daisyUI `skeleton`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Skeleton placeholder component — daisyUI `skeleton`.
///
/// Used as a content placeholder while loading data.
///
/// # Props
/// - `text`: Enable text animation style (`skeleton-text`)
/// - `circle`: Make the skeleton circular (for avatar-style placeholders)
/// - `class`: Additional CSS classes
/// - `children`: Optional child elements
#[component]
pub fn Skeleton(
    #[prop(optional, into)] text: bool,
    #[prop(optional, into)] circle: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();

    if text {
        m.push("skeleton-text");
    }

    if circle {
        m.push("rounded-full");
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "skeleton",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children.map(|c| c())}
        </div>
    }
}
