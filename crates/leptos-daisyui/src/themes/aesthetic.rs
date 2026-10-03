//! Aesthetic system — Rust types for the 2-layer theme approach.
//!
//! Layer 1: DaisyUI color themes (`data-theme`)
//! Layer 2: Aesthetic presets (`data-aesthetic`) controlling typography, spacing, shadows, radii.

use crate::themes::builtin::Theme;
use core::fmt;
use core::str::FromStr;

/// CSS custom property values for an aesthetic preset.
///
/// These map 1:1 to CSS custom properties defined in [`AESTHETIC_CSS`].
/// Plain data — no signals needed, the CSS does the work.
pub struct AestheticTokens {
    pub font_heading: &'static str,
    pub font_body: &'static str,
    pub font_mono: &'static str,
    /// Face for text the user wrote (note titles and bodies, record names).
    pub font_writing: &'static str,
    /// Face for numbers; applied with tabular figures by `.font-numeric`.
    pub font_numeric: &'static str,
    pub radius_card: &'static str,
    pub radius_btn: &'static str,
    pub radius_input: &'static str,
    /// Radius of checkboxes, toggles and badges (daisyUI `--radius-selector`).
    pub radius_check: &'static str,
    pub shadow_card: &'static str,
    pub shadow_card_hover: &'static str,
    /// Height of dense list and navigation rows.
    pub row_height: &'static str,
    pub spacing_page_x: &'static str,
    pub spacing_page_y: &'static str,
    pub spacing_section: &'static str,
    pub border_card: &'static str,
    pub transition_card: &'static str,
}

/// A complete aesthetic preset combining an identity, DaisyUI theme pairing,
/// and token values.
pub struct AestheticPreset {
    /// Unique ID used in `data-aesthetic` attribute and localStorage.
    pub id: &'static str,
    /// Human-readable display label.
    pub label: &'static str,
    /// One-line description of the aesthetic's feel.
    pub description: &'static str,
    /// DaisyUI theme to pair with this aesthetic, or `None` for system-following.
    pub daisy_theme: Option<Theme>,
    /// DaisyUI theme for dark mode (used when `daisy_theme` is `None`).
    pub dark_theme: Theme,
    /// DaisyUI theme for light mode (used when `daisy_theme` is `None`).
    pub light_theme: Theme,
    /// Token values for this aesthetic.
    pub tokens: AestheticTokens,
}

/// All available aesthetic presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Aesthetic {
    Auto,
    Tui,
    CozyJournal,
    Obsidian,
    CleanMinimal,
    SynthwaveNeon,
    PaperManuscript,
    Brutalist,
    GlassFrost,
    InkCalligraphy,
    Field,
    Almanac,
    Darkroom,
}

const ALL: &[Aesthetic] = &[
    Aesthetic::Auto,
    Aesthetic::Tui,
    Aesthetic::CozyJournal,
    Aesthetic::Obsidian,
    Aesthetic::CleanMinimal,
    Aesthetic::SynthwaveNeon,
    Aesthetic::PaperManuscript,
    Aesthetic::Brutalist,
    Aesthetic::GlassFrost,
    Aesthetic::InkCalligraphy,
    Aesthetic::Field,
    Aesthetic::Almanac,
    Aesthetic::Darkroom,
];

impl Aesthetic {
    /// The `data-aesthetic` attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Tui => "tui",
            Self::CozyJournal => "journal",
            Self::Obsidian => "obsidian",
            Self::CleanMinimal => "minimal",
            Self::SynthwaveNeon => "synthwave-neon",
            Self::PaperManuscript => "paper",
            Self::Brutalist => "brutalist",
            Self::GlassFrost => "glass",
            Self::InkCalligraphy => "ink",
            Self::Field => "field",
            Self::Almanac => "almanac",
            Self::Darkroom => "darkroom",
        }
    }

    /// All aesthetic variants in display order.
    pub fn all() -> &'static [Aesthetic] {
        ALL
    }

    /// Get the full preset definition for this aesthetic.
    pub fn preset(&self) -> &'static AestheticPreset {
        &PRESETS[*self as usize]
    }
}

