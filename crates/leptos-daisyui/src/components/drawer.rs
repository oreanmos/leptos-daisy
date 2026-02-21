//! Drawer component — daisyUI `drawer`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DrawerPosition {
    #[default]
    Start,
    End,
}

impl DrawerPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "",
            Self::End => "drawer-end",
        }
    }
}

#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional)] position: Option<DrawerPosition>,
    #[prop(into)] id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if let Some(pos) = position {
        let s = pos.as_str();
        if !s.is_empty() {
            m.push(s);
        }
    }
    let cls = class_signal("drawer", &m, class);
    view! {
        <div class=cls>
            <input id=id.clone() type="checkbox" class="drawer-toggle" />
            {children()}
        </div>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("drawer-content", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn DrawerSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("drawer-side", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn DrawerOverlay(
    #[prop(into)] drawer_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("drawer-overlay", &[], class);
    view! { <label for=drawer_id class=cls aria-hidden="true"></label> }.add_any_attr(attrs)
}
