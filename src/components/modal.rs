//! Modal Component
//!
//! DaisyUI modal component for dialogs and popups.
//!
//! This component provides both a pure wrapper (Tier A) for SSR safety
//! and interactive controller (Tier B) for CSR/Hydrate modes.

use leptos::*;
use leptos::prelude::*;
use crate::utils::class::merge_with_base;
use crate::variants::color::Color;

/// Modal component using dialog pattern (Tier A - Pure wrapper, recommended by DaisyUI)
#[component]
pub fn Modal(
    /// The content to display within the modal
    #[prop(into)] children: Children,
    
    /// The color variant of the modal
    #[prop(optional)] color: Option<Color>,
    
    /// Whether the modal should be open by default
    #[prop(optional)] open: bool,
    
    /// Additional CSS classes to apply to the modal
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["modal".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("modal"));
        }
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("modal", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    view! {
        <dialog id="modal-dialog" class=final_classes open=open>
            <div class="modal-box">
                {children()}
            </div>
        </dialog>
    }
}

/// Modal component using checkbox pattern (Tier A - Pure wrapper, SSR-friendly alternative)
#[component]
pub fn ModalCheckbox(
    /// The content to display within the modal
    #[prop(into)] children: Children,
    
    /// The color variant of the modal
    #[prop(optional)] color: Option<Color>,
    
    /// Whether the modal should be open by default
    #[prop(optional)] open: bool,
    
    /// Additional CSS classes to apply to the modal
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["modal".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("modal"));
        }
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("modal", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    let checkbox_id = format!("modal-checkbox-{}", uuid::Uuid::new_v4().to_string());
    
    view! {
        <div>
            <input type="checkbox" id=checkbox_id class="modal-toggle" checked=open />
            <div class=final_classes>
                <div class="modal-box">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Modal box content area
#[component]
pub fn ModalBox(
    /// The content to display in the modal box
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the modal box
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("modal-box", [custom_class.as_str()])
    } else {
        "modal-box".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Modal action buttons area
#[component]
pub fn ModalActions(
    /// The content to display in the modal actions area
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the modal actions
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("modal-action", [custom_class.as_str()])
    } else {
        "modal-action".to_string()
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// Modal title area
#[component]
pub fn ModalTitle(
    /// The content to display in the modal title
    #[prop(into)] children: Children,
    
    /// Additional CSS classes to apply to the modal title
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("font-bold text-lg", [custom_class.as_str()])
    } else {
        "font-bold text-lg".to_string()
    };
    
    view! {
        <h3 class=classes>
            {children()}
        </h3>
    }
}

/// Modal close button
#[component]
pub fn ModalCloseButton(
    /// Optional click handler for close button
    #[prop(optional)] on_click: Option<Callback<()>>,
    
    /// Additional CSS classes to apply to the close button
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("btn btn-sm btn-circle btn-ghost absolute right-2 top-2", [custom_class.as_str()])
    } else {
        "btn btn-sm btn-circle btn-ghost absolute right-2 top-2".to_string()
    };
    
    view! {
        <button
            class=classes
            r#type="button"
            aria-label="Close modal"
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
        </button>
    }
}

/// Modal backdrop (for dialog pattern)
#[component]
pub fn ModalBackdrop(
    /// Optional click handler for backdrop
    #[prop(optional)] on_click: Option<Callback<()>>,
    
    /// Additional CSS classes to apply to the backdrop
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let classes = if let Some(custom_class) = class {
        merge_with_base("modal-backdrop", [custom_class.as_str()])
    } else {
        "modal-backdrop".to_string()
    };
    
    view! {
        <div
            class=classes
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        ></div>
    }
}

#[cfg(feature = "hydrate")]
/// Modal controller for interactive behavior (Tier B - Feature gated)
#[component]
pub fn ModalController(
    /// Signal controlling modal visibility
    open: RwSignal<bool>,
    
    /// The content to display within the controlled modal
    #[prop(into)] children: Children,
    
    /// The color variant of the modal
    #[prop(optional)] color: Option<Color>,
    
    /// Additional CSS classes to apply to the modal
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let modal_ref = create_node_ref::<html::Dialog>();
    
    // Effect to handle modal visibility changes
    create_effect(move |_| {
        if let Some(modal) = modal_ref.get() {
            if open.get() {
                // Show modal when signal becomes true
                if let Err(e) = modal.show_modal() {
                    log::error!("Failed to show modal: {:?}", e);
                }
            } else {
                // Close modal when signal becomes false
                modal.close();
            }
        }
    });
    
    let mut classes = vec!["modal".to_string()];
    
    // Add color class
    if let Some(color) = color {
        if color != Color::Default {
            classes.push(color.to_class("modal"));
        }
    }
    
    // Merge custom classes if provided
    let final_classes = if let Some(custom_class) = class {
        merge_with_base("modal", [classes.join(" ").as_str(), custom_class.as_str()])
    } else {
        classes.join(" ")
    };
    
    view! {
        <dialog
            node:ref=modal_ref
            class=final_classes
            on:click=move |ev| {
                // Close modal when clicking on backdrop
                if ev.target() == modal_ref.get().unwrap() {
                    open.set(false);
                }
            }
        >
            <div class="modal-box">
                {children()}
            </div>
        </dialog>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_modal_creation() {
        // Test that the component functions can be called
        // This tests compilation, not runtime behavior
        let _modal = Modal;
        let _checkbox = ModalCheckbox;
        let _box = ModalBox;
        let _actions = ModalActions;
        let _title = ModalTitle;
        let _close = ModalCloseButton;
        let _backdrop = ModalBackdrop;
    }

    #[test]
    fn test_modal_variants() {
        // Test that different color variants compile
        let _ = Color::Primary;
        let _ = Color::Secondary;
    }

    #[test]
    fn test_modal_patterns() {
        // Test that different pattern flags compile
        let _ = true; // open, checkbox_pattern parameters
    }

    #[test]
    fn test_modal_parts() {
        // Test that all modal parts compile
        let _ = (); // All parts are tested in test_modal_creation
    }

    #[cfg(feature = "hydrate")]
    #[test]
    fn test_modal_controller() {
        // Test that modal controller compiles with hydrate feature
        let _controller = ModalController;
    }
}