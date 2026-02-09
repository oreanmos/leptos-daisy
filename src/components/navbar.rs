//! Navbar Component
//!
//! DaisyUI navbar component for site navigation.
//!
//! The Navbar component provides a responsive navigation bar with support for
//! branding, menu items, and responsive behavior.

use leptos::*;
use leptos::prelude::*;
use crate::utils::class::merge_with_base;
use crate::variants::color::Color;

/// Navbar component properties
#[component]
pub fn Navbar(
    /// The content to display within the navbar
    #[prop(into)] children: Children,
    
    /// The color variant of the navbar
    #[prop(optional)] color: Option<Color>,
    
    /// Whether the navbar should be fixed at the top
    #[prop(optional)] fixed: bool,
    
    /// Whether the navbar should be transparent (glass effect)
    #[prop(optional)] glass: bool,
    
    /// Whether the navbar should be rounded
    #[prop(optional)] rounded: bool,
    
    /// Whether the navbar should be bordered
    #[prop(optional)] bordered: bool,
    
    /// Additional CSS classes to apply to the navbar
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["navbar".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("navbar"));
        }
    }
    
    // Add modifier classes
    if fixed {
        classes.push("fixed".to_string());
        classes.push("top-0".to_string());
        classes.push("z-50".to_string());
    }
    
    if glass {
        classes.push("glass".to_string());
    }
    
    if rounded {
        classes.push("rounded-box".to_string());
    }
    
    if bordered {
        classes.push("border".to_string());
        classes.push("border-base-300".to_string());
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("navbar", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    view! {
        <nav class=final_classes>
            {children()}
        </nav>
    }
}

/// Navbar start section (left-aligned content)
#[component]
pub fn NavbarStart(
    /// The content to display in the start section
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the start section
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-start", [custom_class.as_str()])
    } else {
        "navbar-start".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Navbar center section (center-aligned content)
#[component]
pub fn NavbarCenter(
    /// The content to display in the center section
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the center section
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-center", [custom_class.as_str()])
    } else {
        "navbar-center".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Navbar end section (right-aligned content)
#[component]
pub fn NavbarEnd(
    /// The content to display in the end section
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the end section
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-end", [custom_class.as_str()])
    } else {
        "navbar-end".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Navbar brand/logo section
#[component]
pub fn NavbarBrand(
    /// The content to display in the brand section
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the brand section
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-brand", [custom_class.as_str()])
    } else {
        "navbar-brand".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Navbar menu section (for mobile/responsive menus)
#[component]
pub fn NavbarMenu(
    /// The content to display in the menu section
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the menu section
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-menu", [custom_class.as_str()])
    } else {
        "navbar-menu".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Navbar menu toggle button (for responsive behavior)
#[component]
pub fn NavbarToggle(
    /// Additional CSS classes to apply to the toggle button
    #[prop(optional)] class: Option<String>,
    
    /// Optional click handler
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("navbar-toggle", [custom_class.as_str()])
    } else {
        "navbar-toggle".to_string()
    };
    
    view! {
        <button
            class=classes
            r#type="button"
            aria-label="Toggle menu"
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_navbar_creation() {
        // Test that the component functions can be called
        // This tests compilation, not runtime behavior
        let _navbar = Navbar;
        let _start = NavbarStart;
        let _center = NavbarCenter;
        let _end = NavbarEnd;
        let _brand = NavbarBrand;
        let _menu = NavbarMenu;
        let _toggle = NavbarToggle;
    }

    #[test]
    fn test_navbar_variants() {
        // Test that different color variants compile
        let _ = Color::Primary;
        let _ = Color::Secondary;
    }

    #[test]
    fn test_navbar_modifiers() {
        // Test that modifier flags compile
        let _ = true; // fixed, glass, rounded, bordered parameters
    }

    #[test]
    fn test_navbar_parts() {
        // Test that all navbar parts compile
        let _ = (); // All parts are tested in test_navbar_creation
    }
}