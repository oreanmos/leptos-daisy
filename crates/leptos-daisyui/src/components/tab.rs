//! Tab component — daisyUI `tab`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabVariant {
    #[default]
    Default,
    Bordered,
    Lifted,
    Boxed,
}
impl TabVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Bordered => "tabs-border",
            Self::Lifted => "tabs-lift",
            Self::Boxed => "tabs-box",
        }
    }
}

#[component]
pub fn Tabs(
    children: Children,
    #[prop(optional)] variant: TabVariant,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let vc = variant.cls();
    let size_cls = size.map(|s| s.class("tabs"));

    let mut modifiers = Vec::with_capacity(2);
    if !vc.is_empty() {
        modifiers.push(vc);
    }
    if let Some(ref s) = size_cls {
        modifiers.push(s);
    }
    let cls = class_signal("tabs", &modifiers, class);
    view! { <div class=cls role="tablist">{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn Tab(
    children: Children,
    #[prop(optional, into)] active: MaybeProp<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let disabled_cls = if disabled { " tab-disabled" } else { "" };
    let cls = move || {
        let base = if active.get().unwrap_or(false) {
            "tab tab-active"
        } else {
            "tab"
        };
        match class.get() {
            Some(c) if !c.is_empty() => format!("{base}{disabled_cls} {c}"),
            _ => format!("{base}{disabled_cls}"),
        }
    };
    view! {
        <button type="button" role="tab" class=cls
            aria-selected=move || active.get().unwrap_or(false)
            disabled=disabled
        >{children()}</button>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn TabContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("tab-content", &[], class);
    view! { <div class=cls role="tabpanel">{children()}</div> }.add_any_attr(attrs)
}
