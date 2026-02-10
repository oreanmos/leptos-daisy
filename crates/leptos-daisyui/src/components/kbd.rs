//! Kbd component — daisyUI `kbd` for keyboard keys.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Kbd size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl KbdSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::ExtraSmall => "kbd-xs",
            Self::Small => "kbd-sm",
            Self::Medium => "kbd-md",
            Self::Large => "kbd-lg",
            Self::ExtraLarge => "kbd-xl",
        }
    }
}

/// Keyboard key component.
#[component]
pub fn Kbd(
    children: Children,
    #[prop(optional)] size: KbdSize,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    let size_cls = size.cls();
    if !size_cls.is_empty() {
        mods.push(size_cls);
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "kbd",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <kbd class={cls}>{children()}</kbd> }
}
