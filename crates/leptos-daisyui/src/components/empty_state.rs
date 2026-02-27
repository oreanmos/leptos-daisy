//! EmptyState component — centered placeholder for empty content areas.

use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// A centered empty-state placeholder with icon, message, and optional action slot.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <EmptyState message="No items found" description="Try adding some items to get started.">
///         <Button color=Color::Primary>"Add Item"</Button>
///     </EmptyState>
/// }
/// ```
#[component]
pub fn EmptyState(
    /// Main message text.
    #[prop(into)]
    message: String,
    /// Optional secondary description.
    #[prop(optional, into)]
    description: Option<String>,
    /// Optional icon slot (replaces the default empty-box icon).
    #[prop(optional)]
    icon: Option<Children>,
    /// Optional action slot (e.g. a button).
    #[prop(optional)]
    children: Option<Children>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || {
        let base = "flex flex-col items-center justify-center py-16 text-base-content/60";
        match class.get() {
            Some(extra) => format!("{base} {extra}"),
            None => base.to_string(),
        }
    };

    let default_icon = icon.is_none();

    view! {
        <div class=cls>
            {if default_icon {
                Some(view! {
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                         stroke-width="1" stroke="currentColor" class="w-16 h-16 mb-4 opacity-40">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5m6 4.125l2.25 2.25m0 0l2.25 2.25M12 11.625l2.25-2.25M12 11.625l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z" />
                    </svg>
                }.into_any())
            } else {
                None
            }}
            {icon.map(|i| view! { <div class="mb-4">{i()}</div> })}
            <p class="text-lg font-medium">{message}</p>
            {description.map(|d| view! { <p class="text-sm mt-2">{d}</p> })}
            {children.map(|c| view! { <div class="mt-4">{c()}</div> })}
        </div>
    }
    .add_any_attr(attrs)
}
