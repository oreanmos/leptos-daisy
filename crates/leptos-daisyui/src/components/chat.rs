//! Chat component — daisyUI `chat` + `chat-bubble`.
use crate::utils::class::class_signal;
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChatBubbleColor {
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
impl ChatBubbleColor {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "chat-bubble-primary",
            Self::Secondary => "chat-bubble-secondary",
            Self::Accent => "chat-bubble-accent",
            Self::Neutral => "chat-bubble-neutral",
            Self::Info => "chat-bubble-info",
            Self::Success => "chat-bubble-success",
            Self::Warning => "chat-bubble-warning",
            Self::Error => "chat-bubble-error",
        }
    }
}

#[component]
pub fn Chat(
    children: Children,
    #[prop(optional)] position: ChatPosition,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [position.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let cls = class_signal("chat", &mods, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ChatImage(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("chat-image", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ChatHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("chat-header", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ChatBubble(
    children: Children,
    #[prop(optional)] color: ChatBubbleColor,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mods: Vec<&str> = [color.cls()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let cls = class_signal("chat-bubble", &mods, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ChatFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("chat-footer", &[], class);
    view! { <div class=cls>{children()}</div> }
}
