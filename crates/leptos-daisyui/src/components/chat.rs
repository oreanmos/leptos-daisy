//! Chat component — daisyUI `chat` + `chat-bubble`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChatPosition {
    #[default]
    Start,
    End,
}
impl ChatPosition {
    fn cls(&self) -> &'static str {
        match self {
            Self::Start => "chat-start",
            Self::End => "chat-end",
        }
    }
}

#[component]
pub fn Chat(
    children: Children,
    #[prop(optional)] position: ChatPosition,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mods: Vec<&str> = [position.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let cls = class_signal("chat", &mods, class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ChatImage(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("chat-image", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ChatHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("chat-header", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ChatBubble(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut mods = Vec::new();
    if let Some(c) = color {
        let s = c.class("chat-bubble");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("chat-bubble", &refs, class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ChatFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("chat-footer", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}