impl fmt::Display for Aesthetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Aesthetic {
    type Err = UnknownAestheticError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ALL.iter()
            .find(|a| a.as_str() == s)
            .copied()
            .ok_or(UnknownAestheticError)
    }
}

/// Returned by [`Aesthetic::from_str`] when the string does not match any aesthetic.
#[derive(Debug, Clone)]
pub struct UnknownAestheticError;

impl fmt::Display for UnknownAestheticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown aesthetic name")
    }
}

// ── Token constants ───────────────────────────────────────────────

const TOKENS_CLEAN_MINIMAL: AestheticTokens = AestheticTokens {
    font_heading: "ui-sans-serif, system-ui, -apple-system, sans-serif",
    font_body: "ui-sans-serif, system-ui, -apple-system, sans-serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "ui-sans-serif, system-ui, -apple-system, sans-serif",
    font_numeric: "ui-sans-serif, system-ui, -apple-system, sans-serif",
    radius_card: "0.5rem",
    radius_btn: "0.375rem",
    radius_input: "0.375rem",
    radius_check: "0.375rem",
    shadow_card: "0 1px 2px rgba(0,0,0,0.05)",
    shadow_card_hover: "0 4px 8px rgba(0,0,0,0.1)",
    row_height: "2.125rem",
    spacing_page_x: "1.5rem",
    spacing_page_y: "2rem",
    spacing_section: "1.5rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 10%, transparent)",
    transition_card: "all 200ms ease",
};

const TOKENS_TUI: AestheticTokens = AestheticTokens {
    font_heading: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_body: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_numeric: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    radius_check: "0",
    shadow_card: "none",
    shadow_card_hover: "none",
    row_height: "2.125rem",
    spacing_page_x: "1rem",
    spacing_page_y: "1rem",
    spacing_section: "0.75rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 20%, transparent)",
    transition_card: "none",
};

const TOKENS_COZY_JOURNAL: AestheticTokens = AestheticTokens {
    font_heading: "Georgia, 'Times New Roman', serif",
    font_body: "Georgia, 'Times New Roman', serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "Georgia, 'Times New Roman', serif",
    font_numeric: "Georgia, 'Times New Roman', serif",
    radius_card: "0.75rem",
    radius_btn: "0.5rem",
    radius_input: "0.5rem",
    radius_check: "0.5rem",
    shadow_card: "0 2px 8px rgba(0,0,0,0.06)",
    shadow_card_hover: "0 6px 20px rgba(0,0,0,0.1)",
    row_height: "2.125rem",
    spacing_page_x: "2rem",
    spacing_page_y: "2.5rem",
    spacing_section: "1.75rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 8%, transparent)",
    transition_card: "all 300ms ease",
};

const TOKENS_OBSIDIAN: AestheticTokens = AestheticTokens {
    font_heading: "ui-sans-serif, system-ui, sans-serif",
    font_body: "ui-sans-serif, system-ui, sans-serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "ui-sans-serif, system-ui, sans-serif",
    font_numeric: "ui-sans-serif, system-ui, sans-serif",
    radius_card: "0.375rem",
    radius_btn: "0.25rem",
    radius_input: "0.25rem",
    radius_check: "0.25rem",
    shadow_card: "0 1px 3px rgba(0,0,0,0.3)",
    shadow_card_hover: "0 4px 12px rgba(0,0,0,0.4)",
    row_height: "2.125rem",
    spacing_page_x: "1.5rem",
    spacing_page_y: "1.5rem",
    spacing_section: "1rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 15%, transparent)",
    transition_card: "all 150ms ease",
};

const TOKENS_SYNTHWAVE_NEON: AestheticTokens = AestheticTokens {
    font_heading: "'Inter', 'Segoe UI', system-ui, sans-serif",
    font_body: "ui-sans-serif, system-ui, sans-serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "ui-sans-serif, system-ui, sans-serif",
    font_numeric: "ui-sans-serif, system-ui, sans-serif",
    radius_card: "0.25rem",
    radius_btn: "0.25rem",
    radius_input: "0.25rem",
    radius_check: "0.25rem",
    shadow_card: "0 0 12px rgba(255,0,255,0.15), 0 0 4px rgba(0,255,255,0.1)",
    shadow_card_hover: "0 0 24px rgba(255,0,255,0.25), 0 0 8px rgba(0,255,255,0.2)",
    row_height: "2.125rem",
    spacing_page_x: "1.5rem",
    spacing_page_y: "1.5rem",
    spacing_section: "1.25rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 15%, transparent)",
    transition_card: "all 200ms ease",
};

