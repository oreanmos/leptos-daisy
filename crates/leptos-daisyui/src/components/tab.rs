//! Tab component — daisyUI `tabs`.

use crate::utils::class::build_class;
use crate::variants::size::Size;
use leptos::prelude::*;

/// Variant styles for tabs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabVariant {
    #[default]
    Default,
    /// Lifted tabs with shadow effect.
    Lift,
    /// Boxed tabs with border.
    Box,
    /// Bordered tabs.
    Border,
}

impl TabVariant {
    /// Get the CSS class for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Lift => "tabs-lift",
            Self::Box => "tabs-box",
            Self::Border => "tabs-border",
        }
    }
}

/// Position for tabs (top or bottom).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
}

impl TabPosition {
    /// Get the CSS class for this position.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Top => "",
            Self::Bottom => "tabs-bottom",
        }
    }
}

/// A daisyUI tabs container component.
#[component]
pub fn Tabs(
    children: Children,
    /// Size variant for the tabs.
    #[prop(optional, into)]
    size: Option<Size>,
    /// Visual variant for the tabs.
    #[prop(optional, into)]
    variant: Option<TabVariant>,
    /// Position of the tabs (top or bottom).
    #[prop(optional, into)]
    position: Option<TabPosition>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if let Some(s) = size {
        mods.push(s.class("tabs"));
    }

    if let Some(v) = variant {
        let c = v.class();
        if !c.is_empty() {
            mods.push(c.to_string());
        }
    }

    if let Some(p) = position {
        let c = p.class();
        if !c.is_empty() {
            mods.push(c.to_string());
        }
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "tabs",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls} role="tablist">
            {children()}
        </div>
    }
}

/// A single tab item.
#[component]
pub fn Tab(
    children: Children,
    /// Whether this tab is active.
    #[prop(optional)]
    active: bool,
    /// Whether this tab is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if active {
        mods.push("tab-active");
    }

    if disabled {
        mods.push("tab-disabled");
    }

    let refs: Vec<&str> = mods.to_vec();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "tab",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <button
            class={cls}
            role="tab"
            aria-selected={active}
            disabled={disabled}
            type="button"
        >
            {children()}
        </button>
    }
}

/// Content panel for a tab.
#[component]
pub fn TabPanel(
    children: Children,
    /// Whether this panel is active/visible.
    #[prop(optional)]
    active: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "tab-content",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div
            class={cls}
            role="tabpanel"
            hidden={!active}
        >
            {children()}
        </div>
    }
}

/// A radio-based tab for automatic state management.
#[component]
pub fn TabRadio(
    children: Children,
    /// The name for the radio group.
    #[prop(into)]
    name: String,
    /// The value for this tab.
    #[prop(into)]
    value: String,
    /// Whether this tab is checked by default.
    #[prop(optional)]
    checked: bool,
    /// Whether this tab is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "tab",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <label class={cls}>
            <input
                type="radio"
                name={name}
                value={value}
                class="tab"
                checked={checked}
                disabled={disabled}
                aria-disabled={disabled}
            />
            {children()}
        </label>
    }
}
