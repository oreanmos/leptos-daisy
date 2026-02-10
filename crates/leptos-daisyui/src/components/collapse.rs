//! Collapse component — daisyUI `collapse` + accordion support.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Collapse trigger type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseTrigger {
    /// Uses checkbox for toggle (default).
    #[default]
    Checkbox,
    /// Uses details/summary elements.
    Details,
    /// Uses radio for accordion behavior.
    Radio,
}

/// Collapse component for collapsible content.
#[component]
pub fn Collapse(
    children: Children,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] open: bool,
    #[prop(optional)] trigger: CollapseTrigger,
    #[prop(optional, into)] radio_name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "collapse",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    match trigger {
        CollapseTrigger::Details => view! {
            <details class={cls} open={open}>
                {title.map(|t| view! { <summary class="collapse-title">{t}</summary> })}
                <div class="collapse-content">{children()}</div>
            </details>
        }
        .into_any(),
        CollapseTrigger::Checkbox => {
            let checkbox_id = format!(
                "collapse-{:x}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            );
            view! {
                <div class={cls}>
                    <input type="checkbox" id={&checkbox_id} checked={open} />
                    {title.map(|t| view! { <label for={&checkbox_id} class="collapse-title">{t}</label> })}
                    <div class="collapse-content">{children()}</div>
                </div>
            }
            .into_any()
        }
        CollapseTrigger::Radio => {
            let name = radio_name
                .get()
                .unwrap_or_else(|| "collapse-radio".to_string());
            let radio_id = format!(
                "{}-{:x}",
                name,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            );
            view! {
                <div class={cls}>
                    <input type="radio" name={name} id={&radio_id} checked={open} />
                    {title.map(|t| view! { <label for={&radio_id} class="collapse-title">{t}</label> })}
                    <div class="collapse-content">{children()}</div>
                </div>
            }
            .into_any()
        }
    }
}

/// Collapse title component (for custom title content).
#[component]
pub fn CollapseTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "collapse-title",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Collapse content component.
#[component]
pub fn CollapseContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "collapse-content",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <div class={cls}>{children()}</div> }
}

/// Accordion group container for radio-based collapses.
#[component]
pub fn Accordion(
    children: Children,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], None);
    let final_cls = format!("{} {}", cls, uc);
    view! {
        <div class={final_cls} data-accordion={name}>
            {children()}
        </div>
    }
}