const TOKENS_PAPER_MANUSCRIPT: AestheticTokens = AestheticTokens {
    font_heading: "'Palatino Linotype', Palatino, 'Book Antiqua', serif",
    font_body: "'Palatino Linotype', Palatino, 'Book Antiqua', serif",
    font_mono: "'Courier New', Courier, monospace",
    font_writing: "'Palatino Linotype', Palatino, 'Book Antiqua', serif",
    font_numeric: "'Palatino Linotype', Palatino, 'Book Antiqua', serif",
    radius_card: "0.125rem",
    radius_btn: "0.125rem",
    radius_input: "0.125rem",
    radius_check: "0.125rem",
    shadow_card: "2px 2px 0 color-mix(in oklch, var(--color-base-content) 15%, transparent)",
    shadow_card_hover: "3px 3px 0 color-mix(in oklch, var(--color-base-content) 20%, transparent)",
    row_height: "2.125rem",
    spacing_page_x: "2rem",
    spacing_page_y: "2.5rem",
    spacing_section: "2rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 20%, transparent)",
    transition_card: "all 150ms ease",
};

const TOKENS_BRUTALIST: AestheticTokens = AestheticTokens {
    font_heading: "ui-monospace, 'Courier New', monospace",
    font_body: "ui-monospace, 'Courier New', monospace",
    font_mono: "ui-monospace, 'Courier New', monospace",
    font_writing: "ui-monospace, 'Courier New', monospace",
    font_numeric: "ui-monospace, 'Courier New', monospace",
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    radius_check: "0",
    shadow_card: "4px 4px 0 color-mix(in oklch, var(--color-base-content) 80%, transparent)",
    shadow_card_hover: "6px 6px 0 color-mix(in oklch, var(--color-base-content) 90%, transparent)",
    row_height: "2.125rem",
    spacing_page_x: "1.5rem",
    spacing_page_y: "1.5rem",
    spacing_section: "1.5rem",
    border_card: "2px solid color-mix(in oklch, var(--color-base-content) 80%, transparent)",
    transition_card: "none",
};

const TOKENS_GLASS_FROST: AestheticTokens = AestheticTokens {
    font_heading: "'Inter', ui-sans-serif, system-ui, sans-serif",
    font_body: "'Inter', ui-sans-serif, system-ui, sans-serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "'Inter', ui-sans-serif, system-ui, sans-serif",
    font_numeric: "'Inter', ui-sans-serif, system-ui, sans-serif",
    radius_card: "1rem",
    radius_btn: "0.75rem",
    radius_input: "0.75rem",
    radius_check: "0.75rem",
    shadow_card: "0 4px 16px rgba(0,0,0,0.06)",
    shadow_card_hover: "0 8px 32px rgba(0,0,0,0.1)",
    row_height: "2.125rem",
    spacing_page_x: "2rem",
    spacing_page_y: "2rem",
    spacing_section: "1.75rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 8%, transparent)",
    transition_card: "all 250ms ease",
};

const TOKENS_INK_CALLIGRAPHY: AestheticTokens = AestheticTokens {
    font_heading: "'Didot', 'Bodoni MT', 'Noto Serif Display', serif",
    font_body: "'Garamond', 'Noto Serif', Georgia, serif",
    font_mono: "ui-monospace, 'Fira Code', 'Cascadia Code', monospace",
    font_writing: "'Garamond', 'Noto Serif', Georgia, serif",
    font_numeric: "'Garamond', 'Noto Serif', Georgia, serif",
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    radius_check: "0",
    shadow_card: "none",
    shadow_card_hover: "0 1px 4px rgba(0,0,0,0.08)",
    row_height: "2.125rem",
    spacing_page_x: "2rem",
    spacing_page_y: "2.5rem",
    spacing_section: "2rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 12%, transparent)",
    transition_card: "all 200ms ease",
};

