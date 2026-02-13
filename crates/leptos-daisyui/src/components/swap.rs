//! Swap component — daisyUI `swap`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapAnimation {
    #[default]
    None,
    Rotate,
    Flip,
}
impl SwapAnimation {
    fn cls(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Rotate => "swap-rotate",
            Self::Flip => "swap-flip",
        }
    }
}

#[component]
pub fn Swap(
    on: Children,
    off: Children,
    #[prop(optional)] animation: SwapAnimation,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let ac = animation.cls();
    if !ac.is_empty() {
        m.push(ac);
    }
    if active {
        m.push("swap-active");
    }
    let cls = class_signal("swap", &m, class);
    view! {
        <label class=cls>
            <input type="checkbox"
                aria-label=move || aria_label.get()
                aria-checked=active
            />
            <div class="swap-on">{on()}</div>
            <div class="swap-off">{off()}</div>
        </label>
    }
    .add_any_attr(attrs)
}
