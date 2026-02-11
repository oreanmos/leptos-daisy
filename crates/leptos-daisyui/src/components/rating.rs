//! Rating component — daisyUI `rating`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RatingMask {
    #[default]
    Star,
    Star2,
    Heart,
    HeartFill,
}
impl RatingMask {
    fn cls(&self) -> &'static str {
        match self {
            Self::Star => "mask-star",
            Self::Star2 => "mask-star-2",
            Self::Heart => "mask-heart",
            Self::HeartFill => "mask-heart-fill",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RatingHalf {
    #[default]
    None,
    First,
    Second,
}
impl RatingHalf {
    fn cls(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::First => Some("mask-half-1"),
            Self::Second => Some("mask-half-2"),
        }
    }
}

#[component]
pub fn Rating(
    children: Children,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] half: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(s) = size {
        m.push(s.class("rating"));
    }
    if half {
        m.push("rating-half".into());
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("rating", &refs, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn RatingItem(
    #[prop(into)] name: String,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional)] mask: RatingMask,
    #[prop(optional)] half: RatingHalf,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut modifiers = vec![mask.cls()];
    if let Some(half_cls) = half.cls() {
        modifiers.push(half_cls);
    }
    let cls = class_signal("mask", &modifiers, class);
    view! {
        <input
            type="radio"
            name=name
            value=move || value.get()
            class=cls
            aria-label=move || aria_label.get()
            checked=move || checked.get().unwrap_or(false)
            disabled=move || disabled.get().unwrap_or(false)
        />
    }
}
