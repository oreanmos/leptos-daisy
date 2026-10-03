//! The Oikonotes looks: four colour themes (`field`, `field-dark`, `almanac`,
//! `darkroom`) with the Oikonotes colour roles.
//!
//! Values come from the owner-approved Oikonotes design system. Follow the
//! Terminal pattern: inject [`LooksThemeStyles`] once near app root, then set
//! `data-theme` to one of the ids. Pair a theme with the matching aesthetic
//! (`field`, `almanac`, `darkroom`) for fonts, radii and row height.
//!
//! # Role variables
//!
//! daisyUI v5 roles use daisyUI's own names (`--color-base-100`,
//! `--color-primary`, ...). `secondary` is the recessed `base-300` and
//! `neutral` is `base-content`, since the design system defines neither.
//! The extra Oikonotes roles are plain custom properties on the same selector,
//! all in the `--color-*` namespace, plus one shadow:
//!
//! | Variable | Role |
//! |---|---|
//! | `--color-base-content-muted` | labels, metadata, timestamps |
//! | `--color-border`, `--color-border-strong` | hairlines; control edges |
//! | `--color-link` | inline links and quiet actions |
//! | `--color-suggestion`, `--color-suggestion-rule`, `--color-suggestion-content` | AI proposals waiting on the user |
//! | `--color-app-1` ... `--color-app-6` and `--color-app-N-tint` | app palette slots and their washes |
//! | `--color-chart-1` ... `--color-chart-8` | chart series; 7 and 8 are tints of 2 and 4 |
//! | `--shadow-overlay` | the only shadow: menus, popovers, dialogs |
//!
//! Use them in Tailwind 4 with arbitrary values, for example
//! `bg-(--color-app-1-tint)` or `border-(--color-border)`.

use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// `data-theme` id of the Field notebook look (light).
pub const FIELD_THEME_NAME: &str = "field";
/// `data-theme` id of the Field notebook look (dark).
pub const FIELD_DARK_THEME_NAME: &str = "field-dark";
/// `data-theme` id of the Almanac look.
pub const ALMANAC_THEME_NAME: &str = "almanac";
/// `data-theme` id of the Darkroom look.
pub const DARKROOM_THEME_NAME: &str = "darkroom";

/// Every Oikonotes role variable defined by each looks theme, beyond the
/// daisyUI roles. Used by tests and by consumers that derive the same roles
/// for other themes.
pub const OIKONOTES_ROLE_VARS: &[&str] = &[
    "--color-base-content-muted",
    "--color-border",
    "--color-border-strong",
    "--color-link",
    "--color-suggestion",
    "--color-suggestion-rule",
    "--color-suggestion-content",
    "--color-app-1",
    "--color-app-1-tint",
    "--color-app-2",
    "--color-app-2-tint",
    "--color-app-3",
    "--color-app-3-tint",
    "--color-app-4",
    "--color-app-4-tint",
    "--color-app-5",
    "--color-app-5-tint",
    "--color-app-6",
    "--color-app-6-tint",
    "--color-chart-1",
    "--color-chart-2",
    "--color-chart-3",
    "--color-chart-4",
    "--color-chart-5",
    "--color-chart-6",
    "--color-chart-7",
    "--color-chart-8",
    "--shadow-overlay",
];

