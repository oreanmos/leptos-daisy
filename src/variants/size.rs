//! Size variants for DaisyUI components
//!
//! Provides a shared Size enum that can be used across
//! multiple DaisyUI components for consistent sizing.

/// Size variants for DaisyUI components
///
/// These correspond to the standard DaisyUI size classes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Extra small size
    ExtraSmall,

    /// Small size
    Small,

    /// Medium size (default)
    #[default]
    Medium,

    /// Large size
    Large,

    /// Extra large size
    ExtraLarge,
}

impl Size {
    /// Converts the size variant to its corresponding DaisyUI class suffix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::size::Size;
    ///
    /// assert_eq!(Size::Small.class_suffix(), "sm");
    /// assert_eq!(Size::Large.class_suffix(), "lg");
    /// assert_eq!(Size::Medium.class_suffix(), "md");
    /// ```
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Size::ExtraSmall => "xs",
            Size::Small => "sm",
            Size::Medium => "md",
            Size::Large => "lg",
            Size::ExtraLarge => "xl",
        }
    }

    /// Creates a size class for a given component type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::size::Size;
    ///
    /// assert_eq!(Size::Small.to_class("btn"), "btn-sm");
    /// assert_eq!(Size::Large.to_class("text"), "text-lg");
    /// assert_eq!(Size::Medium.to_class("btn"), "btn-md");
    /// ```
    pub fn to_class(&self, component: &str) -> String {
        format!("{}-{}", component, self.class_suffix())
    }

    /// Creates a size class with optional prefix configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::size::Size;
    ///
    /// assert_eq!(Size::Small.to_class_with_prefix("btn", Some("d-")), "d-btn-sm");
    /// assert_eq!(Size::Large.to_class_with_prefix("text", None), "text-lg");
    /// ```
    pub fn to_class_with_prefix(&self, component: &str, prefix: Option<&str>) -> String {
        let base_class = self.to_class(component);
        if let Some(p) = prefix {
            format!("{}{}", p, base_class)
        } else {
            base_class
        }
    }
}

/// Converts a Size to a string representation
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::ExtraSmall => write!(f, "xs"),
            Size::Small => write!(f, "sm"),
            Size::Medium => write!(f, "md"),
            Size::Large => write!(f, "lg"),
            Size::ExtraLarge => write!(f, "xl"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_class_suffix() {
        assert_eq!(Size::ExtraSmall.class_suffix(), "xs");
        assert_eq!(Size::Small.class_suffix(), "sm");
        assert_eq!(Size::Medium.class_suffix(), "md");
        assert_eq!(Size::Large.class_suffix(), "lg");
        assert_eq!(Size::ExtraLarge.class_suffix(), "xl");
    }

    #[test]
    fn test_size_to_class() {
        assert_eq!(Size::ExtraSmall.to_class("btn"), "btn-xs");
        assert_eq!(Size::Small.to_class("btn"), "btn-sm");
        assert_eq!(Size::Medium.to_class("btn"), "btn-md");
        assert_eq!(Size::Large.to_class("btn"), "btn-lg");
        assert_eq!(Size::ExtraLarge.to_class("btn"), "btn-xl");
    }

    #[test]
    fn test_size_to_class_with_prefix() {
        assert_eq!(
            Size::Small.to_class_with_prefix("btn", Some("d-")),
            "d-btn-sm"
        );
        assert_eq!(Size::Large.to_class_with_prefix("text", None), "text-lg");
        assert_eq!(
            Size::ExtraLarge.to_class_with_prefix("btn", Some("tw:")),
            "tw:btn-xl"
        );
    }

    #[test]
    fn test_size_display() {
        assert_eq!(Size::Small.to_string(), "sm");
        assert_eq!(Size::Large.to_string(), "lg");
        assert_eq!(Size::Medium.to_string(), "md");
    }
}
