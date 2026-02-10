//! Dock component — daisyUI `dock`.
//! Bottom navigation bar (replaces deprecated bottom-navigation).

use crate::utils::class::build_class;
use crate::variants::size::Size;
use leptos::prelude::*;

/// Data structure for a dock item.
#[derive(Clone, Debug)]
pub struct DockItem {
    /// The label/text to display.
    pub label: String,
    /// The icon to display (HTML string or icon class).
    pub icon: String,
    /// Whether this item is active.
    pub active: bool,
    /// Optional href for the item.
    pub href: Option<String>,
}

impl DockItem {
    /// Create a new dock item.
    pub fn new(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            active: false,
            href: None,
        }
    }

    /// Create a new active dock item.
    pub fn active(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            active: true,
            href: None,
        }
    }

    /// Set the href for this item.
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Set whether this item is active.
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// A daisyUI dock component for bottom navigation.
#[component]
pub fn Dock(
    /// The dock items to display.
    #[prop(into)]
    items: Vec<DockItem>,
    /// Size variant for the dock.
    #[prop(optional, into)]
    size: Option<Size>,
    /// Callback when an item is clicked. Receives the item index.
    #[prop(optional, into)]
    on_item_click: Option<Callback<usize>>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if let Some(s) = size {
        mods.push(s.class("dock"));
    }

    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "dock",
        &refs,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let handle_click = move |index: usize| {
        if let Some(cb) = on_item_click {
            cb.run(index);
        }
    };

    view! {
        <div class={cls}>
            {items.into_iter().enumerate().map(|(index, item)| {
                let item_class = if item.active {
                    "dock-active"
                } else {
                    ""
                };

                view! {
                    {if let Some(href) = item.href {
                        view! {
                            <a
                                href={href}
                                class={item_class}
                                aria-current={if item.active { Some("page") } else { None }}
                            >
                                <span class="dock-icon" inner_html={item.icon.clone()}></span>
                                <span class="dock-label">{item.label.clone()}</span>
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                class={item_class}
                                on:click=move |_| handle_click(index)
                                type="button"
                                aria-current={if item.active { Some("page") } else { None }}
                            >
                                <span class="dock-icon" inner_html={item.icon.clone()}></span>
                                <span class="dock-label">{item.label.clone()}</span>
                            </button>
                        }.into_any()
                    }}
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// A single dock item for custom layouts.
#[component]
pub fn DockItemComponent(
    children: Children,
    /// Whether this item is active.
    #[prop(optional)]
    active: bool,
    /// Optional href for the item.
    #[prop(optional, into)]
    href: Option<String>,
    /// Click handler for the item.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if active {
        mods.push("dock-active");
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

    let handle_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };

    view! {
        {if let Some(href) = href {
            view! {
                <a
                    class={cls}
                    href={href}
                    aria-current={if active { Some("page") } else { None }}
                >
                    {children()}
                </a>
            }.into_any()
        } else {
            view! {
                <button
                    class={cls}
                    on:click=handle_click
                    type="button"
                    aria-current={if active { Some("page") } else { None }}
                >
                    {children()}
                </button>
            }.into_any()
        }}
    }
}
