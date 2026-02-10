//! Mask component — daisyUI `mask`.
//!
//! Image masking shapes for creating decorative image effects.
//! Provides various geometric shapes to mask images.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// Mask shape variants for daisyUI mask component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskShape {
    /// Squircle shape (rounded square)
    #[default]
    Squircle,
    /// Heart shape
    Heart,
    /// Hexagon shape
    Hexagon,
    /// Diamond shape
    Diamond,
    /// Circle shape
    Circle,
    /// Star shape
    Star,
    /// Triangle shape
    Triangle,
    /// Half mask (left side)
    Half1,
    /// Half mask (right side)
    Half2,
    /// Parallelogram shape
    Parallelogram,
    /// Parallelogram variant 2
    Parallelogram2,
    /// Parallelogram variant 3
    Parallelogram3,
    /// Pentagon shape
    Pentagon,
    /// Hexagon variant 2
    Hexagon2,
    /// Square shape
    Square,
    /// Decagon shape (10 sides)
    Decagon,
    /// Diamond variant 2
    Diamond2,
    /// Star variant 2
    Star2,
    /// Triangle variant 2
    Triangle2,
    /// Triangle variant 3
    Triangle3,
    /// Triangle variant 4
    Triangle4,
    /// Circle half
    CircleHalf,
    /// Rectangle shape
    Rectangle,
    /// Oval shape
    Oval,
    /// Octagon shape
    Octagon,
    /// Trapezoid shape
    Trapezoid,
    /// Trapezoid variant 2
    Trapezoid2,
    /// Arch shape
    Arch,
    /// Arch variant 2
    Arch2,
    /// Heart variant 2
    Heart2,
}

impl MaskShape {
    /// Returns the CSS class for the mask shape.
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

/// A daisyUI mask component for applying shape masks to images.
///
/// Creates a masked image with various geometric shapes.
///
/// # Props
/// - `shape`: The mask shape to apply
/// - `class`: Additional CSS classes
/// - `children`: Content to display inside the mask (typically an img)
#[component]
pub fn Mask(
    children: Children,
    #[prop(optional)] shape: MaskShape,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mask",
        &[shape.cls()],
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