/// Raw CSS for the `field` theme.
pub const FIELD_THEME_CSS: &str = r#":root:has(input.theme-controller[value="field"]:checked),
[data-theme="field"] {
  color-scheme: light;

  /* daisyUI v5 roles */
  --color-primary: #3e6b4b;
  --color-primary-content: #ffffff;
  --color-secondary: #efebdf;
  --color-secondary-content: #1e221f;
  --color-accent: #b7791f;
  --color-accent-content: #1e221f;
  --color-neutral: #1e221f;
  --color-neutral-content: #fffdf7;
  --color-base-100: #fffdf7;
  --color-base-200: #f6f3ea;
  --color-base-300: #efebdf;
  --color-base-content: #1e221f;
  --color-info: #2c5d8a;
  --color-info-content: #ffffff;
  --color-success: #2f6b45;
  --color-success-content: #ffffff;
  --color-warning: #8a5a0a;
  --color-warning-content: #ffffff;
  --color-error: #a8321e;
  --color-error-content: #ffffff;

  /* daisyUI v5 tokens (radii come from the aesthetic) */
  --border: 1px;
  --depth: 0;
  --noise: 0;

  /* Oikonotes roles */
  --color-base-content-muted: #5a615b;
  --color-border: #e2ddcf;
  --color-border-strong: #d6d0bf;
  --color-link: #2f5a3c;
  --color-suggestion: #f6e7c8;
  --color-suggestion-rule: #b7791f;
  --color-suggestion-content: #6b4a10;
  --color-app-1: #3e6b4b;
  --color-app-1-tint: #e3ecdf;
  --color-app-2: #a8461f;
  --color-app-2-tint: #f5e3d8;
  --color-app-3: #35557a;
  --color-app-3-tint: #dde6f0;
  --color-app-4: #5c6b1f;
  --color-app-4-tint: #e9edd5;
  --color-app-5: #7a3b62;
  --color-app-5-tint: #f0e0ea;
  --color-app-6: #8a6414;
  --color-app-6-tint: #f3e8cc;
  --color-chart-1: #2f5a3c;
  --color-chart-2: #5e9470;
  --color-chart-3: #a6c8ae;
  --color-chart-4: #b7791f;
  --color-chart-5: #e3c27f;
  --color-chart-6: #b9b8ad;
  --color-chart-7: color-mix(in oklch, var(--color-chart-2) 65%, white);
  --color-chart-8: color-mix(in oklch, var(--color-chart-4) 65%, white);
  --shadow-overlay: 0 8px 24px rgba(30, 34, 31, 0.14);
}
"#;

/// Raw CSS for the `field-dark` theme.
pub const FIELD_DARK_THEME_CSS: &str = r#":root:has(input.theme-controller[value="field-dark"]:checked),
[data-theme="field-dark"] {
  color-scheme: dark;

  /* daisyUI v5 roles */
  --color-primary: #8cc49c;
  --color-primary-content: #10231a;
  --color-secondary: #151816;
  --color-secondary-content: #ece8dc;
  --color-accent: #e0a84a;
  --color-accent-content: #151816;
  --color-neutral: #ece8dc;
  --color-neutral-content: #232824;
  --color-base-100: #232824;
  --color-base-200: #1b1f1c;
  --color-base-300: #151816;
  --color-base-content: #ece8dc;
  --color-info: #8ab8e6;
  --color-info-content: #10231a;
  --color-success: #7cc493;
  --color-success-content: #10231a;
  --color-warning: #e7b75c;
  --color-warning-content: #10231a;
  --color-error: #f08a73;
  --color-error-content: #10231a;

  /* daisyUI v5 tokens (radii come from the aesthetic) */
  --border: 1px;
  --depth: 0;
  --noise: 0;

  /* Oikonotes roles */
  --color-base-content-muted: #aeb3a8;
  --color-border: #343b35;
  --color-border-strong: #4a524b;
  --color-link: #9fd1ad;
  --color-suggestion: #3d3220;
  --color-suggestion-rule: #e0a84a;
  --color-suggestion-content: #f1d39a;
  --color-app-1: #8cc49c;
  --color-app-1-tint: #243328;
  --color-app-2: #e59a78;
  --color-app-2-tint: #3a2820;
  --color-app-3: #95b4d9;
  --color-app-3-tint: #222c38;
  --color-app-4: #b9c878;
  --color-app-4-tint: #2c301c;
  --color-app-5: #d69bc0;
  --color-app-5-tint: #36232f;
  --color-app-6: #d9b45e;
  --color-app-6-tint: #352c19;
  --color-chart-1: #9fd1ad;
  --color-chart-2: #6fae84;
  --color-chart-3: #3f7a52;
  --color-chart-4: #e0a84a;
  --color-chart-5: #b98a3a;
  --color-chart-6: #6d736b;
  --color-chart-7: color-mix(in oklch, var(--color-chart-2) 65%, white);
  --color-chart-8: color-mix(in oklch, var(--color-chart-4) 65%, white);
  --shadow-overlay: 0 8px 24px rgba(0, 0, 0, 0.5);
}
"#;

