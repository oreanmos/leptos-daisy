//! Prelude module for leptos-daisyui
//!
//! This module re-exports commonly used types, traits, and functions
//! for convenient access when using the leptos-daisyui library.

// Re-export common Leptos types for convenience
pub use leptos::*;

// Re-export our utility modules
pub use crate::utils::*;
pub use crate::variants::*;

// Re-export components
pub use crate::components::*;

// Re-export specific commonly used types
pub use crate::variants::color::Color;
pub use crate::variants::size::Size;
pub use crate::variants::state::State;
pub use crate::variants::variant::Variant;

pub use crate::utils::aria::{
    aria_attr, aria_attributes, aria_bool_attr, button_aria_attrs, dialog_aria_attrs,
};
pub use crate::utils::attrs::{attr, bool_attr};
pub use crate::utils::class::{merge_classes, merge_with_base, simple_merge};
pub use crate::utils::config::{
    DaisyConfig, config_class, provide_daisy_config, use_daisy_config, use_daisy_config_or_default,
};
