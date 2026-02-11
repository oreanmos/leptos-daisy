//! Drawer component — daisyUI `drawer`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional)] end: bool,
    #[prop(into)] id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if end {
        m.push("drawer-end");
    }
    let cls = class_signal("drawer", &m, class);
    view! {
        <div class=cls>
            <input id=id.clone() type="checkbox" class="drawer-toggle" />
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-content", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DrawerSide(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-side", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DrawerOverlay(
    #[prop(into)] drawer_id: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("drawer-overlay", &[], class);
    view! { <label for=drawer_id class=cls aria-hidden="true"></label> }
}
