//! Dropdown component — daisyUI `dropdown`.
use crate::utils::class::class_signal;
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
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn DropdownTrigger(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! { <div tabindex="0" role="button" class=cls>{children()}</div> }
}

#[component]
pub fn DropdownContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
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
    view! { <ul tabindex="0" class=cls>{children()}</ul> }
}

#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] active: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = move || {
        let mut m: Vec<&str> = Vec::new();
        if active.get().unwrap_or(false) {
            m.push("active");
        }
        if disabled.get().unwrap_or(false) {
            m.push("disabled");
        }
        let user_class = class.get().unwrap_or_default();
        let static_cls = m.join(" ");
        if user_class.is_empty() {
            static_cls
        } else if static_cls.is_empty() {
            user_class
        } else {
            format!("{static_cls} {user_class}")
        }
    };

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
}
