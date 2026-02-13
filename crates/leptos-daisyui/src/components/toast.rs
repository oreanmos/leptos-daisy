//! Toast component — daisyUI `toast`.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastHorizontal {
    Start,
    Center,
    #[default]
    End,
}
impl ToastHorizontal {
    fn cls(&self) -> &'static str {
        match self {
            Self::Start => "toast-start",
            Self::Center => "toast-center",
            Self::End => "toast-end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVertical {
    Top,
    Middle,
    #[default]
    Bottom,
}
impl ToastVertical {
    fn cls(&self) -> &'static str {
        match self {
            Self::Top => "toast-top",
            Self::Middle => "toast-middle",
            Self::Bottom => "toast-bottom",
        }
    }
}

#[component]
pub fn Toast(
    children: Children,
    #[prop(optional)] horizontal: ToastHorizontal,
    #[prop(optional)] vertical: ToastVertical,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("toast", &[horizontal.cls(), vertical.cls()], class);
    view! { <div class=cls role="alert" aria-live="polite">{children()}</div> }.add_any_attr(attrs)
}