const TOKENS_FIELD: AestheticTokens = AestheticTokens {
    font_heading: "'Atkinson Hyperlegible Next', 'Atkinson Hyperlegible', system-ui, sans-serif",
    font_body: "'Atkinson Hyperlegible Next', 'Atkinson Hyperlegible', system-ui, sans-serif",
    font_mono: "'JetBrains Mono', ui-monospace, monospace",
    font_writing: "'Source Serif 4', Georgia, serif",
    font_numeric: "'Atkinson Hyperlegible Next', 'Atkinson Hyperlegible', system-ui, sans-serif",
    radius_card: "0.75rem",
    radius_btn: "0.5rem",
    radius_input: "0.5rem",
    radius_check: "0.375rem",
    shadow_card: "none",
    shadow_card_hover: "none",
    row_height: "2.125rem",
    spacing_page_x: "2.25rem",
    spacing_page_y: "2rem",
    spacing_section: "1.25rem",
    border_card: "1px solid var(--color-border)",
    transition_card: "none",
};

const TOKENS_ALMANAC: AestheticTokens = AestheticTokens {
    font_heading: "'Fraunces', Georgia, serif",
    font_body: "'Public Sans', system-ui, sans-serif",
    font_mono: "'JetBrains Mono', ui-monospace, monospace",
    font_writing: "'Fraunces', Georgia, serif",
    font_numeric: "'Fraunces', Georgia, serif",
    radius_card: "1.25rem",
    radius_btn: "999px",
    radius_input: "999px",
    radius_check: "0.375rem",
    shadow_card: "none",
    shadow_card_hover: "none",
    row_height: "2.125rem",
    spacing_page_x: "2.25rem",
    spacing_page_y: "2rem",
    spacing_section: "1.25rem",
    border_card: "1px solid var(--color-border)",
    transition_card: "none",
};

const TOKENS_DARKROOM: AestheticTokens = AestheticTokens {
    font_heading: "'IBM Plex Sans', system-ui, sans-serif",
    font_body: "'IBM Plex Sans', system-ui, sans-serif",
    font_mono: "'IBM Plex Mono', ui-monospace, monospace",
    font_writing: "'IBM Plex Sans', system-ui, sans-serif",
    font_numeric: "'IBM Plex Mono', ui-monospace, monospace",
    radius_card: "0.25rem",
    radius_btn: "0.25rem",
    radius_input: "0.25rem",
    radius_check: "0.25rem",
    shadow_card: "none",
    shadow_card_hover: "none",
    row_height: "2rem",
    spacing_page_x: "2.25rem",
    spacing_page_y: "1.5rem",
    spacing_section: "1rem",
    border_card: "1px solid var(--color-border)",
    transition_card: "none",
};

// ── Preset definitions ────────────────────────────────────────────

