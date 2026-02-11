//! Carousel component — daisyUI `carousel` + modifiers.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselSnap {
    #[default]
    Center,
    Start,
    End,
}
impl CarouselSnap {
    fn cls(&self) -> &'static str {
        match self {
            Self::Center => "carousel-center",
            Self::Start => "carousel-start",
            Self::End => "carousel-end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl CarouselOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Horizontal => "carousel-horizontal",
            Self::Vertical => "carousel-vertical",
        }
    }
}

#[component]
pub fn Carousel(
    children: Children,
    #[prop(optional)] snap: CarouselSnap,
    #[prop(optional)] orientation: CarouselOrientation,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [snap.cls(), orientation.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let cls = class_signal("carousel", &mods, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn CarouselItem(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("carousel-item", &[], class);
    view! { <div id=id class=cls>{children()}</div> }
}
