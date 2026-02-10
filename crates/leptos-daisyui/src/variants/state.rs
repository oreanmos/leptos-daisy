//! Shared interactive state variants.

/// Interactive state variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Default,
    Active,
    Disabled,
    Loading,
    Focus,
}

impl State {
    /// The suffix used in class names, e.g. `"active"`, `"loading"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Loading => "loading",
            Self::Focus => "focus",
        }
    }

    /// Build `"{component}-{state}"`, e.g. `"btn-active"`.
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

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
