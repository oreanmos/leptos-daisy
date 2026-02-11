//! Indicator component — daisyUI `indicator`.
use crate::utils::class::class_signal;
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorHorizontal {
    Start,
    Center,
    #[default]
    End,
}
impl IndicatorHorizontal {
    fn cls(&self) -> &'static str {
        match self {
            Self::Start => "indicator-start",
            Self::Center => "indicator-center",
            Self::End => "",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorVertical {
    #[default]
    Top,
    Middle,
    Bottom,
}
impl IndicatorVertical {
    fn cls(&self) -> &'static str {
        match self {
            Self::Top => "",
            Self::Middle => "indicator-middle",
            Self::Bottom => "indicator-bottom",
        }
    }
}

#[component]
pub fn Indicator(
    children: Children,
    #[prop(optional)] horizontal: IndicatorHorizontal,
    #[prop(optional)] vertical: IndicatorVertical,
    item: Arc<dyn Fn() -> AnyView + Send + Sync>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let hc = horizontal.cls();
    if !hc.is_empty() {
        m.push(hc);
    }
    let vc = vertical.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    let cls = class_signal("indicator", &m, class);
    view! {
        <div class=cls>
            {children()}
            <span class="indicator-item">{item()}</span>
        </div>
    }
}

#[component]
pub fn IndicatorItem(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("indicator-item", &[], class);
    view! { <span class=cls>{children()}</span> }
}
