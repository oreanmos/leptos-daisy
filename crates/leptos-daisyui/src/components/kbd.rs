//! Kbd component — daisyUI `kbd`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

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
    let cls = class_signal("kbd", &mods, class);
    view! { <kbd class=cls>{children()}</kbd> }
}
