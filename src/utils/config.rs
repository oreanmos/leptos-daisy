//! DaisyUI configuration utilities
//!
//! Provides configuration structures and helpers for working with
//! DaisyUI prefixing and Tailwind configuration.

/// DaisyUI configuration structure
///
/// This configuration allows customization of DaisyUI and Tailwind
/// prefixing behavior throughout the application.
#[derive(Clone, Debug, Default)]
pub struct DaisyConfig {
    /// DaisyUI class prefix (e.g., "d-" for "d-btn" instead of "btn")
    pub daisy_prefix: Option<String>,

    /// Tailwind utility prefix (e.g., "tw:" for "tw:text-sm" instead of "text-sm")
    pub tailwind_prefix: Option<String>,

    /// Whether to apply prefixing to theme-controller (special case in DaisyUI)
    pub prefix_theme_controller: bool,
}

impl DaisyConfig {
    /// Creates a new DaisyConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new DaisyConfig with specified prefixes
    pub fn with_prefixes(daisy_prefix: Option<String>, tailwind_prefix: Option<String>) -> Self {
        Self {
            daisy_prefix,
            tailwind_prefix,
            prefix_theme_controller: false, // Default to false as per DaisyUI docs
        }
    }

    /// Applies the configuration to a DaisyUI class name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::utils::config::DaisyConfig;
    ///
    /// let config = DaisyConfig::with_prefixes(Some("d-".to_string()), Some("tw:".to_string()));
    /// let prefixed = config.apply_to_class("btn");
    /// assert_eq!(prefixed, "tw:d-btn");
    /// ```
    pub fn apply_to_class(&self, class: &str) -> String {
        // Special handling for theme-controller
        if class == "theme-controller" && !self.prefix_theme_controller {
            return class.to_string();
        }

        let mut result = String::new();

        // Add Tailwind prefix if configured
        if let Some(tw_prefix) = &self.tailwind_prefix {
            result.push_str(tw_prefix);
        }

        // Add DaisyUI prefix if configured
        if let Some(d_prefix) = &self.daisy_prefix {
            result.push_str(d_prefix);
        }

        // Add the class name
        result.push_str(class);

        result
    }

    /// Applies the configuration to multiple class names
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::utils::config::DaisyConfig;
    ///
    /// let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
    /// let prefixed = config.apply_to_classes(["btn", "btn-primary"]);
    /// assert_eq!(prefixed, "d-btn d-btn-primary");
    /// ```
    pub fn apply_to_classes(&self, classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
        classes
            .into_iter()
            .map(|class| self.apply_to_class(class.as_ref()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Creates a prefixed version of a class if prefixes are configured
    ///
    /// This is useful for conditional prefixing where you want to
    /// apply prefixes only when they're configured.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leptos_daisy::utils::config::DaisyConfig;
    ///
    /// let config = DaisyConfig::new();
    /// let result = config.prefixed_class("btn");
    /// assert_eq!(result, "btn"); // No prefixes configured
    ///
    /// let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
    /// let result = config.prefixed_class("btn");
    /// assert_eq!(result, "d-btn");
    /// ```
    pub fn prefixed_class(&self, class: &str) -> String {
        if self.daisy_prefix.is_none() && self.tailwind_prefix.is_none() {
            return class.to_string();
        }
        self.apply_to_class(class)
    }
}

use leptos::context::{provide_context, use_context};
/// Provides a context provider for DaisyConfig
///
/// This allows components to access the configuration
/// through Leptos' context system.
use leptos::*;

/// Creates a DaisyConfig context provider
///
/// # Examples
///
/// ```rust
/// use leptos::*;
/// use leptos_daisy::utils::config::{DaisyConfig, provide_daisy_config};
///
/// #[component]
/// fn App() -> impl IntoView {
///     let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
///     provide_daisy_config(config);
///     // ... rest of app
/// }
/// ```
pub fn provide_daisy_config(config: DaisyConfig) -> DaisyConfig {
    provide_context(config.clone());
    config
}

/// Gets the current DaisyConfig from context
///
/// # Examples
///
/// ```rust
/// use leptos::*;
/// use leptos_daisy::utils::config::use_daisy_config;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let config = use_daisy_config();
///     // Use config...
/// }
/// ```
pub fn use_daisy_config() -> Option<DaisyConfig> {
    use_context::<DaisyConfig>()
}

/// Gets the current DaisyConfig from context or uses default
///
/// # Examples
///
/// ```rust
/// use leptos::*;
/// use leptos_daisy::utils::config::use_daisy_config_or_default;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let config = use_daisy_config_or_default();
///     // config is always available
/// }
/// ```
pub fn use_daisy_config_or_default() -> DaisyConfig {
    use_context::<DaisyConfig>().unwrap_or_default()
}

/// Helper function to apply configuration to a class with fallback to default
///
/// This is useful in components where you want to respect the global
/// configuration but provide a sensible default.
///
/// # Examples
///
/// ```rust
/// use leptos::*;
/// use leptos_daisy::utils::config::config_class;
///
/// #[component]
/// fn MyButton() -> impl IntoView {
///     let btn_class = config_class("btn");
///     // btn_class will be "btn" or prefixed version if config exists
/// }
/// ```
pub fn config_class(class: &str) -> String {
    if let Some(config) = use_context::<DaisyConfig>() {
        config.apply_to_class(class)
    } else {
        class.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DaisyConfig::new();
        assert_eq!(config.apply_to_class("btn"), "btn");
        assert_eq!(
            config.apply_to_class("theme-controller"),
            "theme-controller"
        );
    }

    #[test]
    fn test_daisy_prefix() {
        let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
        assert_eq!(config.apply_to_class("btn"), "d-btn");
        assert_eq!(config.apply_to_class("btn-primary"), "d-btn-primary");
    }

    #[test]
    fn test_tailwind_prefix() {
        let config = DaisyConfig::with_prefixes(None, Some("tw:".to_string()));
        assert_eq!(config.apply_to_class("btn"), "tw:btn");
        assert_eq!(config.apply_to_class("text-sm"), "tw:text-sm");
    }

    #[test]
    fn test_both_prefixes() {
        let config = DaisyConfig::with_prefixes(Some("d-".to_string()), Some("tw:".to_string()));
        assert_eq!(config.apply_to_class("btn"), "tw:d-btn");
        assert_eq!(config.apply_to_class("btn-primary"), "tw:d-btn-primary");
    }

    #[test]
    fn test_theme_controller_special_case() {
        let config = DaisyConfig::with_prefixes(Some("d-".to_string()), Some("tw:".to_string()));
        assert_eq!(
            config.apply_to_class("theme-controller"),
            "theme-controller"
        );

        let mut config =
            DaisyConfig::with_prefixes(Some("d-".to_string()), Some("tw:".to_string()));
        config.prefix_theme_controller = true;
        assert_eq!(
            config.apply_to_class("theme-controller"),
            "tw:d-theme-controller"
        );
    }

    #[test]
    fn test_multiple_classes() {
        let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
        assert_eq!(
            config.apply_to_classes(["btn", "btn-primary", "text-sm"]),
            "d-btn d-btn-primary d-text-sm"
        );
    }

    #[test]
    fn test_prefixed_class() {
        let config = DaisyConfig::new();
        assert_eq!(config.prefixed_class("btn"), "btn");

        let config = DaisyConfig::with_prefixes(Some("d-".to_string()), None);
        assert_eq!(config.prefixed_class("btn"), "d-btn");
    }
}
