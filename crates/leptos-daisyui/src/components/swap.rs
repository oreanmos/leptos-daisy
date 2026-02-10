//! Swap component — daisyUI `swap`.
//!
//! Icon/content swap animation for toggling between two states.
//! Can be controlled via checkbox or class-based state.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// Swap animation effect variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapEffect {
    /// No animation effect
    None,
    /// Rotate animation effect
    #[default]
    Rotate,
    /// Flip animation effect
    Flip,
}

impl SwapEffect {
    /// Returns the CSS class for the swap effect.
    pub fn cls(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Rotate => Some("swap-rotate"),
            Self::Flip => Some("swap-flip"),
        }
    }
}

/// A daisyUI swap component for toggling between two icons/content.
///
/// Creates an animated swap between "on" and "off" states.
/// Can be controlled via the `active` prop or used with a hidden checkbox.
///
/// # Props
/// - `active`: Controlled state for the swap (true = on state visible)
/// - `effect`: Animation effect (rotate, flip, or none)
/// - `on_icon`: Content to show when active (first child)
/// - `off_icon`: Content to show when inactive (second child)
/// - `class`: Additional CSS classes
#[component]
pub fn Swap(
    /// Content to show when swap is active (on state)
    on_icon: ChildrenFn,
    /// Content to show when swap is inactive (off state)
    off_icon: ChildrenFn,
    #[prop(optional)] active: bool,
    #[prop(optional)] effect: SwapEffect,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();

    if active {
        m.push("swap-active");
    }

    if let Some(effect_cls) = effect.cls() {
        m.push(effect_cls);
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "swap",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label class={cls}>
            <div class="swap-on">{on_icon()}</div>
            <div class="swap-off">{off_icon()}</div>
        </label>
    }
}

/// A daisyUI swap component with checkbox control.
///
/// Creates an animated swap controlled by a hidden checkbox.
/// Useful for toggle interactions like theme switches.
///
/// # Props
/// - `effect`: Animation effect (rotate, flip, or none)
/// - `on_icon`: Content to show when checked
/// - `off_icon`: Content to show when unchecked
/// - `class`: Additional CSS classes
#[component]
pub fn SwapCheckbox(
    /// Content to show when checked (on state)
    on_icon: ChildrenFn,
    /// Content to show when unchecked (off state)
    off_icon: ChildrenFn,
    #[prop(optional)] effect: SwapEffect,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();

    if let Some(effect_cls) = effect.cls() {
        m.push(effect_cls);
    }

    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "swap",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label class={cls}>
            <input type="checkbox" class="swap-input" />
            <div class="swap-on">{on_icon()}</div>
            <div class="swap-off">{off_icon()}</div>
        </label>
    }
}
