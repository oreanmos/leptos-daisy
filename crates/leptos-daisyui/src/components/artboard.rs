//! Artboard component — daisyUI `artboard`.
//!
//! A device frame mockup for showcasing designs.
//! Provides phone-sized frames with various size options.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// Phone size variants for the artboard.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArtboardSize {
    /// Phone size 1 (smallest)
    Phone1,
    /// Phone size 2
    Phone2,
    /// Phone size 3
    Phone3,
    /// Phone size 4
    Phone4,
    /// Phone size 5
    Phone5,
    /// Phone size 6 (largest) - default
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

/// A daisyUI artboard component.
///
/// Creates a device frame mockup, typically used for showcasing
/// mobile designs or screenshots.
///
/// # Props
/// - `size`: Phone frame size (phone-1 through phone-6)
/// - `horizontal`: Use horizontal orientation
/// - `demo`: Use demo mode with grid background
/// - `class`: Additional CSS classes
/// - `children`: Content to display inside the artboard
#[component]
pub fn Artboard(
    children: Children,
    #[prop(optional)] size: ArtboardSize,
    #[prop(optional)] horizontal: bool,
    #[prop(optional)] demo: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    m.push(size.cls());
    if horizontal {
        m.push("artboard-horizontal");
    }
    if demo {
        m.push("artboard-demo");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "artboard",
        &m,
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
