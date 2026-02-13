//! RadialProgress component — daisyUI `radial-progress`.
use crate::utils::class::class_signal;
use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn RadialProgress(
    #[prop(into)] value: f64,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("text");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("radial-progress"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("radial-progress", &refs, class);
    let style = format!("--value:{};", value);
    view! {
        <div class=cls style=style role="progressbar" aria-valuenow=value>
            {children.map(|c| c())}
        </div>
    }
    .add_any_attr(attrs)
}
