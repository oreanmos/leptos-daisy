//! Mockup Code component — daisyUI `mockup-code`.
//!
//! Code editor mockup for displaying code snippets with syntax highlighting appearance.
//! Provides line numbers via data-prefix attribute.

use crate::utils::class::build_class;
use leptos::prelude::*;

/// A line of code for the mockup code component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeLine {
    /// The line content/text
    pub content: String,
    /// Optional prefix/line number (defaults to line number if not specified)
    pub prefix: Option<String>,
    /// Optional color class for the line (e.g., "text-warning", "text-success")
    pub color: Option<String>,
}

impl CodeLine {
    /// Create a new code line with just content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            prefix: None,
            color: None,
        }
    }

    /// Create a new code line with a custom prefix.
    pub fn with_prefix(content: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            prefix: Some(prefix.into()),
            color: None,
        }
    }

    /// Create a new code line with color.
    pub fn with_color(content: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            prefix: None,
            color: Some(color.into()),
        }
    }

    /// Create a new code line with prefix and color.
    pub fn with_prefix_and_color(
        content: impl Into<String>,
        prefix: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            content: content.into(),
            prefix: Some(prefix.into()),
            color: Some(color.into()),
        }
    }
}

/// A daisyUI code mockup component.
///
/// Creates a code editor-style mockup for displaying code snippets.
/// Supports line prefixes (line numbers) and syntax highlighting colors.
///
/// # Props
/// - `lines`: Vector of code lines with optional prefixes and colors
/// - `class`: Additional CSS classes
#[component]
pub fn MockupCode(
    lines: Vec<CodeLine>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mockup-code",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {lines.into_iter().map(|line| {
                let prefix = line.prefix.unwrap_or_default();
                let color_class = line.color.unwrap_or_default();
                view! {
                    <pre data-prefix={prefix} class={color_class}><code>{line.content}</code></pre>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// A daisyUI code mockup component with raw children.
///
/// Use this variant when you want full control over the content.
///
/// # Props
/// - `class`: Additional CSS classes
/// - `children`: Raw children content
#[component]
pub fn MockupCodeRaw(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "mockup-code",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );

    view! {
        <div class={cls}>
            {children()}
        </div>
    }
}
