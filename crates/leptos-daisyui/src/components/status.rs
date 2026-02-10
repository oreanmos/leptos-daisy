//! Status component — daisyUI `status` for status indicators.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Status size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl StatusSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::ExtraSmall => "status-xs",
            Self::Small => "status-sm",
            Self::Medium => "status-md",
            Self::Large => "status-lg",
            Self::ExtraLarge => "status-xl",
        }
    }
}

/// Status color/state variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}

impl StatusColor {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "status-primary",
            Self::Secondary => "status-secondary",
            Self::Accent => "status-accent",
            Self::Neutral => "status-neutral",
            Self::Info => "status-info",
            Self::Success => "status-success",
            Self::Warning => "status-warning",
            Self::Error => "status-error",
        }
    }
}

/// Status indicator component.
#[component]
pub fn Status(
    #[prop(optional)] size: StatusSize,
    #[prop(optional)] color: StatusColor,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [size.cls(), color.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "status",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls} aria-hidden="true"></div> }
}
