//! Leptos components for DaisyUI
//!
//! This crate provides Leptos implementations of all DaisyUI components.
//! DaisyUI is a popular Tailwind CSS component library with 65+ components.

#![warn(missing_docs)]
#![doc(html_logo_url = "https://daisyui.com/favicon.ico")]

/// Re-export common Leptos types for convenience
pub use leptos::*;

/// Components module containing all DaisyUI component implementations
pub mod components;

/// Utilities and helper functions for working with DaisyUI in Leptos
pub mod utils;

/// Variant enums for consistent API design across components
pub mod variants;

/// Prelude module for convenient imports
pub mod prelude;
