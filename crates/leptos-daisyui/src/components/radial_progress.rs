//! RadialProgress component — daisyUI `radial-progress` with CSS variables.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Radial progress component (circular progress indicator).
/// Uses CSS `--value`, `--size`, and `--thickness` variables.
#[component]
pub fn RadialProgress(
    /// Progress value (0-100)
    #[prop(into)]
    value: u8,
    /// CSS size value (e.g., "12rem", "100px")
    #[prop(optional, into)]
    size: Option<String>,
    /// CSS thickness value (e.g., "4px", "0.5rem")
    #[prop(optional, into)]
    thickness: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "radial-progress",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let mut style = format!("--value:{};", value);
    if let Some(s) = size {
        style.push_str(&format!("--size:{};", s));
    }
    if let Some(t) = thickness {
        style.push_str(&format!("--thickness:{};", t));
    }

    view! { <div class={cls} style={style} role="progressbar">{value}"%"</div> }
}
