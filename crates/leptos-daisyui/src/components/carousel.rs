//! Carousel component — daisyUI `carousel` + modifiers.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Carousel snap position.
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

/// Carousel orientation.
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

/// Carousel container component.
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
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "carousel",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Carousel item component.
#[component]
pub fn CarouselItem(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let base_cls = "carousel-item";
    let final_cls = if uc.is_empty() {
        base_cls.to_string()
    } else {
        format!("{} {}", base_cls, uc)
    };
    view! {
        <div id={id} class={final_cls}>
            {children()}
        </div>
    }
}
