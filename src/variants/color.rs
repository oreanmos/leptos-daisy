//! Color variants for DaisyUI components
//!
//! Provides a shared Color enum that can be used across
//! multiple DaisyUI components for consistent color theming.

/// Color variants for DaisyUI components
///
/// These correspond to the standard DaisyUI color palette.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Color {
    /// Default color (no specific color class)
    #[default]
    Default,

    /// Primary color variant
    Primary,

    /// Secondary color variant
    Secondary,

    /// Accent color variant
    Accent,

    /// Neutral color variant
    Neutral,

    /// Info color variant (blue)
    Info,

    /// Success color variant (green)
    Success,

    /// Warning color variant (yellow/orange)
    Warning,

    /// Error color variant (red)
    Error,
}

impl Color {
    /// Converts the color variant to its corresponding DaisyUI class prefix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::color::Color;
    ///
    /// assert_eq!(Color::Primary.class_prefix(), "primary");
    /// assert_eq!(Color::Success.class_prefix(), "success");
    /// assert_eq!(Color::Default.class_prefix(), "");
    /// ```
    pub fn class_prefix(&self) -> &'static str {
        match self {
            Color::Default => "",
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Accent => "accent",
            Color::Neutral => "neutral",
            Color::Info => "info",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Error => "error",
        }
    }

    /// Creates a color class for a given component type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::color::Color;
    ///
    /// assert_eq!(Color::Primary.to_class("btn"), "btn-primary");
    /// assert_eq!(Color::Success.to_class("alert"), "alert-success");
    /// assert_eq!(Color::Default.to_class("btn"), "btn");
    /// ```
    pub fn to_class(&self, component: &str) -> String {
        let prefix = self.class_prefix();
        if prefix.is_empty() {
            component.to_string()
        } else {
            format!("{}-{}", component, prefix)
        }
    }

    /// Creates a color class with optional prefix configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::color::Color;
    ///
    /// assert_eq!(Color::Primary.to_class_with_prefix("btn", Some("d-")), "d-btn-primary");
    /// assert_eq!(Color::Success.to_class_with_prefix("alert", None), "alert-success");
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

/// Converts a Color to a string representation
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Default => write!(f, "default"),
            Color::Primary => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Accent => write!(f, "accent"),
            Color::Neutral => write!(f, "neutral"),
            Color::Info => write!(f, "info"),
            Color::Success => write!(f, "success"),
            Color::Warning => write!(f, "warning"),
            Color::Error => write!(f, "error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_class_prefix() {
        assert_eq!(Color::Default.class_prefix(), "");
        assert_eq!(Color::Primary.class_prefix(), "primary");
        assert_eq!(Color::Secondary.class_prefix(), "secondary");
        assert_eq!(Color::Accent.class_prefix(), "accent");
        assert_eq!(Color::Neutral.class_prefix(), "neutral");
        assert_eq!(Color::Info.class_prefix(), "info");
        assert_eq!(Color::Success.class_prefix(), "success");
        assert_eq!(Color::Warning.class_prefix(), "warning");
        assert_eq!(Color::Error.class_prefix(), "error");
    }

    #[test]
    fn test_color_to_class() {
        assert_eq!(Color::Default.to_class("btn"), "btn");
        assert_eq!(Color::Primary.to_class("btn"), "btn-primary");
        assert_eq!(Color::Success.to_class("alert"), "alert-success");
        assert_eq!(Color::Error.to_class("badge"), "badge-error");
    }

    #[test]
    fn test_color_to_class_with_prefix() {
        assert_eq!(
            Color::Primary.to_class_with_prefix("btn", Some("d-")),
            "d-btn-primary"
        );
        assert_eq!(
            Color::Success.to_class_with_prefix("alert", None),
            "alert-success"
        );
        assert_eq!(
            Color::Default.to_class_with_prefix("btn", Some("tw:")),
            "tw:btn"
        );
    }

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Primary.to_string(), "primary");
        assert_eq!(Color::Success.to_string(), "success");
        assert_eq!(Color::Default.to_string(), "default");
    }
}
