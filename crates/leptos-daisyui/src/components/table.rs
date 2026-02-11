//! Table component — daisyUI `table`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::prelude::*;

#[component]
pub fn Table(
    children: Children,
    #[prop(optional)] zebra: bool,
    #[prop(optional)] pin_rows: bool,
    #[prop(optional)] pin_cols: bool,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<String> = Vec::new();
    if zebra {
        m.push("table-zebra".into());
    }
    if pin_rows {
        m.push("table-pin-rows".into());
    }
    if pin_cols {
        m.push("table-pin-cols".into());
    }
    if let Some(s) = size {
        m.push(s.class("table"));
    }
    let refs: Vec<&str> = m.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("table", &refs, class);
    view! { <table class=cls>{children()}</table> }
}
