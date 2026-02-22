//! Collapse component — daisyUI `collapse` + accordion support.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseTrigger {
    #[default]
    Checkbox,
    Details,
    Radio,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseIcon {
    #[default]
    None,
    Arrow,
    Plus,
}

impl CollapseIcon {
    fn class(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Arrow => "collapse-arrow",
            Self::Plus => "collapse-plus",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseState {
    #[default]
    Auto,
    Open,
    Close,
}

impl CollapseState {
    fn class(self) -> &'static str {
        match self {
            Self::Auto => "",
            Self::Open => "collapse-open",
            Self::Close => "collapse-close",
        }
    }
}

/// Collapse component.
///
/// - `title`: Simple string title.
/// - `header`: Custom title content (takes precedence over `title`). Pass `view! { ... }.into_any()`.
#[component]
pub fn Collapse(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] header: Option<AnyView>,
    #[prop(optional)] trigger: CollapseTrigger,
    #[prop(optional)] icon: CollapseIcon,
    #[prop(optional)] state: CollapseState,
    #[prop(optional, into)] radio_name: MaybeProp<String>,
    #[prop(optional, into)] open: MaybeProp<bool>,
    children: Children,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    let icon_cls = icon.class();
    if !icon_cls.is_empty() {
        mods.push(icon_cls);
    }
    let state_cls = state.class();
    if !state_cls.is_empty() {
        mods.push(state_cls);
    }
    let cls = class_signal("collapse", &mods, class);

    let title_view = if let Some(h) = header {
        Some(h)
    } else {
        title.map(|t| view! { <>{t}</> }.into_any())
    };

    match trigger {
        CollapseTrigger::Details => view! {
            <details class=cls open=move || open.get().unwrap_or(false)>
                {title_view.map(|t| view! { <summary class="collapse-title">{t}</summary> })}
                <div class="collapse-content">{children()}</div>
            </details>
        }
        .into_any(),
        CollapseTrigger::Checkbox => view! {
            <div class=cls>
                <input type="checkbox" checked=move || open.get().unwrap_or(false) />
                {title_view.map(|t| view! { <div class="collapse-title">{t}</div> })}
                <div class="collapse-content">{children()}</div>
            </div>
        }
        .into_any(),
        CollapseTrigger::Radio => view! {
            <div class=cls>
                <input
                    type="radio"
                    name=move || radio_name.get().unwrap_or_else(|| "collapse-radio".to_string())
                    checked=move || open.get().unwrap_or(false)
                />
                {title_view.map(|t| view! { <div class="collapse-title">{t}</div> })}
                <div class="collapse-content">{children()}</div>
            </div>
        }
        .into_any(),
    }
}

#[component]
pub fn CollapseTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("collapse-title", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn CollapseContent(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("collapse-content", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn Accordion(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("join join-vertical w-full", &[], class);
    view! { <div class=cls>{children()}</div> }
}
