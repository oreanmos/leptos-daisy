//! Chat component — daisyUI `chat` + `chat-bubble`.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Chat message position.
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

/// Chat bubble color variants.
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

/// Chat container for a single message.
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
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "chat",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Chat image/avatar wrapper.
#[component]
pub fn ChatImage(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "chat-image",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Chat header section.
#[component]
pub fn ChatHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "chat-header",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Chat bubble (message content).
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
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "chat-bubble",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Chat footer section.
#[component]
pub fn ChatFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "chat-footer",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}
