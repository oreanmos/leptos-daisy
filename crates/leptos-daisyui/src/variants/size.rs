//! Shared size variants matching daisyUI's size scale.

/// Size variants for daisyUI components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Size {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl Size {
    /// The suffix used in class names, e.g. `"sm"`, `"lg"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "xs",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::ExtraLarge => "xl",
        }
    }

    /// Build `"{component}-{size}"`, e.g. `"btn-lg"`.
    pub fn class(&self, component: &str) -> String {
        format!("{component}-{}", self.as_str())
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
