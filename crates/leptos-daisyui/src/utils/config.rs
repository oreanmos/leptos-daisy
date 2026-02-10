//! DaisyUI configuration — prefix support and context provider.

use leptos::prelude::*;

/// Global daisyUI configuration for class prefixing.
#[derive(Clone, Debug, Default)]
pub struct DaisyConfig {
    /// DaisyUI class prefix, e.g. `"d-"` makes `btn` → `d-btn`.
    pub daisy_prefix: Option<String>,
}

impl DaisyConfig {
    /// Create a new default (no-prefix) config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply the prefix to a single daisyUI class token.
    pub fn apply(&self, class: &str) -> String {
        match &self.daisy_prefix {
            Some(p) => format!("{p}{class}"),
            None => class.to_string(),
        }
    }
}

/// Provide `DaisyConfig` via Leptos context so components can read it.
pub fn provide_daisy_config(config: DaisyConfig) {
    provide_context(config);
}

/// Read the `DaisyConfig` from context, falling back to defaults.
pub fn use_daisy_config() -> DaisyConfig {
    use_context::<DaisyConfig>().unwrap_or_default()
}
