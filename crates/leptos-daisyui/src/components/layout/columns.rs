use crate::utils::class::{build_class, class_signal};
use leptos::prelude::*;

#[component]
pub fn MultiColumnShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex flex-1", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn MainColumn(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex-1 min-w-0", &[], class);
    view! { <main class=cls>{children()}</main> }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelWidth {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}
impl PanelWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "w-48",
            Self::Small => "w-56",
            Self::Medium => "w-64",
            Self::Large => "w-80",
            Self::ExtraLarge => "w-96",
        }
    }
}

#[component]
pub fn Panel(
    children: Children,
    #[prop(optional)] width: PanelWidth,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "shrink-0 overflow-y-auto border-base-300",
        &[width.cls()],
        class,
    );
    view! { <aside class=cls>{children()}</aside> }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SecondaryColumnWidth {
    Narrow,
    #[default]
    Default,
    Wide,
}
impl SecondaryColumnWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-64",
            Self::Default => "w-80",
            Self::Wide => "w-96",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnVariant {
    #[default]
    Default,
    Light,
    Muted,
    Dark,
}
impl ColumnVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Light => "bg-base-100",
            Self::Muted => "bg-base-200",
            Self::Dark => "bg-neutral",
        }
    }
}

#[component]
pub fn SecondaryColumn(
    children: Children,
    #[prop(optional)] width: SecondaryColumnWidth,
    #[prop(optional)] variant: ColumnVariant,
    #[prop(optional)] bordered: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["flex flex-col overflow-y-auto", width.cls()];
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    if bordered {
        m.push("border-r border-base-300");
    }
    let cls = class_signal("", &m, class);
    view! { <aside class=cls>{children()}</aside> }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelWidth {
    Narrow,
    #[default]
    Default,
    Wide,
}
impl RightPanelWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-64",
            Self::Default => "w-80",
            Self::Wide => "w-96",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Breakpoint {
    Large,
    #[default]
    ExtraLarge,
}
impl Breakpoint {
    fn cls(&self) -> &'static str {
        match self {
            Self::Large => "hidden lg:block",
            Self::ExtraLarge => "hidden xl:block",
        }
    }
}

#[component]
pub fn RightPanel(
    children: Children,
    #[prop(optional)] width: RightPanelWidth,
    #[prop(optional)] breakpoint: Breakpoint,
    #[prop(optional)] fixed: bool,
    #[prop(optional)] variant: ColumnVariant,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec!["overflow-y-auto", width.cls(), breakpoint.cls()];
    if fixed {
        m.push("fixed inset-y-0 right-0");
    }
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    let cls = class_signal("", &m, class);
    view! { <aside class=cls>{children()}</aside> }
}
