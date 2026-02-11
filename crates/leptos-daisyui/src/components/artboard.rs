//! Artboard component — daisyUI `artboard`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

/// Phone size variants for the artboard.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArtboardSize {
    Phone1,
    Phone2,
    Phone3,
    Phone4,
    Phone5,
    #[default]
    Phone6,
}
impl ArtboardSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Phone1 => "artboard-phone-1",
            Self::Phone2 => "artboard-phone-2",
            Self::Phone3 => "artboard-phone-3",
            Self::Phone4 => "artboard-phone-4",
            Self::Phone5 => "artboard-phone-5",
            Self::Phone6 => "artboard-phone-6",
        }
    }
}

#[component]
pub fn Artboard(
    children: Children,
    #[prop(optional)] size: ArtboardSize,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] demo: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec![size.cls()];
    if horizontal {
        m.push("artboard-horizontal");
    }
    if demo {
        m.push("artboard-demo");
    }
    let cls = class_signal("artboard", &m, class);
    view! { <div class=cls>{children()}</div> }
}
