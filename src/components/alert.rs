//! Alert Component
//!
//! DaisyUI alert component for displaying notification messages.
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use leptos_daisy::components::*;
//!
//! #[component]
//! fn MyComponent() -> impl IntoView {
//!     view! {
//!         <Alert variant=AlertVariant::Info>
//!             "This is an info alert"
//!         </Alert>
//!         <Alert variant=AlertVariant::Success style=AlertStyle::Outline>
//!             "This is a success alert with outline style"
//!         </Alert>
//!     }
//! }
//! ```
//!
//! Reactive usage with signals:
//! ```rust
//! use leptos::*;
//! use leptos_daisy::components::*;
//!
//! #[component]
//! fn ReactiveAlertDemo() -> impl IntoView {
//!     let alert_variant = create_signal(AlertVariant::Info);
//!     let alert_style = create_signal(AlertStyle::Default);
//!     
//!     view! {
//!         <button on:click=move |_| {
//!             alert_variant.set(AlertVariant::Success);
//!             alert_style.set(AlertStyle::Outline);
//!         }>
//!             "Change Alert"
//!         </button>
//!         
//!         <ReactiveAlert
//!             variant=alert_variant
//!             style=alert_style
//!         >
//!             "This alert changes dynamically!"
//!         </ReactiveAlert>
//!     }
//! }
//! ```

use leptos::prelude::*;
use leptos::*;

/// Alert variants/types
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AlertVariant {
    /// Info variant for informational messages
    #[default]
    Info,
    /// Success variant for success messages
    Success,
    /// Warning variant for warning messages
    Warning,
    /// Error variant for error messages
    Error,
}

/// Alert styles
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AlertStyle {
    /// Default style with solid background
    #[default]
    Default,
    /// Outline style with border and transparent background
    Outline,
    /// Dash style with dashed border
    Dash,
    /// Soft style with subtle background
    Soft,
}

/// Alert layout direction
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AlertDirection {
    /// Default layout direction
    #[default]
    Default,
    /// Vertical layout with stacked content
    Vertical,
    /// Horizontal layout with side-by-side content
    Horizontal,
}

fn get_alert_classes(
    variant: AlertVariant,
    style: AlertStyle,
    direction: AlertDirection,
    icon: bool,
    class: Option<String>,
) -> String {
    let mut classes = vec!["alert".to_string()];

    // Add variant class
    match variant {
        AlertVariant::Info => classes.push("alert-info".to_string()),
        AlertVariant::Success => classes.push("alert-success".to_string()),
        AlertVariant::Warning => classes.push("alert-warning".to_string()),
        AlertVariant::Error => classes.push("alert-error".to_string()),
    }

    // Add style class
    match style {
        AlertStyle::Outline => classes.push("alert-outline".to_string()),
        AlertStyle::Dash => classes.push("alert-dash".to_string()),
        AlertStyle::Soft => classes.push("alert-soft".to_string()),
        AlertStyle::Default => {}
    }

    // Add direction class
    match direction {
        AlertDirection::Vertical => classes.push("alert-vertical".to_string()),
        AlertDirection::Horizontal => classes.push("alert-horizontal".to_string()),
        AlertDirection::Default => {}
    }

    // Add icon class if needed
    if icon {
        classes.push("alert-icon".to_string());
    }

    // Add custom classes
    if let Some(custom_class) = class {
        classes.push(custom_class);
    }

    classes.join(" ")
}

/// Reactive Alert component that can respond to signal changes
#[component]
pub fn ReactiveAlert(
    /// The content to display within the alert
    #[prop(into)]
    children: Children,
    /// Reactive variant signal for dynamic variant changes
    #[prop(optional)]
    variant: Option<Signal<AlertVariant>>,
    /// Static variant for non-reactive use
    #[prop(optional)]
    static_variant: Option<AlertVariant>,
    /// Reactive style signal for dynamic style changes
    #[prop(optional)]
    style: Option<Signal<AlertStyle>>,
    /// Static style for non-reactive use
    #[prop(optional)]
    static_style: Option<AlertStyle>,
    /// Reactive direction signal for dynamic direction changes
    #[prop(optional)]
    direction: Option<Signal<AlertDirection>>,
    /// Static direction for non-reactive use
    #[prop(optional)]
    static_direction: Option<AlertDirection>,
    /// Whether to show an icon in the alert
    #[prop(optional)]
    icon: bool,
    /// Additional CSS classes to apply to the alert
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    // Create memoized classes that update reactively
    let classes = Memo::new(move |_| {
        let variant = variant
            .as_ref()
            .map(|s| s.get())
            .unwrap_or_else(|| static_variant.clone().unwrap_or_default());

        let style = style
            .as_ref()
            .map(|s| s.get())
            .unwrap_or_else(|| static_style.clone().unwrap_or_default());

        let direction = direction
            .as_ref()
            .map(|s| s.get())
            .unwrap_or_else(|| static_direction.clone().unwrap_or_default());

        get_alert_classes(variant, style, direction, icon, class.clone())
    });

    view! {
        <div class=classes role="alert">
            {children()}
        </div>
    }
}

/// Alert component with children (more flexible)
#[component]
pub fn Alert(
    /// The content to display within the alert
    #[prop(into)]
    children: Children,
    /// The visual variant of the alert (info, success, warning, error)
    #[prop(optional)]
    variant: AlertVariant,
    /// The style of the alert (default, outline, dash, soft)
    #[prop(optional)]
    style: AlertStyle,
    /// The layout direction of the alert (default, vertical, horizontal)
    #[prop(optional)]
    direction: AlertDirection,
    /// Whether to show an icon in the alert
    #[prop(optional)]
    icon: bool,
    /// Additional CSS classes to apply to the alert
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let classes = get_alert_classes(variant, style, direction, icon, class);

    view! {
        <div class=classes role="alert">
            {children()}
        </div>
    }
}

/// Simple alert component with text content
#[component]
pub fn SimpleAlert(
    /// The text content of the alert
    text: String,
    /// The visual variant of the alert (info, success, warning, error)
    #[prop(optional)]
    variant: AlertVariant,
    /// The style of the alert (default, outline, dash, soft)
    #[prop(optional)]
    style: AlertStyle,
    /// The layout direction of the alert (default, vertical, horizontal)
    #[prop(optional)]
    direction: AlertDirection,
    /// Whether to show an icon in the alert
    #[prop(optional)]
    icon: bool,
    /// Additional CSS classes to apply to the alert
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let classes = get_alert_classes(variant, style, direction, icon, class);

    view! {
        <div class=classes role="alert">
            {text}
        </div>
    }
}
