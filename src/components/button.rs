//! Button Component
//!
//! DaisyUI button component with various styles and sizes.

use crate::utils::class::merge_with_base;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::state::State;
use crate::variants::variant::Variant;
use leptos::prelude::*;
use leptos::*;

fn get_button_classes(
    color: Option<Color>,
    size: Option<Size>,
    variant: Option<Variant>,
    state: Option<State>,
    square: bool,
    circle: bool,
    glass: bool,
    no_animation: bool,
) -> String {
    let mut classes = Vec::new();

    // Start with base class
    classes.push("btn".to_string());

    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("btn"));
        }
    }

    // Add size class
    if let Some(size) = size {
        if size != Size::Medium {
            // Medium is default
            classes.push(size.to_class("btn"));
        }
    }

    // Add variant class
    if let Some(variant) = variant {
        if variant != Variant::Solid {
            // Solid is default
            classes.push(variant.to_class("btn"));
        }
    }

    // Add state class
    if let Some(state) = state {
        if state != State::Default {
            classes.push(state.to_class("btn"));
        }
    }

    // Add shape classes
    if square {
        classes.push("btn-square".to_string());
    }
    if circle {
        classes.push("btn-circle".to_string());
    }

    // Add other modifiers
    if glass {
        classes.push("glass".to_string());
    }
    if no_animation {
        classes.push("no-animation".to_string());
    }

    // Merge all classes with proper conflict resolution
    merge_with_base("btn", classes.iter().map(|s| s.as_str()))
}

/// Button component with children (more flexible)
#[component]
pub fn Button(
    #[prop(into)] children: Children,
    #[prop(optional)] color: Option<Color>,
    #[prop(optional)] size: Option<Size>,
    #[prop(optional)] variant: Option<Variant>,
    #[prop(optional)] state: Option<State>,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional)] glass: bool,
    #[prop(optional)] no_animation: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    // #[prop(optional)] #[prop(attrs)] attrs: Vec<(&'static str, &'static str)>, // TODO: Implement proper attrs handling
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let classes = get_button_classes(
        color,
        size,
        variant,
        state,
        square,
        circle,
        glass,
        no_animation,
    );

    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base(&classes, [custom_class.as_str()])
    } else {
        classes
    };

    view! {
        <button
            class=final_classes
            disabled=disabled
            r#type="button"
            // ..attrs // TODO: Implement proper attrs handling
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

/// Simple button component with text content
#[component]
pub fn SimpleButton(
    text: String,
    #[prop(optional)] color: Option<Color>,
    #[prop(optional)] size: Option<Size>,
    #[prop(optional)] variant: Option<Variant>,
    #[prop(optional)] state: Option<State>,
    #[prop(optional)] square: bool,
    #[prop(optional)] circle: bool,
    #[prop(optional)] glass: bool,
    #[prop(optional)] no_animation: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    // #[prop(optional)] #[prop(attrs)] attrs: Vec<(&'static str, &'static str)>, // TODO: Implement proper attrs handling
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let classes = get_button_classes(
        color,
        size,
        variant,
        state,
        square,
        circle,
        glass,
        no_animation,
    );

    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base(&classes, [custom_class.as_str()])
    } else {
        classes
    };

    view! {
        <button
            class=final_classes
            disabled=disabled
            r#type="button"
            // ..attrs // TODO: Implement proper attrs handling
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {text}
        </button>
    }
}
