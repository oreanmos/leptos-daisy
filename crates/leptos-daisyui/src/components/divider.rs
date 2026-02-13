//! Divider component — daisyUI `divider`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl DividerOrientation {
    fn cls(&self) -> &'static str {
        match self {
            Self::Horizontal => "divider-horizontal",
            Self::Vertical => "divider-vertical",
        }
    }
}

#[component]
pub fn Divider(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<String> = vec![orientation.cls().to_string()];
    if let Some(c) = color {
        let s = c.class("divider");
        if !s.is_empty() {
            m.push(s);
        }
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("divider", &refs, class);
    view! {
        <div class=cls>
            {text.map(|t| view! { <span>{t}</span> })}
            {children.map(|c| c())}
        </div>
    }
    .add_any_attr(attrs)
}
