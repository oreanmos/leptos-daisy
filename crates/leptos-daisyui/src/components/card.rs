//! Card component — daisyUI `card` + parts.
use crate::utils::class::build_class;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Default,
    Bordered,
    Compact,
    Normal,
    Side,
}
impl CardVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Bordered => "card-bordered",
            Self::Compact => "card-compact",
            Self::Normal => "card-normal",
            Self::Side => "card-side",
        }
    }
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] variant: CardVariant,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] compact: bool,
    #[prop(optional)] side: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    if bordered {
        m.push("card-bordered");
    }
    if compact {
        m.push("card-compact");
    }
    if side {
        m.push("card-side");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "card",
        &m,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "card-header",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn CardBody(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "card-body",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "card-title",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <h2 class={cls}>{children()}</h2> }
}

#[component]
pub fn CardActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "card-actions",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}
