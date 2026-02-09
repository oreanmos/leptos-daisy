//! ARIA attribute utilities
//!
//! Provides helper functions and constants for working with
//! ARIA (Accessible Rich Internet Applications) attributes.

// Note: Attribute type is not directly accessible in Leptos
// We'll use the attribute macros instead

/// Common ARIA roles
pub mod roles {
    pub const ALERT: &str = "alert";
    pub const BUTTON: &str = "button";
    pub const DIALOG: &str = "dialog";
    pub const LINK: &str = "link";
    pub const NAVIGATION: &str = "navigation";
    pub const PROGRESSBAR: &str = "progressbar";
    pub const TAB: &str = "tab";
    pub const TABLIST: &str = "tablist";
    pub const TABPANEL: &str = "tabpanel";
}

/// Common ARIA attributes
pub mod attributes {
    pub const ARIA_LABEL: &str = "aria-label";
    pub const ARIA_LABELLED_BY: &str = "aria-labelledby";
    pub const ARIA_DESCRIBED_BY: &str = "aria-describedby";
    pub const ARIA_HIDDEN: &str = "aria-hidden";
    pub const ARIA_DISABLED: &str = "aria-disabled";
    pub const ARIA_EXPANDED: &str = "aria-expanded";
    pub const ARIA_SELECTED: &str = "aria-selected";
    pub const ARIA_BUSY: &str = "aria-busy";
    pub const ARIA_LIVE: &str = "aria-live";
    pub const ARIA_ATOMIC: &str = "aria-atomic";
    pub const ARIA_CONTROLS: &str = "aria-controls";
}

/// Common ARIA values
pub mod values {
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const POLITE: &str = "polite";
    pub const ASSERTIVE: &str = "assertive";
}

/// Creates an ARIA attribute tuple for use with Leptos view macro
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::aria::aria_attr;
///
/// let label_attr = aria_attr("aria-label", "Close dialog");
/// let disabled_attr = aria_attr("aria-disabled", "true");
/// ```
pub fn aria_attr(name: impl Into<String>, value: impl Into<String>) -> (&'static str, String) {
    (Box::leak(name.into().into_boxed_str()), value.into())
}

/// Creates a boolean ARIA attribute tuple
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::aria::aria_bool_attr;
///
/// let expanded_attr = aria_bool_attr("aria-expanded", true);
/// // Creates: ("aria-expanded", "true")
/// ```
pub fn aria_bool_attr(name: impl Into<String>, value: bool) -> (&'static str, &'static str) {
    let name_str = name.into();
    let leaked_name = Box::leak(name_str.into_boxed_str());
    let value_str = if value { "true" } else { "false" };
    let leaked_value = Box::leak(value_str.to_string().into_boxed_str());
    (leaked_name, leaked_value)
}

/// Creates a collection of ARIA attribute tuples for common accessibility patterns.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::aria::aria_attributes;
///
/// let button_attrs = aria_attributes("button", Some("Close"), None);
/// // Creates: vec![("role", "button"), ("aria-label", "Close")]
/// ```
pub fn aria_attributes(
    role: impl Into<String>,
    label: Option<impl Into<String>>,
    described_by: Option<impl Into<String>>,
) -> Vec<(&'static str, String)> {
    let mut attrs = Vec::new();

    // Add role
    attrs.push(("role", role.into()));

    // Add label if provided
    if let Some(label_value) = label {
        attrs.push(("aria-label", label_value.into()));
    }

    // Add described_by if provided
    if let Some(described_by_value) = described_by {
        attrs.push(("aria-describedby", described_by_value.into()));
    }

    attrs
}

/// Creates ARIA attribute tuples for a dialog component.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::aria::dialog_aria_attrs;
///
/// let dialog_attrs = dialog_aria_attrs("my-dialog-title", "my-dialog-description");
/// // Creates: vec![("role", "dialog"), ("aria-labelledby", "my-dialog-title"), ("aria-describedby", "my-dialog-description")]
/// ```
pub fn dialog_aria_attrs(
    labelled_by: impl Into<String>,
    described_by: impl Into<String>,
) -> Vec<(&'static str, String)> {
    vec![
        ("role", "dialog".to_string()),
        ("aria-labelledby", labelled_by.into()),
        ("aria-describedby", described_by.into()),
    ]
}

/// Creates ARIA attribute tuples for a button component.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::aria::button_aria_attrs;
///
/// let button_attrs = button_aria_attrs(Some("Close dialog"), None);
/// // Creates: vec![("role", "button"), ("aria-label", "Close dialog")]
/// ```
pub fn button_aria_attrs(
    label: Option<impl Into<String>>,
    disabled: Option<bool>,
) -> Vec<(&'static str, String)> {
    let mut attrs = vec![("role", "button".to_string())];

    if let Some(label_value) = label {
        attrs.push(("aria-label", label_value.into()));
    }

    if let Some(disabled_value) = disabled {
        if disabled_value {
            attrs.push(("aria-disabled", "true".to_string()));
            attrs.push(("tabindex", "-1".to_string()));
        }
    }

    attrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aria_attr() {
        let attr = aria_attr("aria-label", "test");
        assert_eq!(attr.0, "aria-label");
        assert_eq!(attr.1, "test");
    }

    #[test]
    fn test_aria_bool_attr() {
        let attr_true = aria_bool_attr("aria-expanded", true);
        assert_eq!(attr_true.1, "true");

        let attr_false = aria_bool_attr("aria-expanded", false);
        assert_eq!(attr_false.1, "false");
    }

    #[test]
    fn test_aria_attributes() {
        let attrs = aria_attributes("button", Some("Close"), Some("description"));
        assert_eq!(attrs.len(), 3);
        assert_eq!(attrs[0].0, "role");
        assert_eq!(attrs[1].0, "aria-label");
        assert_eq!(attrs[2].0, "aria-describedby");
    }

    #[test]
    fn test_dialog_aria_attrs() {
        let attrs = dialog_aria_attrs("title-id", "desc-id");
        assert_eq!(attrs.len(), 3);
        assert_eq!(attrs[0].1, "dialog");
        assert_eq!(attrs[1].1, "title-id");
        assert_eq!(attrs[2].1, "desc-id");
    }

    #[test]
    fn test_button_aria_attrs() {
        let attrs = button_aria_attrs(Some("Click me"), Some(true));
        assert_eq!(attrs.len(), 4); // role, aria-label, aria-disabled, tabindex
        assert_eq!(attrs[0].1, "button");
        assert_eq!(attrs[1].1, "Click me");
        assert_eq!(attrs[2].1, "true");
        assert_eq!(attrs[3].1, "-1");
    }
}
