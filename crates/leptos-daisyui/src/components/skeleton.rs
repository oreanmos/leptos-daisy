//! Skeleton component — daisyUI `skeleton`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] width: Option<String>,
    #[prop(optional, into)] height: Option<String>,
    #[prop(optional)] circle: bool,
    #[prop(optional)] wave: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    if circle {
        m.push("rounded-full".into());
    }
    if wave {
        m.push("skeleton-wave".into());
    }
    if let Some(w) = &width {
        m.push(format!("w-{}", w));
    }
    if let Some(h) = &height {
        m.push(format!("h-{}", h));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("skeleton", &refs, class);
    view! { <div class=cls aria-busy="true"></div> }.add_any_attr(attrs)
}
