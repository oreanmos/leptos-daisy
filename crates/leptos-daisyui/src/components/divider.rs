//! Divider component — daisyUI `divider`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl DividerOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Horizontal => "divider-horizontal",
            Self::Vertical => "divider-vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerColor {
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
impl DividerColor {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "divider-primary",
            Self::Secondary => "divider-secondary",
            Self::Accent => "divider-accent",
            Self::Neutral => "divider-neutral",
            Self::Info => "divider-info",
            Self::Success => "divider-success",
            Self::Warning => "divider-warning",
            Self::Error => "divider-error",
        }
    }
}

#[component]
pub fn Divider(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional)] color: DividerColor,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec![orientation.cls()];
    let cc = color.cls();
    if !cc.is_empty() {
        m.push(cc);
    }
    let cls = class_signal("divider", &m, class);
    view! {
        <div class=cls>
            {text.map(|t| view! { <span>{t}</span> })}
            {children.map(|c| c())}
        </div>
    }
}