/// Raw CSS for the `almanac` theme.
pub const ALMANAC_THEME_CSS: &str = r#":root:has(input.theme-controller[value="almanac"]:checked),
[data-theme="almanac"] {
  color-scheme: light;

  /* daisyUI v5 roles */
  --color-primary: #121212;
  --color-primary-content: #ffffff;
  --color-secondary: #f3eee6;
  --color-secondary-content: #121212;
  --color-accent: #a8461f;
  --color-accent-content: #ffffff;
  --color-neutral: #121212;
  --color-neutral-content: #ffffff;
  --color-base-100: #ffffff;
  --color-base-200: #fbf8f3;
  --color-base-300: #f3eee6;
  --color-base-content: #121212;
  --color-info: #35557a;
  --color-info-content: #ffffff;
  --color-success: #2f6b45;
  --color-success-content: #ffffff;
  --color-warning: #8a5a0a;
  --color-warning-content: #ffffff;
  --color-error: #a8321e;
  --color-error-content: #ffffff;

  /* daisyUI v5 tokens (radii come from the aesthetic) */
  --border: 1px;
  --depth: 0;
  --noise: 0;

  /* Oikonotes roles */
  --color-base-content-muted: #57534e;
  --color-border: #e7e1d8;
  --color-border-strong: #cfc6bb;
  --color-link: #8f3a18;
  --color-suggestion: #fff6e0;
  --color-suggestion-rule: #c89a2c;
  --color-suggestion-content: #6b4a10;
  --color-app-1: #3e6b4b;
  --color-app-1-tint: #e3ecdf;
  --color-app-2: #a8461f;
  --color-app-2-tint: #f5e3d8;
  --color-app-3: #35557a;
  --color-app-3-tint: #dde6f0;
  --color-app-4: #5c6b1f;
  --color-app-4-tint: #e9edd5;
  --color-app-5: #7a3b62;
  --color-app-5-tint: #f0e0ea;
  --color-app-6: #8a6414;
  --color-app-6-tint: #f3e8cc;
  --color-chart-1: #6e2c12;
  --color-chart-2: #a8461f;
  --color-chart-3: #d27a50;
  --color-chart-4: #e9ad8c;
  --color-chart-5: #f3d2bf;
  --color-chart-6: #cfc6bb;
  --color-chart-7: color-mix(in oklch, var(--color-chart-2) 65%, white);
  --color-chart-8: color-mix(in oklch, var(--color-chart-4) 65%, white);
  --shadow-overlay: 0 10px 30px rgba(18, 18, 18, 0.12);
}
"#;

/// Raw CSS for the `darkroom` theme.
pub const DARKROOM_THEME_CSS: &str = r#":root:has(input.theme-controller[value="darkroom"]:checked),
[data-theme="darkroom"] {
  color-scheme: dark;

  /* daisyUI v5 roles */
  --color-primary: #d9d1c3;
  --color-primary-content: #1c1a16;
  --color-secondary: #100f0d;
  --color-secondary-content: #f2ede4;
  --color-accent: #e0a24a;
  --color-accent-content: #1c1a16;
  --color-neutral: #f2ede4;
  --color-neutral-content: #2a2620;
  --color-base-100: #2a2620;
  --color-base-200: #221f1b;
  --color-base-300: #100f0d;
  --color-base-content: #f2ede4;
  --color-info: #8fb4e0;
  --color-info-content: #1c1a16;
  --color-success: #8fc79a;
  --color-success-content: #1c1a16;
  --color-warning: #f0b35a;
  --color-warning-content: #1c1a16;
  --color-error: #e06c5f;
  --color-error-content: #1c1a16;

  /* daisyUI v5 tokens (radii come from the aesthetic) */
  --border: 1px;
  --depth: 0;
  --noise: 0;

  /* Oikonotes roles */
  --color-base-content-muted: #b8afa2;
  --color-border: #34302a;
  --color-border-strong: #4a453d;
  --color-link: #e6b86e;
  --color-suggestion: rgba(224, 162, 74, 0.16);
  --color-suggestion-rule: #e0a24a;
  --color-suggestion-content: #f0c98a;
  --color-app-1: #9cc4a6;
  --color-app-1-tint: #2a332b;
  --color-app-2: #e59a78;
  --color-app-2-tint: #3a2a22;
  --color-app-3: #9ab5d6;
  --color-app-3-tint: #262d36;
  --color-app-4: #c2c98a;
  --color-app-4-tint: #2f301f;
  --color-app-5: #d7a3c3;
  --color-app-5-tint: #362a31;
  --color-app-6: #e0b866;
  --color-app-6-tint: #37301f;
  --color-chart-1: #f2ede4;
  --color-chart-2: #b8afa2;
  --color-chart-3: #8c8478;
  --color-chart-4: #e0a24a;
  --color-chart-5: #a87a3a;
  --color-chart-6: #5a544b;
  --color-chart-7: color-mix(in oklch, var(--color-chart-2) 65%, white);
  --color-chart-8: color-mix(in oklch, var(--color-chart-4) 65%, white);
  --shadow-overlay: 0 8px 24px rgba(0, 0, 0, 0.6);
}
"#;

