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
    pub radius_card: &'static str,
    pub radius_btn: &'static str,
    pub radius_input: &'static str,
    pub shadow_card: &'static str,
    pub shadow_card_hover: &'static str,
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
    radius_card: "0.5rem",
    radius_btn: "0.375rem",
    radius_input: "0.375rem",
    shadow_card: "0 1px 2px rgba(0,0,0,0.05)",
    shadow_card_hover: "0 4px 8px rgba(0,0,0,0.1)",
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
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    shadow_card: "none",
    shadow_card_hover: "none",
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
    radius_card: "0.75rem",
    radius_btn: "0.5rem",
    radius_input: "0.5rem",
    shadow_card: "0 2px 8px rgba(0,0,0,0.06)",
    shadow_card_hover: "0 6px 20px rgba(0,0,0,0.1)",
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
    radius_card: "0.375rem",
    radius_btn: "0.25rem",
    radius_input: "0.25rem",
    shadow_card: "0 1px 3px rgba(0,0,0,0.3)",
    shadow_card_hover: "0 4px 12px rgba(0,0,0,0.4)",
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
    radius_card: "0.25rem",
    radius_btn: "0.25rem",
    radius_input: "0.25rem",
    shadow_card: "0 0 12px rgba(255,0,255,0.15), 0 0 4px rgba(0,255,255,0.1)",
    shadow_card_hover: "0 0 24px rgba(255,0,255,0.25), 0 0 8px rgba(0,255,255,0.2)",
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
    radius_card: "0.125rem",
    radius_btn: "0.125rem",
    radius_input: "0.125rem",
    shadow_card: "2px 2px 0 color-mix(in oklch, var(--color-base-content) 15%, transparent)",
    shadow_card_hover: "3px 3px 0 color-mix(in oklch, var(--color-base-content) 20%, transparent)",
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
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    shadow_card: "4px 4px 0 color-mix(in oklch, var(--color-base-content) 80%, transparent)",
    shadow_card_hover: "6px 6px 0 color-mix(in oklch, var(--color-base-content) 90%, transparent)",
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
    radius_card: "1rem",
    radius_btn: "0.75rem",
    radius_input: "0.75rem",
    shadow_card: "0 4px 16px rgba(0,0,0,0.06)",
    shadow_card_hover: "0 8px 32px rgba(0,0,0,0.1)",
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
    radius_card: "0",
    radius_btn: "0",
    radius_input: "0",
    shadow_card: "none",
    shadow_card_hover: "0 1px 4px rgba(0,0,0,0.08)",
    spacing_page_x: "2rem",
    spacing_page_y: "2.5rem",
    spacing_section: "2rem",
    border_card: "1px solid color-mix(in oklch, var(--color-base-content) 12%, transparent)",
    transition_card: "all 200ms ease",
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
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_every_variant() {
        assert_eq!(Aesthetic::all().len(), 10);
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
}
