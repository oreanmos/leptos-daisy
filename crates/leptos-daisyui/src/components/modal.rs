//! Modal component — daisyUI `modal`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalBoxSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Full,
}
impl ModalBoxSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Small => "max-w-sm",
            Self::Medium => "max-w-md",
            Self::Large => "max-w-lg",
            Self::ExtraLarge => "max-w-2xl",
            Self::Full => "max-w-4xl w-full",
        }
    }
}

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

fn modal_cls(
    position: ModalPosition,
    state: ModalState,
    open: bool,
    class: Option<String>,
) -> String {
    let pc = position.cls();
    let state_cls = state.cls();
    let mut parts: Vec<&str> = vec!["modal"];
    if !pc.is_empty() {
        parts.push(pc);
    }
    if !state_cls.is_empty() {
        parts.push(state_cls);
    } else if open {
        parts.push("modal-open");
    }
    let base = parts.join(" ");
    match class {
        Some(uc) if !uc.is_empty() => format!("{base} {uc}"),
        _ => base,
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
    let cls = move || modal_cls(position, state, open.get().unwrap_or(false), class.get());
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
    #[prop(optional, into)] size: Option<ModalBoxSize>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    if let Some(s) = size {
        let c = s.cls();
        if !c.is_empty() {
            m.push(c);
        }
    }
    let cls = class_signal("modal-box", &m, class);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_box_size_cls() {
        assert_eq!(ModalBoxSize::Small.cls(), "max-w-sm");
        assert_eq!(ModalBoxSize::Medium.cls(), "max-w-md");
        assert_eq!(ModalBoxSize::Large.cls(), "max-w-lg");
        assert_eq!(ModalBoxSize::ExtraLarge.cls(), "max-w-2xl");
        assert_eq!(ModalBoxSize::Full.cls(), "max-w-4xl w-full");
    }

    #[test]
    fn test_modal_position_cls() {
        assert_eq!(ModalPosition::Center.cls(), "");
        assert_eq!(ModalPosition::Top.cls(), "modal-top");
        assert_eq!(ModalPosition::Bottom.cls(), "modal-bottom");
    }

    #[test]
    fn test_modal_state_cls() {
        assert_eq!(ModalState::Auto.cls(), "");
        assert_eq!(ModalState::Open.cls(), "modal-open");
        assert_eq!(ModalState::Close.cls(), "modal-close");
    }

    #[test]
    fn test_modal_cls_default() {
        assert_eq!(
            modal_cls(ModalPosition::Center, ModalState::Auto, false, None),
            "modal"
        );
    }

    #[test]
    fn test_modal_cls_open_prop() {
        assert_eq!(
            modal_cls(ModalPosition::Center, ModalState::Auto, true, None),
            "modal modal-open"
        );
    }

    #[test]
    fn test_modal_cls_state_open() {
        // state=Open overrides open=false
        assert_eq!(
            modal_cls(ModalPosition::Center, ModalState::Open, false, None),
            "modal modal-open"
        );
        // state=Open overrides open=true (same result)
        assert_eq!(
            modal_cls(ModalPosition::Center, ModalState::Open, true, None),
            "modal modal-open"
        );
    }

    #[test]
    fn test_modal_cls_state_close() {
        // state=Close overrides open=true
        assert_eq!(
            modal_cls(ModalPosition::Center, ModalState::Close, true, None),
            "modal modal-close"
        );
    }

    #[test]
    fn test_modal_cls_position() {
        assert_eq!(
            modal_cls(ModalPosition::Top, ModalState::Auto, false, None),
            "modal modal-top"
        );
        assert_eq!(
            modal_cls(ModalPosition::Bottom, ModalState::Auto, false, None),
            "modal modal-bottom"
        );
    }

    #[test]
    fn test_modal_cls_extra_class() {
        assert_eq!(
            modal_cls(
                ModalPosition::Center,
                ModalState::Auto,
                false,
                Some("custom-class".to_string())
            ),
            "modal custom-class"
        );
    }

    #[test]
    fn test_modal_cls_complex() {
        assert_eq!(
            modal_cls(
                ModalPosition::Top,
                ModalState::Close,
                true, // ignored due to state
                Some("custom-class".to_string())
            ),
            "modal modal-top modal-close custom-class"
        );
    }
}
