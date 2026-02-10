//! Shared visual style variants matching daisyUI modifiers.

/// Visual style variants (outline, ghost, soft, etc.).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variant {
    #[default]
    Solid,
    Outline,
    Ghost,
    Link,
    Soft,
    Dash,
}

impl Variant {
    /// The suffix used in class names, e.g. `"outline"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Solid => "",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Link => "link",
            Self::Soft => "soft",
            Self::Dash => "dash",
        }
    }

    /// Build `"{component}-{variant}"`, e.g. `"btn-outline"`.
    /// Returns empty string for `Solid` (default).
    pub fn class(&self, component: &str) -> String {
        let s = self.as_str();
        if s.is_empty() {
            String::new()
        } else {
            format!("{component}-{s}")
        }
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
