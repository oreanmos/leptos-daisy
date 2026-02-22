//! Shared color variants matching daisyUI's semantic palette.

/// Color variants for daisyUI components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}

impl Color {
    /// The suffix used in daisyUI class names, e.g. `"primary"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Accent => "accent",
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Build `"{component}-{color}"`, e.g. `"btn-primary"`.
    /// Returns empty string for `Default`.
    pub fn class(&self, component: &str) -> String {
        let s = self.as_str();
        if s.is_empty() {
            String::new()
        } else {
            format!("{component}-{s}")
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        assert_eq!(Color::Default.class("btn"), "");
        assert_eq!(Color::Primary.class("btn"), "btn-primary");
        assert_eq!(Color::Secondary.class("btn"), "btn-secondary");
        assert_eq!(Color::Accent.class("btn"), "btn-accent");
        assert_eq!(Color::Neutral.class("btn"), "btn-neutral");
        assert_eq!(Color::Info.class("btn"), "btn-info");
        assert_eq!(Color::Success.class("btn"), "btn-success");
        assert_eq!(Color::Warning.class("btn"), "btn-warning");
        assert_eq!(Color::Error.class("btn"), "btn-error");

        // Test with a different component prefix
        assert_eq!(Color::Primary.class("badge"), "badge-primary");
    }
}
