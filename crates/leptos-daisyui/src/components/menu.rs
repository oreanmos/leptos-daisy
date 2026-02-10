//! Menu component — daisyUI `menu`.

use crate::utils::class::build_class;
use crate::variants::size::Size;
use leptos::prelude::*;

/// A daisyUI menu component for navigation.
#[component]
pub fn Menu(
    children: Children,
    /// Size variant for the menu.
    #[prop(optional, into)]
    size: Option<Size>,
    /// Whether to display the menu vertically.
    #[prop(optional)]
    vertical: bool,
    /// Whether to display the menu horizontally.
    #[prop(optional)]
    horizontal: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if let Some(s) = size {
        mods.push(s.class("menu"));
    }

    if vertical {
        mods.push("menu-vertical".to_string());
    }

    if horizontal {
        mods.push("menu-horizontal".to_string());
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "menu",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <ul class={cls}>
            {children()}
        </ul>
    }
}

/// A menu item within a Menu.
#[component]
pub fn MenuItem(
    children: Children,
    /// Whether this item is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether this item is active.
    #[prop(optional)]
    active: bool,
    /// Whether this item has focus styling.
    #[prop(optional)]
    focused: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if disabled {
        mods.push("menu-disabled");
    }

    if active {
        mods.push("menu-active");
    }

    if focused {
        mods.push("menu-focus");
    }

    let refs: Vec<&str> = mods.to_vec();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <li class={cls}>
            {children()}
        </li>
    }
}

/// A menu title/section header within a Menu.
#[component]
pub fn MenuTitle(
    children: Children,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "menu-title",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <li class={cls}>
            {children()}
        </li>
    }
}

/// A dropdown submenu within a Menu.
#[component]
pub fn MenuDropdown(
    children: Children,
    /// The trigger content for the dropdown.
    trigger: Children,
    /// Additional classes to apply to the dropdown container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "menu-dropdown",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <li class={cls}>
            {trigger()}
            <ul>
                {children()}
            </ul>
        </li>
    }
}
