//! Loading component — daisyUI `loading`.
use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::prelude::*;

/// Loading variant types for daisyUI loading indicators.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LoadingVariant {
    /// Default spinner loading indicator
    #[default]
    Spinner,
    /// Dots loading indicator
    Dots,
    /// Ring loading indicator
    Ring,
    /// Ball loading indicator
    Ball,
    /// Bars loading indicator
    Bars,
    /// Infinity loading indicator
    Infinity,
}

impl LoadingVariant {
    /// The class suffix for the loading variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Spinner => "loading-spinner",
            Self::Dots => "loading-dots",
            Self::Ring => "loading-ring",
            Self::Ball => "loading-ball",
            Self::Bars => "loading-bars",
            Self::Infinity => "loading-infinity",
        }
    }
}

impl std::fmt::Display for LoadingVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Loading indicator component — daisyUI `loading`.
///
/// # Props
/// - `variant`: The loading animation style (spinner, dots, ring, ball, bars, infinity)
/// - `size`: Size of the loading indicator (xs, sm, md, lg, xl)
/// - `color`: Color of the loading indicator using text-* colors
/// - `class`: Additional CSS classes
#[component]
pub fn Loading(
    #[prop(optional, into)] variant: Option<LoadingVariant>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();

    // Add variant class (default to spinner if not specified)
    m.push(variant.unwrap_or_default().as_str().to_string());

    // Add size modifier
    if let Some(s) = size {
        m.push(s.class("loading"));
    }

    // Add color as text-* class (loading uses text colors)
    if let Some(c) = color {
        let color_str = c.as_str();
        if !color_str.is_empty() {
            m.push(format!("text-{}", color_str));
        }
    }

    let mods: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "loading",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! { <span class={cls}></span> }
}
