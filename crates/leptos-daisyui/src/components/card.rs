//! Card component — daisyUI `card` + parts.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
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
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc.into());
    }
    if bordered {
        m.push("card-bordered".into());
    }
    if compact {
        m.push("card-compact".into());
    }
    if side {
        m.push("card-side".into());
    }
    if let Some(s) = size {
        m.push(s.class("card"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("card", &refs, class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("card-header", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn CardBody(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("card-body", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("card-title", &[], class);
    view! { <h2 class=cls>{children()}</h2> }
}

#[component]
pub fn CardActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("card-actions", &[], class);
    view! { <div class=cls>{children()}</div> }
}
