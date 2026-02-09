//! State variants for DaisyUI components
//!
//! Provides a shared State enum that can be used across
//! multiple DaisyUI components for consistent state handling.

/// State variants for DaisyUI components
///
/// These represent common interactive states that components can be in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    /// Default state (no specific state class)
    #[default]
    Default,

    /// Active state (component is currently active)
    Active,

    /// Disabled state (component is disabled)
    Disabled,

    /// Loading state (component is in loading state)
    Loading,

    /// Hover state (component is being hovered)
    Hover,

    /// Focus state (component has focus)
    Focus,
}

impl State {
    /// Converts the state variant to its corresponding DaisyUI class suffix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::state::State;
    ///
    /// assert_eq!(State::Active.class_suffix(), "active");
    /// assert_eq!(State::Disabled.class_suffix(), "disabled");
    /// assert_eq!(State::Default.class_suffix(), "");
    /// ```
    pub fn class_suffix(&self) -> &'static str {
        match self {
            State::Default => "",
            State::Active => "active",
            State::Disabled => "disabled",
            State::Loading => "loading",
            State::Hover => "hover",
            State::Focus => "focus",
        }
    }

    /// Creates a state class for a given component type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::state::State;
    ///
    /// assert_eq!(State::Active.to_class("btn"), "btn-active");
    /// assert_eq!(State::Disabled.to_class("btn"), "btn-disabled");
    /// assert_eq!(State::Default.to_class("btn"), "btn");
    /// ```
    pub fn to_class(&self, component: &str) -> String {
        let suffix = self.class_suffix();
        if suffix.is_empty() {
            component.to_string()
        } else {
            format!("{}-{}", component, suffix)
        }
    }

    /// Creates a state class with optional prefix configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::state::State;
    ///
    /// assert_eq!(State::Active.to_class_with_prefix("btn", Some("d-")), "d-btn-active");
    /// assert_eq!(State::Disabled.to_class_with_prefix("btn", None), "btn-disabled");
    /// ```
    pub fn to_class_with_prefix(&self, component: &str, prefix: Option<&str>) -> String {
        let base_class = self.to_class(component);
        if let Some(p) = prefix {
            format!("{}{}", p, base_class)
        } else {
            base_class
        }
    }

    /// Checks if this state represents an interactive state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::state::State;
    ///
    /// assert!(State::Active.is_interactive());
    /// assert!(!State::Disabled.is_interactive());
    /// ```
    pub fn is_interactive(&self) -> bool {
        matches!(self, State::Active | State::Hover | State::Focus)
    }

    /// Checks if this state represents a non-interactive state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::variants::state::State;
    ///
    /// assert!(State::Disabled.is_non_interactive());
    /// assert!(State::Loading.is_non_interactive());
    /// assert!(!State::Active.is_non_interactive());
    /// ```
    pub fn is_non_interactive(&self) -> bool {
        matches!(self, State::Disabled | State::Loading)
    }
}

/// Converts a State to a string representation
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Default => write!(f, "default"),
            State::Active => write!(f, "active"),
            State::Disabled => write!(f, "disabled"),
            State::Loading => write!(f, "loading"),
            State::Hover => write!(f, "hover"),
            State::Focus => write!(f, "focus"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_class_suffix() {
        assert_eq!(State::Default.class_suffix(), "");
        assert_eq!(State::Active.class_suffix(), "active");
        assert_eq!(State::Disabled.class_suffix(), "disabled");
        assert_eq!(State::Loading.class_suffix(), "loading");
        assert_eq!(State::Hover.class_suffix(), "hover");
        assert_eq!(State::Focus.class_suffix(), "focus");
    }

    #[test]
    fn test_state_to_class() {
        assert_eq!(State::Default.to_class("btn"), "btn");
        assert_eq!(State::Active.to_class("btn"), "btn-active");
        assert_eq!(State::Disabled.to_class("btn"), "btn-disabled");
        assert_eq!(State::Loading.to_class("btn"), "btn-loading");
    }

    #[test]
    fn test_state_to_class_with_prefix() {
        assert_eq!(
            State::Active.to_class_with_prefix("btn", Some("d-")),
            "d-btn-active"
        );
        assert_eq!(
            State::Disabled.to_class_with_prefix("btn", None),
            "btn-disabled"
        );
        assert_eq!(
            State::Loading.to_class_with_prefix("alert", Some("tw:")),
            "tw:alert-loading"
        );
    }

    #[test]
    fn test_state_interactive() {
        assert!(State::Active.is_interactive());
        assert!(State::Hover.is_interactive());
        assert!(State::Focus.is_interactive());
        assert!(!State::Disabled.is_interactive());
        assert!(!State::Loading.is_interactive());
        assert!(!State::Default.is_interactive());
    }

    #[test]
    fn test_state_non_interactive() {
        assert!(State::Disabled.is_non_interactive());
        assert!(State::Loading.is_non_interactive());
        assert!(!State::Active.is_non_interactive());
        assert!(!State::Hover.is_non_interactive());
        assert!(!State::Focus.is_non_interactive());
        assert!(!State::Default.is_non_interactive());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(State::Active.to_string(), "active");
        assert_eq!(State::Disabled.to_string(), "disabled");
        assert_eq!(State::Default.to_string(), "default");
    }
}
