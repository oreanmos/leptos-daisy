//! Card Component
//!
//! DaisyUI card component for displaying content in a box with optional header, body, and actions.

use leptos::prelude::*;
use leptos::*;

/// Card variants/styles
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CardVariant {
    #[default]
    Default,
    Bordered,
    Compact,
    Normal,
    Side,
}

fn get_card_classes(
    variant: CardVariant,
    bordered: bool,
    compact: bool,
    normal: bool,
    side: bool,
    class: Option<String>,
) -> String {
    let mut classes = vec!["card".to_string()];

    // Add variant class
    match variant {
        CardVariant::Bordered => classes.push("card-bordered".to_string()),
        CardVariant::Compact => classes.push("card-compact".to_string()),
        CardVariant::Normal => classes.push("card-normal".to_string()),
        CardVariant::Side => classes.push("card-side".to_string()),
        CardVariant::Default => {}
    }

    // Add modifier classes
    if bordered {
        classes.push("card-bordered".to_string());
    }
    if compact {
        classes.push("card-compact".to_string());
    }
    if normal {
        classes.push("card-normal".to_string());
    }
    if side {
        classes.push("card-side".to_string());
    }

    // Add custom classes
    if let Some(custom_class) = class {
        classes.push(custom_class);
    }

    classes.join(" ")
}

/// Card component with children (more flexible)
#[component]
pub fn Card(
    #[prop(into)] children: Children,
    #[prop(optional)] variant: CardVariant,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] compact: bool,
    #[prop(optional)] normal: bool,
    #[prop(optional)] side: bool,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = get_card_classes(variant, bordered, compact, normal, side, class);

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card header component
#[component]
pub fn CardHeader(
    #[prop(into)] children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = format!("card-header {}", class.unwrap_or_default());

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card body component
#[component]
pub fn CardBody(
    #[prop(into)] children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = format!("card-body {}", class.unwrap_or_default());

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card title component
#[component]
pub fn CardTitle(
    #[prop(into)] children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = format!("card-title {}", class.unwrap_or_default());

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Card actions component
#[component]
pub fn CardActions(
    #[prop(into)] children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = format!("card-actions {}", class.unwrap_or_default());

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}
