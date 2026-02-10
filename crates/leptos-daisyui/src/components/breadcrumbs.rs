//! Breadcrumbs component — daisyUI `breadcrumbs`.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// Data structure for a breadcrumb item.
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// The label/text to display.
    pub label: String,
    /// The href for the link (None for current page).
    pub href: Option<String>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item with a link.
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
        }
    }

    /// Create a new breadcrumb item for the current page (no link).
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }
}

/// A daisyUI breadcrumbs component for navigation.
#[component]
pub fn Breadcrumbs(
    /// The breadcrumb items to display.
    #[prop(into)]
    items: Vec<BreadcrumbItem>,
    /// Separator character (default: "/").
    #[prop(optional, into)]
    separator: Option<String>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "breadcrumbs",
        &["text-sm"],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let _sep = separator.unwrap_or_else(|| "/".to_string());

    view! {
        <div class={cls}>
            <ul>
                {items.into_iter().map(|item| {
                    view! {
                        <li>
                            {if let Some(href) = item.href {
                                view! {
                                    <a href={href}>{item.label}</a>
                                }.into_any()
                            } else {
                                view! {
                                    <span aria-current="page">{item.label}</span>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
