//! Mask component — daisyUI `mask`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskShape {
    #[default]
    Squircle,
    Heart,
    Hexagon,
    Diamond,
    Circle,
    Star,
    Triangle,
    Half1,
    Half2,
    Parallelogram,
    Parallelogram2,
    Parallelogram3,
    Pentagon,
    Hexagon2,
    Square,
    Decagon,
    Diamond2,
    Star2,
    Triangle2,
    Triangle3,
    Triangle4,
    CircleHalf,
    Rectangle,
    Oval,
    Octagon,
    Trapezoid,
    Trapezoid2,
    Arch,
    Arch2,
    Heart2,
}
impl MaskShape {
    pub fn cls(&self) -> &'static str {
        match self {
            Self::Squircle => "mask-squircle",
            Self::Heart => "mask-heart",
            Self::Hexagon => "mask-hexagon",
            Self::Diamond => "mask-diamond",
            Self::Circle => "mask-circle",
            Self::Star => "mask-star",
            Self::Triangle => "mask-triangle",
            Self::Half1 => "mask-half-1",
            Self::Half2 => "mask-half-2",
            Self::Parallelogram => "mask-parallelogram",
            Self::Parallelogram2 => "mask-parallelogram-2",
            Self::Parallelogram3 => "mask-parallelogram-3",
            Self::Pentagon => "mask-pentagon",
            Self::Hexagon2 => "mask-hexagon-2",
            Self::Square => "mask-square",
            Self::Decagon => "mask-decagon",
            Self::Diamond2 => "mask-diamond-2",
            Self::Star2 => "mask-star-2",
            Self::Triangle2 => "mask-triangle-2",
            Self::Triangle3 => "mask-triangle-3",
            Self::Triangle4 => "mask-triangle-4",
            Self::CircleHalf => "mask-circle-half",
            Self::Rectangle => "mask-rectangle",
            Self::Oval => "mask-oval",
            Self::Octagon => "mask-octagon",
            Self::Trapezoid => "mask-trapezoid",
            Self::Trapezoid2 => "mask-trapezoid-2",
            Self::Arch => "mask-arch",
            Self::Arch2 => "mask-arch-2",
            Self::Heart2 => "mask-heart-2",
        }
    }
}

#[component]
pub fn Mask(
    children: Children,
    #[prop(optional)] shape: MaskShape,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mask", &[shape.cls()], class);
    view! { <div class=cls>{children()}</div> }
}
