//! Toast component — daisyUI `toast`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Vertical position for toast container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum VerticalPosition {
    /// Position at the top
    Top,
    /// Position at the bottom (default)
    #[default]
    Bottom,
}

impl VerticalPosition {
    /// The class suffix for vertical position.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "toast-top",
            Self::Bottom => "toast-bottom",
        }
    }
}

impl std::fmt::Display for VerticalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Horizontal position for toast container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HorizontalPosition {
    /// Position at the start (left in LTR)
    Start,
    /// Position at the center
    Center,
    /// Position at the end (right in LTR, default)
    #[default]
    End,
}

impl HorizontalPosition {
    /// The class suffix for horizontal position.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "toast-start",
            Self::Center => "toast-center",
            Self::End => "toast-end",
        }
    }
}

impl std::fmt::Display for HorizontalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Toast notifications container component — daisyUI `toast`.
///
/// This is a container for toast notifications. It handles positioning
/// and stacking of toast alerts.
///
/// # Props
/// - `vertical`: Vertical position (top, bottom)
/// - `horizontal`: Horizontal position (start, center, end)
/// - `class`: Additional CSS classes
/// - `children`: Toast notification elements (typically Alert components)
#[component]
pub fn Toast(
    #[prop(optional, into)] vertical: Option<VerticalPosition>,
    #[prop(optional, into)] horizontal: Option<HorizontalPosition>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();

    // Add vertical position
    if let Some(v) = vertical {
        m.push(v.as_str());
    }

    // Add horizontal position
    if let Some(h) = horizontal {
        m.push(h.as_str());
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "toast",
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
