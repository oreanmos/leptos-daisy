//! Button component — daisyUI `btn`.

use crate::utils::class::build_class;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::state::State;
use crate::variants::variant::Variant;
use leptos::prelude::*;

/// A daisyUI button.
#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] variant: Option<Variant>,
    #[prop(optional, into)] state: Option<State>,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional)] glass: bool,
    #[prop(optional)] wide: bool,
    #[prop(optional)] no_animation: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();
    if let Some(c) = color {
        let s = c.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    if let Some(s) = size {
        mods.push(s.class("btn"));
    }
    if let Some(v) = variant {
        let s = v.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    if let Some(st) = state {
        let s = st.class("btn");
        if !s.is_empty() {
            mods.push(s);
        }
    }
    if square {
        mods.push("btn-square".into());
    }
    if circle {
        mods.push("btn-circle".into());
    }
    if glass {
        mods.push("glass".into());
    }
    if wide {
        mods.push("btn-wide".into());
    }
    if no_animation {
        mods.push("no-animation".into());
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "btn",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <button class={cls} type="button" disabled={disabled}>
            {children()}
        </button>
    }
}
