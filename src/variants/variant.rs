//! Variant styles for DaisyUI components
//!
//! Provides a shared Variant enum that can be used across
//! multiple DaisyUI components for consistent styling variants.

/// Variant styles for DaisyUI components
///
/// These represent different visual styles that components can have.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// Default variant (solid background)
    #[default]
    Solid,

    /// Outline variant (border with transparent background)
    Outline,

    /// Ghost variant (minimal styling, often used for subtle buttons)
    Ghost,

    /// Link variant (text-only, looks like a link)
    Link,

    /// Soft variant (subtle background with softer colors)
    Soft,

    /// Dash variant (dashed border)
    Dash,
}

impl Variant {
    /// Converts the variant to its corresponding DaisyUI class suffix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::variant::Variant;
    ///
    /// assert_eq!(Variant::Outline.class_suffix(), "outline");
    /// assert_eq!(Variant::Ghost.class_suffix(), "ghost");
    /// assert_eq!(Variant::Solid.class_suffix(), "");
    /// ```
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Variant::Solid => "",
            Variant::Outline => "outline",
            Variant::Ghost => "ghost",
            Variant::Link => "link",
            Variant::Soft => "soft",
            Variant::Dash => "dash",
        }
    }

    /// Creates a variant class for a given component type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::variant::Variant;
    ///
    /// assert_eq!(Variant::Outline.to_class("btn"), "btn-outline");
    /// assert_eq!(Variant::Ghost.to_class("btn"), "btn-ghost");
    /// assert_eq!(Variant::Solid.to_class("btn"), "btn");
    /// ```
    pub fn to_class(&self, component: &str) -> String {
        let suffix = self.class_suffix();
        if suffix.is_empty() {
            component.to_string()
        } else {
            format!("{}-{}", component, suffix)
        }
    }

    /// Creates a variant class with optional prefix configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::variant::Variant;
    ///
    /// assert_eq!(Variant::Outline.to_class_with_prefix("btn", Some("d-")), "d-btn-outline");
    /// assert_eq!(Variant::Ghost.to_class_with_prefix("btn", None), "btn-ghost");
    /// ```
    pub fn to_class_with_prefix(&self, component: &str, prefix: Option<&str>) -> String {
        let base_class = self.to_class(component);
        if let Some(p) = prefix {
            format!("{}{}", p, base_class)
        } else {
            base_class
        }
    }

    /// Checks if this variant has a background (not transparent)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::variant::Variant;
    ///
    /// assert!(Variant::Solid.has_background());
    /// assert!(Variant::Soft.has_background());
    /// assert!(!Variant::Outline.has_background());
    /// assert!(!Variant::Ghost.has_background());
    /// assert!(!Variant::Link.has_background());
    /// ```
    pub fn has_background(&self) -> bool {
        matches!(self, Variant::Solid | Variant::Soft)
    }

    /// Checks if this variant is text-only (no background)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::variant::Variant;
    ///
    /// assert!(Variant::Link.is_text_only());
    /// assert!(Variant::Ghost.is_text_only());
    /// assert!(!Variant::Solid.is_text_only());
    /// ```
    pub fn is_text_only(&self) -> bool {
        matches!(self, Variant::Link | Variant::Ghost)
    }
}

/// Converts a Variant to a string representation
impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Solid => write!(f, "solid"),
            Variant::Outline => write!(f, "outline"),
            Variant::Ghost => write!(f, "ghost"),
            Variant::Link => write!(f, "link"),
            Variant::Soft => write!(f, "soft"),
            Variant::Dash => write!(f, "dash"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_class_suffix() {
        assert_eq!(Variant::Solid.class_suffix(), "");
        assert_eq!(Variant::Outline.class_suffix(), "outline");
        assert_eq!(Variant::Ghost.class_suffix(), "ghost");
        assert_eq!(Variant::Link.class_suffix(), "link");
        assert_eq!(Variant::Soft.class_suffix(), "soft");
        assert_eq!(Variant::Dash.class_suffix(), "dash");
    }

    #[test]
    fn test_variant_to_class() {
        assert_eq!(Variant::Solid.to_class("btn"), "btn");
        assert_eq!(Variant::Outline.to_class("btn"), "btn-outline");
        assert_eq!(Variant::Ghost.to_class("btn"), "btn-ghost");
        assert_eq!(Variant::Link.to_class("alert"), "alert-link");
        assert_eq!(Variant::Soft.to_class("btn"), "btn-soft");
    }

    #[test]
    fn test_variant_to_class_with_prefix() {
        assert_eq!(
            Variant::Outline.to_class_with_prefix("btn", Some("d-")),
            "d-btn-outline"
        );
        assert_eq!(
            Variant::Ghost.to_class_with_prefix("btn", None),
            "btn-ghost"
        );
        assert_eq!(
            Variant::Link.to_class_with_prefix("alert", Some("tw:")),
            "tw:alert-link"
        );
    }

    #[test]
    fn test_variant_background() {
        assert!(Variant::Solid.has_background());
        assert!(Variant::Soft.has_background());
        assert!(!Variant::Outline.has_background());
        assert!(!Variant::Ghost.has_background());
        assert!(!Variant::Link.has_background());
        assert!(!Variant::Dash.has_background());
    }

    #[test]
    fn test_variant_text_only() {
        assert!(Variant::Link.is_text_only());
        assert!(Variant::Ghost.is_text_only());
        assert!(!Variant::Solid.is_text_only());
        assert!(!Variant::Outline.is_text_only());
        assert!(!Variant::Soft.is_text_only());
        assert!(!Variant::Dash.is_text_only());
    }

    #[test]
    fn test_variant_display() {
        assert_eq!(Variant::Solid.to_string(), "solid");
        assert_eq!(Variant::Outline.to_string(), "outline");
        assert_eq!(Variant::Ghost.to_string(), "ghost");
    }
}
