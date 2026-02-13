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
            Self::Bordered => "tabs-bordered",
            Self::Lifted => "tabs-lifted",
            Self::Boxed => "tabs-boxed",
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
    let mut m: Vec<String> = Vec::new();
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc.into());
    }
    if let Some(s) = size {
        m.push(s.class("tabs"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("tabs", &refs, class);
    view! { <div class=cls role="tablist">{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn Tab(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if active {
        m.push("tab-active");
    }
    if disabled {
        m.push("tab-disabled");
    }
    let cls = class_signal("tab", &m, class);
    view! {
        <button type="button" role="tab" class=cls
            aria-selected=active
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
