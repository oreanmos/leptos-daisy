//! Attribute handling utilities
//!
//! Provides helper functions for working with Leptos attributes.
//!
//! Note: In the current Leptos version, the Attribute type is not directly accessible,
//! so these utilities are simplified to work with basic attribute patterns.

/// Creates a basic attribute tuple for use with Leptos view macro
///
/// This is a convenience function for creating attribute tuples
/// when you need to pass attributes programmatically.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::attrs::attr;
///
/// let class_attr = attr("class", "btn btn-primary");
/// let disabled_attr = attr("disabled", "true");
/// ```
pub fn attr(name: impl Into<String>, value: impl Into<String>) -> (&'static str, String) {
    (Box::leak(name.into().into_boxed_str()), value.into())
}

/// Creates a boolean attribute tuple
///
/// This is useful for boolean HTML attributes like disabled, checked, etc.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::attrs::bool_attr;
///
/// let disabled_attr = bool_attr("disabled");
/// // Creates: ("disabled", "disabled")
/// ```
pub fn bool_attr(name: impl Into<String>) -> (&'static str, &'static str) {
    let name_str = name.into();
    let leaked = Box::leak(name_str.into_boxed_str());
    (leaked, leaked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_creation() {
        let attr1 = attr("class", "btn");
        assert_eq!(attr1.0, "class");
        assert_eq!(attr1.1, "btn");

        let bool_attr1 = bool_attr("disabled");
        assert_eq!(bool_attr1.0, "disabled");
        assert_eq!(bool_attr1.1, "disabled");
    }
}
