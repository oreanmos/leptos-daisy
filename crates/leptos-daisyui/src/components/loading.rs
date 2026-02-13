//! Loading component — daisyUI `loading`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LoadingVariant {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}
impl LoadingVariant {
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

#[component]
pub fn Loading(
    #[prop(optional, into)] variant: Option<LoadingVariant>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    m.push(variant.unwrap_or_default().as_str().to_string());
    if let Some(s) = size {
        m.push(s.class("loading"));
    }
    if let Some(c) = color {
        let cs = c.as_str();
        if !cs.is_empty() {
            m.push(format!("text-{}", cs));
        }
    }
    let mods: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("loading", &mods, class);
    view! {
        <span class=cls role="status" aria-live="polite"
            aria-label=move || aria_label.get().unwrap_or_else(|| "Loading".to_string())
        ></span>
    }
    .add_any_attr(attrs)
}
