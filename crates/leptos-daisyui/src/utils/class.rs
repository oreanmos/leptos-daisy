//! Class merging utilities for combining CSS class strings.

use leptos::prelude::*;
use std::collections::HashSet;

/// Merges multiple class strings, deduplicating individual classes.
pub fn merge_classes(classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for class in classes {
        for token in class.as_ref().split_whitespace() {
            if !token.is_empty() && !seen.contains(token) {
                let s = token.to_string();
                seen.insert(s.clone());
                result.push(s);
            }
        }
    }

    result.join(" ")
}

/// Merges classes with a guaranteed base class that is always first.
pub fn merge_with_base(base: &str, extras: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    // Base always first
    for token in base.split_whitespace() {
        if !seen.contains(token) {
            let s = token.to_string();
            seen.insert(s.clone());
            result.push(s);
        }
    }

    for class in extras {
        for token in class.as_ref().split_whitespace() {
            if !token.is_empty() && !seen.contains(token) {
                let s = token.to_string();
                seen.insert(s.clone());
                result.push(s);
            }
        }
    }

    result.join(" ")
}

/// Builds a class string from a base, optional extra tokens, and an optional user class.
pub fn build_class(base: &str, modifiers: &[&str], user_class: Option<&str>) -> String {
    let mut parts: Vec<&str> = vec![base];
    parts.extend(modifiers.iter().filter(|s| !s.is_empty()));
    if let Some(uc) = user_class {
        parts.push(uc);
    }
    merge_classes(parts)
}

/// Creates a reactive class computation. Pre-computes the static portion
/// (base + modifiers) once, then merges with the user's `class` prop
/// reactively on each access. This ensures dynamic class changes propagate.
pub fn class_signal(
    base: &str,
    modifiers: &[&str],
    user_class: MaybeProp<String>,
) -> impl Fn() -> String + Send + Sync + 'static + use<> {
    let static_cls = build_class(base, modifiers, None);
    move || match user_class.get() {
        Some(uc) if !uc.is_empty() => format!("{static_cls} {uc}"),
        _ => static_cls.clone(),
    }
}

/// Creates a reactive class computation with dynamic modifiers.
pub fn class_signal_dynamic<F>(
    base: &str,
    modifiers: F,
    user_class: MaybeProp<String>,
) -> impl Fn() -> String + Send + Sync + 'static + use<F>
where
    F: Fn() -> Vec<String> + Send + Sync + 'static,
{
    let base = base.to_string();
    move || {
        let mut parts = Vec::new();
        if !base.is_empty() {
            parts.push(base.clone());
        }
        parts.extend(modifiers());
        if let Some(uc) = user_class.get() {
            parts.push(uc);
        }
        merge_classes(parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_classes() {
        let result = merge_classes(["btn", "btn-primary", "text-sm", "btn-primary"]);
        assert_eq!(result, "btn btn-primary text-sm");
    }

    #[test]
    fn test_merge_with_base() {
        let result = merge_with_base("btn", ["btn-primary", "text-sm"]);
        assert_eq!(result, "btn btn-primary text-sm");
    }

    #[test]
    fn test_build_class() {
        let result = build_class("btn", &["btn-primary", "btn-lg"], Some("my-custom"));
        assert_eq!(result, "btn btn-primary btn-lg my-custom");
    }

    #[test]
    fn test_empty_handling() {
        let result = merge_classes(["", "btn", "", "text-sm", ""]);
        assert_eq!(result, "btn text-sm");
    }

    #[test]
    fn test_class_signal_dynamic() {
        use leptos::prelude::*;
        use std::sync::{Arc, Mutex};

        let active = Arc::new(Mutex::new(false));
        let active_clone = active.clone();
        let user_class = MaybeProp::from("custom");

        let cls = class_signal_dynamic(
            "base",
            move || {
                if *active_clone.lock().unwrap() {
                    vec!["active".to_string()]
                } else {
                    vec![]
                }
            },
            user_class,
        );

        assert_eq!(cls(), "base custom");
        *active.lock().unwrap() = true;
        assert_eq!(cls(), "base active custom");
    }
}