/// Injects the four looks themes into the page.
///
/// Place this once at app root, alongside `AestheticStyles`.
#[component]
pub fn LooksThemeStyles(#[prop(attrs)] attrs: Vec<AnyAttribute>) -> impl IntoView {
    let css = [
        FIELD_THEME_CSS,
        FIELD_DARK_THEME_CSS,
        ALMANAC_THEME_CSS,
        DARKROOM_THEME_CSS,
    ]
    .join("\n");
    view! {
        <style id="leptos-daisyui-looks-theme">{css}</style>
    }
    .add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const THEMES: &[(&str, &str)] = &[
        (FIELD_THEME_NAME, FIELD_THEME_CSS),
        (FIELD_DARK_THEME_NAME, FIELD_DARK_THEME_CSS),
        (ALMANAC_THEME_NAME, ALMANAC_THEME_CSS),
        (DARKROOM_THEME_NAME, DARKROOM_THEME_CSS),
    ];

    const DAISY_ROLES: &[&str] = &[
        "--color-primary",
        "--color-primary-content",
        "--color-secondary",
        "--color-secondary-content",
        "--color-accent",
        "--color-accent-content",
        "--color-neutral",
        "--color-neutral-content",
        "--color-base-100",
        "--color-base-200",
        "--color-base-300",
        "--color-base-content",
        "--color-info",
        "--color-success",
        "--color-warning",
        "--color-error",
    ];

    fn defines(css: &str, var: &str) -> bool {
        css.lines()
            .any(|l| l.trim_start().starts_with(&format!("{var}:")))
    }

    #[test]
    fn every_oikonotes_role_is_defined_for_each_theme() {
        for (name, css) in THEMES {
            for var in OIKONOTES_ROLE_VARS {
                assert!(defines(css, var), "{name} is missing {var}");
            }
        }
    }

    #[test]
    fn every_daisy_role_is_defined_for_each_theme() {
        for (name, css) in THEMES {
            for var in DAISY_ROLES {
                assert!(defines(css, var), "{name} is missing {var}");
            }
        }
    }

    #[test]
    fn each_theme_targets_its_own_selector_and_scheme() {
        for (name, css) in THEMES {
            assert!(css.contains(&format!("[data-theme=\"{name}\"]")));
            assert!(css.contains(&format!("input.theme-controller[value=\"{name}\"]:checked")));
        }
        assert!(FIELD_THEME_CSS.contains("color-scheme: light;"));
        assert!(ALMANAC_THEME_CSS.contains("color-scheme: light;"));
        assert!(FIELD_DARK_THEME_CSS.contains("color-scheme: dark;"));
        assert!(DARKROOM_THEME_CSS.contains("color-scheme: dark;"));
    }

    #[test]
    fn chart_tints_derive_from_series_two_and_four() {
        for (name, css) in THEMES {
            assert!(
                css.contains("--color-chart-7: color-mix(in oklch, var(--color-chart-2)"),
                "{name}"
            );
            assert!(
                css.contains("--color-chart-8: color-mix(in oklch, var(--color-chart-4)"),
                "{name}"
            );
        }
    }
}
