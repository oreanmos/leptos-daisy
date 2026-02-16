//! Terminal-style theme helpers.
//!
//! Use `TerminalThemeStyles` once near app root, then scope sections with
//! `data-theme="terminal"` (or wrap your app with `TerminalThemeShell`).

use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// Theme name used by daisyUI `data-theme`.
pub const TERMINAL_THEME_NAME: &str = "terminal";

/// Raw CSS definition for the built-in terminal theme.
///
/// Includes `:has(input.theme-controller[value="terminal"]:checked)` so it
/// also works with daisyUI's `theme-controller` pattern.
pub const TERMINAL_THEME_CSS: &str = r#":root:has(input.theme-controller[value="terminal"]:checked),
[data-theme="terminal"] {
  color-scheme: dark;

  --color-primary: #16a34a;
  --color-primary-content: #000000;
  --color-secondary: #ea580c;
  --color-secondary-content: #000000;
  --color-accent: #00ffff;
  --color-accent-content: #000000;
  --color-neutral: #1f2937;
  --color-neutral-content: #86efac;
  --color-base-100: #000000;
  --color-base-200: #0f172a;
  --color-base-300: #1e293b;
  --color-base-content: #86efac;
  --color-info: #3b82f6;
  --color-info-content: #000000;
  --color-success: #22c55e;
  --color-success-content: #000000;
  --color-warning: #facc15;
  --color-warning-content: #000000;
  --color-error: #ef4444;
  --color-error-content: #000000;

  /* daisyUI v5 tokens */
  --radius-selector: 0rem;
  --radius-field: 0rem;
  --radius-box: 0rem;
  --border: 1px;
  --depth: 1;
  --noise: 0;

  /* Backwards-compat radius/motion tokens used by older examples */
  --rounded-box: 0rem;
  --rounded-btn: 0rem;
  --rounded-badge: 0rem;
  --tab-radius: 0rem;
  --btn-focus-scale: 1;
  --animation-btn: 0s;
  --animation-input: 0s;

  /* Custom TUI variables */
  --tui-font: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  --tui-text-shadow: 0 0 8px rgba(22, 163, 74, 0.35);
  --tui-scanline-a: rgba(255, 255, 255, 0.035);
  --tui-scanline-b: rgba(0, 0, 0, 0);
  --tui-scanline-opacity: 0.22;
}
"#;

/// Injects the terminal theme CSS into the page.
///
/// Place this once at app root.
#[component]
pub fn TerminalThemeStyles(#[prop(attrs)] attrs: Vec<AnyAttribute>) -> impl IntoView {
    view! {
        <style id="leptos-daisyui-terminal-theme">{TERMINAL_THEME_CSS}</style>
    }
    .add_any_attr(attrs)
}

/// App shell wrapper for the terminal theme.
///
/// Applies:
/// - `data-theme="terminal"`
/// - mono font and text glow
/// - optional scanline overlay
#[component]
pub fn TerminalThemeShell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] scanlines: bool,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let cls = class_signal(
        "min-h-screen bg-base-100 text-base-content [font-family:var(--tui-font)] [text-shadow:var(--tui-text-shadow)]",
        &[],
        class,
    );

    view! {
        <div data-theme=TERMINAL_THEME_NAME class=cls>
            <Show when=move || scanlines>
                <div
                    aria-hidden="true"
                    class="pointer-events-none fixed inset-0 mix-blend-overlay opacity-[var(--tui-scanline-opacity)] bg-[repeating-linear-gradient(to_bottom,var(--tui-scanline-a)_0px,var(--tui-scanline-a)_1px,var(--tui-scanline-b)_2px,var(--tui-scanline-b)_4px)]"
                ></div>
            </Show>

            <div class="relative">{children()}</div>
        </div>
    }
    .add_any_attr(attrs)
}
