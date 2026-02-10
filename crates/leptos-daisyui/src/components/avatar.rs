//! Avatar component — daisyUI `avatar` + states + groups.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Avatar status indicator position/state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarStatus {
    #[default]
    None,
    Online,
    Offline,
}

impl AvatarStatus {
    fn cls(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Online => "avatar-online",
            Self::Offline => "avatar-offline",
        }
    }
}

/// Avatar placeholder style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarPlaceholder {
    #[default]
    None,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl AvatarPlaceholder {
    fn cls(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Neutral => "avatar-placeholder",
            Self::Primary => "avatar-placeholder bg-primary text-primary-content",
            Self::Secondary => "avatar-placeholder bg-secondary text-secondary-content",
            Self::Accent => "avatar-placeholder bg-accent text-accent-content",
            Self::Info => "avatar-placeholder bg-info text-info-content",
            Self::Success => "avatar-placeholder bg-success text-success-content",
            Self::Warning => "avatar-placeholder bg-warning text-warning-content",
            Self::Error => "avatar-placeholder bg-error text-error-content",
        }
    }
}

/// Avatar shape/size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarShape {
    #[default]
    Default,
    Circle,
    Rounded,
}

impl AvatarShape {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Circle => "rounded-full",
            Self::Rounded => "rounded-lg",
        }
    }
}

/// Avatar size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl AvatarSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::ExtraSmall => "w-6",
            Self::Small => "w-8",
            Self::Medium => "w-12",
            Self::Large => "w-16",
            Self::ExtraLarge => "w-24",
        }
    }
}

#[component]
pub fn Avatar(
    children: Children,
    #[prop(optional)] status: AvatarStatus,
    #[prop(optional)] placeholder: AvatarPlaceholder,
    #[prop(optional)] shape: AvatarShape,
    #[prop(optional)] size: AvatarSize,
    #[prop(optional)] online: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = [status.cls(), placeholder.cls(), shape.cls(), size.cls()]
        .into_iter()
        .flat_map(|s| s.split_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    if online {
        mods.push("avatar-online");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "avatar",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Avatar image wrapper.
#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(optional)] shape: AvatarShape,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let shape_cls = shape.cls();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &[shape_cls],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <img src={src} alt={alt} class={cls} /> }
}

/// Avatar placeholder content (initials or icon).
#[component]
pub fn AvatarPlaceholderContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Avatar group container.
#[component]
pub fn AvatarGroup(
    children: Children,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = vec!["avatar-group"];
    if vertical {
        mods.push("avatar-group-vertical");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}