const PRESETS: &[AestheticPreset] = &[
    AestheticPreset {
        id: "auto",
        label: "Auto (System)",
        description: "Follow your system's light/dark mode preference",
        daisy_theme: None,
        dark_theme: Theme::Business,
        light_theme: Theme::Corporate,
        tokens: TOKENS_CLEAN_MINIMAL,
    },
    AestheticPreset {
        id: "tui",
        label: "TUI",
        description: "Terminal interface — monospace, sharp edges, no shadows",
        daisy_theme: Some(Theme::Terminal),
        dark_theme: Theme::Terminal,
        light_theme: Theme::Terminal,
        tokens: TOKENS_TUI,
    },
    AestheticPreset {
        id: "journal",
        label: "Cozy Journal",
        description: "Warm and inviting — serif fonts, soft shadows, generous spacing",
        daisy_theme: Some(Theme::Autumn),
        dark_theme: Theme::Autumn,
        light_theme: Theme::Autumn,
        tokens: TOKENS_COZY_JOURNAL,
    },
    AestheticPreset {
        id: "obsidian",
        label: "Obsidian",
        description: "Dark and focused — clean lines, subtle depth",
        daisy_theme: Some(Theme::Dark),
        dark_theme: Theme::Dark,
        light_theme: Theme::Dark,
        tokens: TOKENS_OBSIDIAN,
    },
    AestheticPreset {
        id: "minimal",
        label: "Clean Minimal",
        description: "Crisp and utilitarian — system fonts, light borders",
        daisy_theme: Some(Theme::Corporate),
        dark_theme: Theme::Business,
        light_theme: Theme::Corporate,
        tokens: TOKENS_CLEAN_MINIMAL,
    },
    AestheticPreset {
        id: "synthwave-neon",
        label: "Synthwave Neon",
        description: "Futuristic — neon glows, geometric fonts, dark vibes",
        daisy_theme: Some(Theme::Synthwave),
        dark_theme: Theme::Synthwave,
        light_theme: Theme::Synthwave,
        tokens: TOKENS_SYNTHWAVE_NEON,
    },
    AestheticPreset {
        id: "paper",
        label: "Paper Manuscript",
        description: "Analog and vintage — serif fonts, offset shadows, subtle texture",
        daisy_theme: Some(Theme::Retro),
        dark_theme: Theme::Coffee,
        light_theme: Theme::Retro,
        tokens: TOKENS_PAPER_MANUSCRIPT,
    },
    AestheticPreset {
        id: "brutalist",
        label: "Brutalist",
        description: "Raw and stark — monospace, hard shadows, no curves",
        daisy_theme: Some(Theme::Black),
        dark_theme: Theme::Black,
        light_theme: Theme::Wireframe,
        tokens: TOKENS_BRUTALIST,
    },
    AestheticPreset {
        id: "glass",
        label: "Glass Frost",
        description: "Airy and translucent — soft radii, diffused shadows, clean type",
        daisy_theme: Some(Theme::Nord),
        dark_theme: Theme::Nord,
        light_theme: Theme::Nord,
        tokens: TOKENS_GLASS_FROST,
    },
    AestheticPreset {
        id: "ink",
        label: "Ink Calligraphy",
        description: "High-contrast and elegant — decorative serif, minimal shadows",
        daisy_theme: Some(Theme::Luxury),
        dark_theme: Theme::Luxury,
        light_theme: Theme::Fantasy,
        tokens: TOKENS_INK_CALLIGRAPHY,
    },
    AestheticPreset {
        id: "field",
        label: "Field Notebook",
        description: "Warm paper and ink: 8px controls, 12px cards, hairlines, no card shadows",
        daisy_theme: Some(Theme::Field),
        dark_theme: Theme::FieldDark,
        light_theme: Theme::Field,
        tokens: TOKENS_FIELD,
    },
    AestheticPreset {
        id: "almanac",
        label: "Almanac",
        description: "Editorial: Fraunces titles and numerals, pill controls, 20px cards",
        daisy_theme: Some(Theme::Almanac),
        dark_theme: Theme::Almanac,
        light_theme: Theme::Almanac,
        tokens: TOKENS_ALMANAC,
    },
    AestheticPreset {
        id: "darkroom",
        label: "Darkroom",
        description: "Dense and dark: IBM Plex, 4px corners, 32px rows, borders not shadows",
        daisy_theme: Some(Theme::Darkroom),
        dark_theme: Theme::Darkroom,
        light_theme: Theme::Darkroom,
        tokens: TOKENS_DARKROOM,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_every_variant() {
        assert_eq!(Aesthetic::all().len(), 13);
    }

    #[test]
    fn round_trip_from_str() {
        for aesthetic in Aesthetic::all() {
            let parsed: Aesthetic = aesthetic.as_str().parse().unwrap();
            assert_eq!(*aesthetic, parsed);
        }
    }

    #[test]
    fn display_matches_as_str() {
        for aesthetic in Aesthetic::all() {
            assert_eq!(aesthetic.to_string(), aesthetic.as_str());
        }
    }

    #[test]
    fn preset_id_matches_as_str() {
        for aesthetic in Aesthetic::all() {
            assert_eq!(aesthetic.preset().id, aesthetic.as_str());
        }
    }

    #[test]
    fn unknown_aesthetic_errors() {
        assert!("nonexistent".parse::<Aesthetic>().is_err());
    }

    #[test]
    fn auto_has_no_fixed_daisy_theme() {
        assert!(Aesthetic::Auto.preset().daisy_theme.is_none());
    }

    #[test]
    fn non_auto_have_daisy_themes() {
        for aesthetic in &Aesthetic::all()[1..] {
            assert!(
                aesthetic.preset().daisy_theme.is_some(),
                "{} should have a daisy_theme",
                aesthetic
            );
        }
    }

    #[test]
    fn preset_order_is_stable_and_looks_are_appended() {
        let ids: Vec<&str> = Aesthetic::all().iter().map(Aesthetic::as_str).collect();
        assert_eq!(
            ids,
            [
                "auto",
                "tui",
                "journal",
                "obsidian",
                "minimal",
                "synthwave-neon",
                "paper",
                "brutalist",
                "glass",
                "ink",
                "field",
                "almanac",
                "darkroom"
            ]
        );
    }

    #[test]
    fn looks_pair_with_their_themes() {
        let field = Aesthetic::Field.preset();
        assert_eq!(field.light_theme, Theme::Field);
        assert_eq!(field.dark_theme, Theme::FieldDark);
        assert_eq!(
            Aesthetic::Almanac.preset().daisy_theme,
            Some(Theme::Almanac)
        );
        assert_eq!(
            Aesthetic::Darkroom.preset().daisy_theme,
            Some(Theme::Darkroom)
        );
    }

    #[test]
    fn looks_shapes_follow_the_design_system() {
        let f = &Aesthetic::Field.preset().tokens;
        assert_eq!((f.radius_btn, f.radius_card), ("0.5rem", "0.75rem"));
        let a = &Aesthetic::Almanac.preset().tokens;
        assert_eq!((a.radius_btn, a.radius_card), ("999px", "1.25rem"));
        let d = &Aesthetic::Darkroom.preset().tokens;
        assert_eq!(
            (d.radius_btn, d.radius_card, d.row_height),
            ("0.25rem", "0.25rem", "2rem")
        );
        assert_eq!(d.font_numeric, "'IBM Plex Mono', ui-monospace, monospace");
    }

    #[test]
    fn existing_aesthetics_keep_body_font_for_writing_and_numeric() {
        for a in &Aesthetic::all()[..10] {
            let t = &a.preset().tokens;
            assert_eq!(t.font_writing, t.font_body, "{a}");
            assert_eq!(t.font_numeric, t.font_body, "{a}");
            assert_eq!(t.radius_check, t.radius_btn, "{a}");
        }
    }

    #[test]
    fn css_defines_every_token_for_the_looks() {
        for id in ["field", "almanac", "darkroom"] {
            let a: Aesthetic = id.parse().unwrap();
            let t = &a.preset().tokens;
            let css = crate::themes::aesthetic_css::AESTHETIC_CSS;
            let block = css
                .split(&format!("[data-aesthetic=\"{id}\"] {{"))
                .nth(1)
                .unwrap_or_else(|| panic!("no css block for {id}"))
                .split('}')
                .next()
                .unwrap();
            for (var, val) in [
                ("--font-writing", t.font_writing),
                ("--font-numeric", t.font_numeric),
                ("--radius-card", t.radius_card),
                ("--radius-btn", t.radius_btn),
                ("--radius-input", t.radius_input),
                ("--radius-check", t.radius_check),
                ("--row-height", t.row_height),
            ] {
                assert!(block.contains(&format!("{var}: {val};")), "{id} {var}");
            }
        }
    }

    #[test]
    fn every_aesthetic_block_sets_the_new_tokens_explicitly() {
        // Scoped shells resolve `var()` at the root, so each block must carry
        // its own values rather than inherit a computed one.
        let css = crate::themes::aesthetic_css::AESTHETIC_CSS;
        for a in &Aesthetic::all()[1..] {
            let block = css
                .split(&format!("[data-aesthetic=\"{}\"] {{", a.as_str()))
                .nth(1)
                .unwrap()
                .split('}')
                .next()
                .unwrap();
            for var in ["--font-writing:", "--font-numeric:", "--radius-check:"] {
                assert!(block.contains(var), "{a} lacks {var}");
            }
        }
    }
}
