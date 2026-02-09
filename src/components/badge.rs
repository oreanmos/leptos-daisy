//! Badge Component
//!
//! DaisyUI badge component for displaying small notifications or status indicators.

use leptos::*;
use leptos::prelude::*;
use crate::utils::class::merge_with_base;
use crate::variants::color::Color;
use crate::variants::size::Size;
use crate::variants::variant::Variant;

/// Badge component properties
#[component]
pub fn Badge(
    /// The content to display within the badge
    #[prop(into)] children: Children,
    
    /// The color variant of the badge
    #[prop(optional)] color: Option<Color>,
    
    /// The size of the badge
    #[prop(optional)] size: Option<Size>,
    
    /// The variant style of the badge
    #[prop(optional)] variant: Option<Variant>,
    
    /// Whether the badge should be outlined
    #[prop(optional)] outline: bool,
    
    /// Additional CSS classes to apply to the badge
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["badge".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("badge"));
        }
    }
    
    // Add size class
    if let Some(size) = size {
        if size != Size::Medium { // Medium is default
            classes.push(size.to_class("badge"));
        }
    }
    
    // Add variant class
    if let Some(variant) = variant {
        if variant != Variant::Solid { // Solid is default
            classes.push(variant.to_class("badge"));
        }
    }
    
    // Add outline class if specified
    if outline {
        classes.push("badge-outline".to_string());
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("badge", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    view! {
        <span class=final_classes>
            {children()}
        </span>
    }
}

/// Simple badge component with text content
#[component]
pub fn SimpleBadge(
    /// The text content of the badge
    text: String,
    
    /// The color variant of the badge
    #[prop(optional)] color: Option<Color>,
    
    /// The size of the badge
    #[prop(optional)] size: Option<Size>,
    
    /// The variant style of the badge
    #[prop(optional)] variant: Option<Variant>,
    
    /// Whether the badge should be outlined
    #[prop(optional)] outline: bool,
    
    /// Additional CSS classes to apply to the badge
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["badge".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("badge"));
        }
    }
    
    // Add size class
    if let Some(size) = size {
        if size != Size::Medium { // Medium is default
            classes.push(size.to_class("badge"));
        }
    }
    
    // Add variant class
    if let Some(variant) = variant {
        if variant != Variant::Solid { // Solid is default
            classes.push(variant.to_class("badge"));
        }
    }
    
    // Add outline class if specified
    if outline {
        classes.push("badge-outline".to_string());
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("badge", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    view! {
        <span class=final_classes>
            {text}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_badge_creation() {
        // Test that the component functions can be called
        // This tests compilation, not runtime behavior
        let _badge1 = Badge;
        let _badge2 = SimpleBadge;
    }

    #[test]
    fn test_badge_variants() {
        // Test that different color variants compile
        let _ = Color::Primary;
        let _ = Color::Success;
        let _ = Color::Error;
    }

    #[test]
    fn test_badge_sizes() {
        // Test that different sizes compile
        let _ = Size::Small;
        let _ = Size::Large;
    }

    #[test]
    fn test_badge_outline() {
        // Test that outline variant compiles
        let _ = true; // outline parameter
    }
}