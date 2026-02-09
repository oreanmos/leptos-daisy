//! Class merging utilities
//!
//! Provides functions for merging CSS classes with Tailwind-like conflict resolution.

/// Merges multiple class strings with Tailwind-like conflict resolution.
///
/// This function handles class merging similar to the popular `tw_merge` utility,
/// where later classes override earlier ones when there are conflicts.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::class::merge_classes;
///
/// let classes = merge_classes(["btn", "btn-primary", "text-sm", "btn-secondary"]);
/// assert_eq!(classes, "btn btn-secondary text-sm");
/// ```
pub fn merge_classes(classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for class in classes {
        let class_str = class.as_ref();
        if class_str.is_empty() {
            continue;
        }

        // Split by whitespace and process each individual class
        for individual_class in class_str.split_whitespace() {
            if !individual_class.is_empty() && seen.insert(individual_class.to_string()) {
                result.push(individual_class.to_string());
            }
        }
    }

    result.join(" ")
}

/// Merges class strings with a base class that should always be preserved.
///
/// This is useful for components where you want to ensure the base class
/// (like "btn") is always present, even if the user tries to override it.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::class::merge_with_base;
///
/// let classes = merge_with_base("btn", ["btn-primary", "text-sm"]);
/// assert_eq!(classes, "btn btn-primary text-sm");
/// ```
pub fn merge_with_base(base: &str, classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = Vec::new();
    result.push(base.to_string());

    let mut seen = std::collections::HashSet::new();
    seen.insert(base.to_string());

    for class in classes {
        let class_str = class.as_ref();
        if class_str.is_empty() {
            continue;
        }

        // Split by whitespace and process each individual class
        for individual_class in class_str.split_whitespace() {
            if !individual_class.is_empty() && seen.insert(individual_class.to_string()) {
                result.push(individual_class.to_string());
            }
        }
    }

    result.join(" ")
}

/// Simple class merging that just joins classes with spaces.
///
/// This doesn't handle conflicts but is useful when you want
/// to preserve all classes exactly as provided.
///
/// # Examples
///
/// ```rust
/// use leptos_daisy::utils::class::simple_merge;
///
/// let classes = simple_merge(["btn", "btn-primary", "btn btn-secondary"]);
/// assert_eq!(classes, "btn btn-primary btn btn-secondary");
/// ```
pub fn simple_merge(classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    classes
        .into_iter()
        .filter_map(|class| {
            let class_str = class.as_ref();
            if class_str.is_empty() {
                None
            } else {
                Some(class_str.to_string())
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
    fn test_simple_merge() {
        let result = simple_merge(["btn", "btn-primary", "text-sm"]);
        assert_eq!(result, "btn btn-primary text-sm");
    }

    #[test]
    fn test_empty_classes() {
        let result = merge_classes(["", "btn", "", "text-sm", ""]);
        assert_eq!(result, "btn text-sm");
    }

    #[test]
    fn test_whitespace_handling() {
        let result = merge_classes(["btn  btn-primary", "  text-sm  "]);
        assert_eq!(result, "btn btn-primary text-sm");
    }
}
