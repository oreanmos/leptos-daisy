//! Dropdown component — daisyUI `dropdown`.
use crate::utils::class::{class_signal, class_signal_dynamic};
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
    Start,
    End,
    Center,
}
impl DropdownPosition {
    fn cls(&self) -> &'static str {
        match self {
            Self::Bottom => "dropdown-bottom",
            Self::Top => "dropdown-top",
            Self::Left => "dropdown-left",
            Self::Right => "dropdown-right",
            Self::Start => "dropdown-start",
            Self::End => "dropdown-end",
            Self::Center => "dropdown-center",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownHover {
    #[default]
    Click,
    Hover,
}
impl DropdownHover {
    fn cls(&self) -> &'static str {
        match self {
            Self::Click => "",
            Self::Hover => "dropdown-hover",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownState {
    #[default]
    Auto,
    Open,
    Close,
}
impl DropdownState {
    fn cls(&self) -> &'static str {
        match self {
            Self::Auto => "",
            Self::Open => "dropdown-open",
            Self::Close => "dropdown-close",
        }
    }
}

#[component]
pub fn Dropdown(
    children: Children,
    #[prop(optional)] position: DropdownPosition,
    #[prop(optional)] hover: DropdownHover,
    #[prop(optional)] state: DropdownState,
    #[prop(optional)] open: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut m: Vec<&str> = vec![position.cls()];
    let hc = hover.cls();
    if !hc.is_empty() {
        m.push(hc);
    }
    let state_cls = state.cls();
    if !state_cls.is_empty() {
        m.push(state_cls);
    } else if open {
        m.push("dropdown-open");
    }
    let cls = class_signal("dropdown", &m, class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn DropdownTrigger(
    children: Children,
    #[prop(optional)] expanded: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! {
        <button type="button" tabindex="0" class=cls
            aria-haspopup="true"
            aria-expanded=expanded
        >{children()}</button>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn DropdownContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal(
        "dropdown-content",
        &[
            "menu",
            "bg-base-100",
            "rounded-box",
            "z-1",
            "w-52",
            "p-2",
            "shadow",
        ],
        class,
    );
    view! { <ul tabindex="0" class=cls role="listbox">{children()}</ul> }.add_any_attr(attrs)
}

#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] active: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal_dynamic(
        "",
        move || {
            let mut m = Vec::new();
            if active.get().unwrap_or(false) {
                m.push("active".to_string());
            }
            if disabled.get().unwrap_or(false) {
                m.push("disabled".to_string());
            }
            m
        },
        class,
    );

    view! {
        <li class=cls>
            <a
                href=move || {
                    if disabled.get().unwrap_or(false) {
                        None
                    } else {
                        let value = href.get().unwrap_or_else(|| "#".to_string());
                        Some(value)
                    }
                }
                aria-disabled=move || disabled.get().unwrap_or(false)
                tabindex=move || {
                    if disabled.get().unwrap_or(false) {
                        -1
                    } else {
                        0
                    }
                }
            >
                {children()}
            </a>
        </li>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_position_classes() {
        assert_eq!(DropdownPosition::Bottom.cls(), "dropdown-bottom");
        assert_eq!(DropdownPosition::Top.cls(), "dropdown-top");
        assert_eq!(DropdownPosition::Left.cls(), "dropdown-left");
        assert_eq!(DropdownPosition::Right.cls(), "dropdown-right");
        assert_eq!(DropdownPosition::Start.cls(), "dropdown-start");
        assert_eq!(DropdownPosition::End.cls(), "dropdown-end");
        assert_eq!(DropdownPosition::Center.cls(), "dropdown-center");
    }

    #[test]
    fn test_dropdown_hover_classes() {
        assert_eq!(DropdownHover::Click.cls(), "");
        assert_eq!(DropdownHover::Hover.cls(), "dropdown-hover");
    }

    #[test]
    fn test_dropdown_state_classes() {
        assert_eq!(DropdownState::Auto.cls(), "");
        assert_eq!(DropdownState::Open.cls(), "dropdown-open");
        assert_eq!(DropdownState::Close.cls(), "dropdown-close");
    }
}
