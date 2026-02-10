//! Pagination component — uses daisyUI `join` and `join-item` classes.

use crate::utils::class::build_class;
use crate::variants::size::Size;
use leptos::prelude::*;

/// A pagination component using daisyUI join classes.
#[component]
pub fn Pagination(
    /// The current active page (1-based).
    #[prop(into)]
    current_page: usize,
    /// The total number of pages.
    #[prop(into)]
    total_pages: usize,
    /// Callback when a page is selected. Receives the 1-based page number.
    #[prop(optional, into)]
    on_page_change: Option<Callback<usize>>,
    /// Size variant for the pagination buttons.
    #[prop(optional, into)]
    size: Option<Size>,
    /// Whether to show the first/last page buttons.
    #[prop(optional)]
    show_first_last: bool,
    /// Whether to show the previous/next buttons.
    #[prop(optional)]
    show_prev_next: bool,
    /// Maximum number of page buttons to show (will ellipsis others).
    #[prop(optional, into)]
    max_visible: Option<usize>,
    /// Label for the previous button.
    #[prop(optional, into)]
    prev_label: Option<String>,
    /// Label for the next button.
    #[prop(optional, into)]
    next_label: Option<String>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "join",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    let size_class = size.map(|s| s.class("btn")).unwrap_or_default();

    let prev_text = prev_label.unwrap_or_else(|| "«".to_string());
    let next_text = next_label.unwrap_or_else(|| "»".to_string());

    // Generate visible page numbers
    let visible_pages = if let Some(max) = max_visible {
        if total_pages <= max {
            (1..=total_pages).collect::<Vec<_>>()
        } else {
            let half = max / 2;
            let start = if current_page <= half {
                1
            } else if current_page + half > total_pages {
                total_pages - max + 1
            } else {
                current_page - half
            };
            (start..=(start + max - 1).min(total_pages)).collect::<Vec<_>>()
        }
    } else {
        (1..=total_pages).collect::<Vec<_>>()
    };

    let handle_page_click = move |page: usize| {
        if let Some(cb) = on_page_change {
            cb.run(page);
        }
    };

    view! {
        <div class={cls}>
            {if show_first_last && current_page > 1 {
                view! {
                    <button
                        class={format!("join-item btn {}", size_class)}
                        on:click=move |_| handle_page_click(1)
                        type="button"
                    >
                        "First"
                    </button>
                }.into_any()
            } else {
                ().into_any()
            }}

            {if show_prev_next && current_page > 1 {
                view! {
                    <button
                        class={format!("join-item btn {}", size_class)}
                        on:click=move |_| handle_page_click(current_page - 1)
                        type="button"
                    >
                        {prev_text.clone()}
                    </button>
                }.into_any()
            } else {
                ().into_any()
            }}

            {visible_pages.into_iter().map(|page| {
                let is_active = page == current_page;
                let btn_class = if is_active {
                    format!("join-item btn btn-active {}", size_class)
                } else {
                    format!("join-item btn {}", size_class)
                };

                view! {
                    <button
                        class={btn_class}
                        on:click=move |_| handle_page_click(page)
                        aria-current={if is_active { Some("page") } else { None }}
                        type="button"
                    >
                        {page.to_string()}
                    </button>
                }
            }).collect::<Vec<_>>()}

            {if show_prev_next && current_page < total_pages {
                view! {
                    <button
                        class={format!("join-item btn {}", size_class)}
                        on:click=move |_| handle_page_click(current_page + 1)
                        type="button"
                    >
                        {next_text.clone()}
                    </button>
                }.into_any()
            } else {
                ().into_any()
            }}

            {if show_first_last && current_page < total_pages {
                view! {
                    <button
                        class={format!("join-item btn {}", size_class)}
                        on:click=move |_| handle_page_click(total_pages)
                        type="button"
                    >
                        "Last"
                    </button>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

/// A simple pagination item for custom pagination layouts.
#[component]
pub fn PaginationItem(
    children: Children,
    /// Whether this item is active.
    #[prop(optional)]
    active: bool,
    /// Whether this item is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<Size>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Additional classes to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods = Vec::new();

    if active {
        mods.push("btn-active");
    }

    let size_class = size.map(|s| s.class("btn")).unwrap_or_default();
    if !size_class.is_empty() {
        mods.push(size_class.as_str());
    }

    let refs: Vec<&str> = mods.to_vec();
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "join-item btn",
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
        <button
            class={cls}
            disabled={disabled}
            on:click=handle_click
            type="button"
        >
            {children()}
        </button>
    }
}
