//! Modal component — daisyUI `modal`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalPosition {
    #[default]
    Center,
    Top,
    Bottom,
}
impl ModalPosition {
    fn cls(&self) -> &'static str {
        match self {
            Self::Center => "",
            Self::Top => "modal-top",
            Self::Bottom => "modal-bottom",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalState {
    #[default]
    Auto,
    Open,
    Close,
}
impl ModalState {
    fn cls(&self) -> &'static str {
        match self {
            Self::Auto => "",
            Self::Open => "modal-open",
            Self::Close => "modal-close",
        }
    }
}

#[component]
pub fn Modal(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] open: MaybeProp<bool>,
    #[prop(optional)] position: ModalPosition,
    #[prop(optional)] state: ModalState,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let pc = position.cls();
    let state_cls = state.cls();
    let cls = move || {
        let mut parts: Vec<&str> = vec!["modal"];
        if !pc.is_empty() {
            parts.push(pc);
        }
        if !state_cls.is_empty() {
            parts.push(state_cls);
        } else if open.get().unwrap_or(false) {
            parts.push("modal-open");
        }
        let base = parts.join(" ");
        match class.get() {
            Some(uc) if !uc.is_empty() => format!("{base} {uc}"),
            _ => base,
        }
    };
    view! {
        <dialog
            id=move || id.get()
            class=cls
            open=move || open.get().unwrap_or(false)
            aria-label=move || aria_label.get()
            aria-modal="true"
        >
            {children()}
        </dialog>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn ModalBox(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("modal-box", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ModalAction(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("modal-action", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn ModalBackdrop(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("modal-backdrop", &[], class);
    view! {
        <form method="dialog" class=cls>
            <button type="submit">"close"</button>
        </form>
    }
    .add_any_attr(attrs)
}
