//! FAB (Floating Action Button) component — daisyUI `fab`.

use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Fab(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    #[prop(optional)] flower: bool,
    children: Children,
) -> impl IntoView {
    let mut mods = Vec::new();
    if flower {
        mods.push("fab-flower");
    }
    let cls = class_signal("fab", &mods, class);
    view! {
        <div class=cls>
            {children()}
        </div>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn FabTrigger(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("btn", &[], class);
    view! {
        <button type="button" class=cls>
            {children()}
        </button>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn FabAction(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    #[prop(optional, into)] label: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("btn", &[], class);
    view! {
        <div>
            {move || label.get()}
            <button type="button" class=cls>
                {children()}
            </button>
        </div>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn FabClose(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let has_label = move || label.get().is_some();
    let cls = class_signal("fab-close", &[], class);
    view! {
        <div class=cls>
            <Show when=has_label>
                {move || label.get()}
            </Show>
            {children()}
        </div>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn FabMainAction(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal("fab-main-action", &[], class);
    view! {
        <div class=cls>
            {children()}
        </div>
    }
    .add_any_attr(attrs)
}
