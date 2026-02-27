//! FormField component — structured form field with label, hint, and error display.

use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// A structured form field that wraps inputs with label, hint text, and error state.
///
/// Renders a `<fieldset>` + `<label>` + children (input slot) + optional
/// hint and error text. Replaces the common `Fieldset` + `Label` + `Input`
/// boilerplate pattern.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <FormField label="Email" hint="We'll never share your email." required=true>
///         <Input placeholder="you@example.com" />
///     </FormField>
/// }
/// ```
#[component]
pub fn FormField(
    /// The field label text.
    #[prop(into)]
    label: String,
    /// Optional hint text shown below the input.
    #[prop(optional, into)]
    hint: Option<String>,
    /// Optional error message. When present, the field displays in error state.
    #[prop(optional, into)]
    error: MaybeProp<String>,
    /// Whether to show a required indicator (*) after the label.
    #[prop(optional)]
    required: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let cls = move || {
        let base = "fieldset";
        match class.get() {
            Some(extra) => format!("{base} {extra}"),
            None => base.to_string(),
        }
    };

    let required_marker = if required {
        Some(view! { <span class="text-error ml-0.5">"*"</span> })
    } else {
        None
    };

    view! {
        <fieldset class=cls>
            <label class="label">
                <span class="label-text font-medium">
                    {label}
                    {required_marker}
                </span>
            </label>
            {children()}
            {move || {
                error.get().map(|e| view! { <p class="label-text text-error text-sm mt-1">{e}</p> })
            }}
            {hint.map(|h| view! { <p class="label-text text-base-content/60 text-sm mt-1">{h}</p> })}
        </fieldset>
    }
    .add_any_attr(attrs)
}
