//! Breadcrumbs component — daisyUI `breadcrumbs`.
use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}
impl BreadcrumbItem {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
        }
    }
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }
}

#[component]
pub fn Breadcrumbs(
    #[prop(into)] items: Vec<BreadcrumbItem>,
    #[prop(optional, into)] separator: Option<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("breadcrumbs", &["text-sm"], class);
    let _sep = separator.unwrap_or_else(|| "/".to_string());
    view! {
        <div class=cls>
            <ul>
                {items.into_iter().map(|item| {
                    view! {
                        <li>
                            {if let Some(href) = item.href {
                                view! { <a href=href>{item.label}</a> }.into_any()
                            } else {
                                view! { <span aria-current="page">{item.label}</span> }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
