//! ErrorState component — an error display with optional retry action.
//!
//! Combines an `Alert` with `Color::Error`, a message, optional description,
//! and a retry button or custom actions.

use crate::components::alert::Alert;
use crate::components::button::Button;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// An error display component with a message and optional retry action.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ErrorState
///         message="Failed to load data"
///         description="The server returned an error. Please try again."
///         on_retry=Callback::new(|_| { /* retry logic */ })
///     />
/// }
/// ```
#[component]
pub fn ErrorState(
    #[prop(into)] message: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] on_retry: Option<Callback<()>>,
    #[prop(optional, into)] retry_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let label = retry_label.unwrap_or_else(|| "Retry".to_string());

    view! {
        <Alert color=Color::Error class=class>
            <div class="flex flex-col gap-2">
                <span class="font-semibold">{message}</span>
                {description.map(|d| view! { <span class="text-sm opacity-80">{d}</span> })}
            </div>
            {if let Some(custom) = children {
                custom().into_any()
            } else if let Some(retry) = on_retry {
                view! {
                    <Button
                        color=Color::Error
                        on_click=Callback::new(move |_| retry.run(()))
                    >
                        {label.clone()}
                    </Button>
                }.into_any()
            } else {
                ().into_any()
            }}
        </Alert>
    }
    .add_any_attr(attrs)
}
