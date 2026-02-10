//! Rating component — daisyUI `rating`.
use crate::utils::class::build_class;
use crate::variants::size::Size;
use leptos::prelude::*;

#[component]
pub fn Rating(
    #[prop(optional)] max: Option<u32>,
    #[prop(optional)] value: Option<u32>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] half: bool,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let max = max.unwrap_or(5);
    let name = name.unwrap_or_else(|| "rating".into());
    let value = value.unwrap_or(0);
    let mut m = Vec::new();
    if let Some(s) = size {
        m.push(s.class("rating"));
    }
    if half {
        m.push("rating-half".into());
    }
    let r: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "rating",
        &r,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    let stars: Vec<u32> = (1..=max).collect();
    view! {
        <div class={cls}>
            {stars.into_iter().map(|i| {
                let checked = i == value;
                let mask = "mask mask-star-2".to_string();
                let n = name.clone();
                let label = format!("{i} star");
                view! { <input type="radio" name={n} class={mask} checked={checked} disabled={disabled} aria-label={label} /> }
            }).collect_view()}
        </div>
    }
}
