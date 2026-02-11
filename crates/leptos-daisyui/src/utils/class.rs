//! Class merging utilities for combining CSS class strings.

use leptos::prelude::*;
use std::collections::HashSet;

/// Merges multiple class strings, deduplicating individual classes.
pub fn merge_classes(classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for class in classes {
        for token in class.as_ref().split_whitespace() {
            if !token.is_empty() && seen.insert(token.to_string()) {
                result.push(token.to_string());
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
        if seen.insert(token.to_string()) {
            result.push(token.to_string());
        }
    }

    for class in extras {
        for token in class.as_ref().split_whitespace() {
            if !token.is_empty() && seen.insert(token.to_string()) {
                result.push(token.to_string());
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
}
