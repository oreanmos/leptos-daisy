//! Enum of all built-in daisyUI themes plus crate-provided custom themes.

use core::fmt;
use core::str::FromStr;

/// Every theme available out-of-the-box with daisyUI v5, plus custom themes
/// shipped by this crate.
///
/// Use [`Theme::as_str`] to get the `data-theme` attribute value and
/// [`Theme::all`] to iterate over every variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme {
    // ── built-in (35) ───────────────────────────────────────────────
    Light,
    Dark,
    Cupcake,
    Bumblebee,
    Emerald,
    Corporate,
    Synthwave,
    Retro,
    Cyberpunk,
    Valentine,
    Halloween,
    Garden,
    Forest,
    Aqua,
    Lofi,
    Pastel,
    Fantasy,
    Wireframe,
    Black,
    Luxury,
    Dracula,
    Cmyk,
    Autumn,
    Business,
    Acid,
    Lemonade,
    Night,
    Coffee,
    Winter,
    Dim,
    Nord,
    Sunset,
    Caramellatte,
    Abyss,
    Silk,
    // ── custom (crate-provided) ─────────────────────────────────────
    Terminal,
}

/// All variants in display order (matches daisyUI docs, custom themes last).
const ALL: &[Theme] = &[
    Theme::Light,
    Theme::Dark,
    Theme::Cupcake,
    Theme::Bumblebee,
    Theme::Emerald,
    Theme::Corporate,
    Theme::Synthwave,
    Theme::Retro,
    Theme::Cyberpunk,
    Theme::Valentine,
    Theme::Halloween,
    Theme::Garden,
    Theme::Forest,
    Theme::Aqua,
    Theme::Lofi,
    Theme::Pastel,
    Theme::Fantasy,
    Theme::Wireframe,
    Theme::Black,
    Theme::Luxury,
    Theme::Dracula,
    Theme::Cmyk,
    Theme::Autumn,
    Theme::Business,
    Theme::Acid,
    Theme::Lemonade,
    Theme::Night,
    Theme::Coffee,
    Theme::Winter,
    Theme::Dim,
    Theme::Nord,
    Theme::Sunset,
    Theme::Caramellatte,
    Theme::Abyss,
    Theme::Silk,
    Theme::Terminal,
];

impl Theme {
    /// The `data-theme` attribute value for this theme.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Cupcake => "cupcake",
            Self::Bumblebee => "bumblebee",
            Self::Emerald => "emerald",
            Self::Corporate => "corporate",
            Self::Synthwave => "synthwave",
            Self::Retro => "retro",
            Self::Cyberpunk => "cyberpunk",
            Self::Valentine => "valentine",
            Self::Halloween => "halloween",
            Self::Garden => "garden",
            Self::Forest => "forest",
            Self::Aqua => "aqua",
            Self::Lofi => "lofi",
            Self::Pastel => "pastel",
            Self::Fantasy => "fantasy",
            Self::Wireframe => "wireframe",
            Self::Black => "black",
            Self::Luxury => "luxury",
            Self::Dracula => "dracula",
            Self::Cmyk => "cmyk",
            Self::Autumn => "autumn",
            Self::Business => "business",
            Self::Acid => "acid",
            Self::Lemonade => "lemonade",
            Self::Night => "night",
            Self::Coffee => "coffee",
            Self::Winter => "winter",
            Self::Dim => "dim",
            Self::Nord => "nord",
            Self::Sunset => "sunset",
            Self::Caramellatte => "caramellatte",
            Self::Abyss => "abyss",
            Self::Silk => "silk",
            Self::Terminal => "terminal",
        }
    }

    /// All themes in display order (built-in first, custom last).
    pub fn all() -> &'static [Theme] {
        ALL
    }

    /// Whether this theme uses a dark colour scheme.
    pub fn is_dark(&self) -> bool {
        matches!(
            self,
            Self::Dark
                | Self::Synthwave
                | Self::Halloween
                | Self::Forest
                | Self::Black
                | Self::Luxury
                | Self::Dracula
                | Self::Business
                | Self::Night
                | Self::Coffee
                | Self::Dim
                | Self::Sunset
                | Self::Abyss
                | Self::Terminal
        )
    }

    /// Whether this is a custom theme shipped by this crate (not built into
    /// daisyUI).
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Terminal)
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Theme {
    type Err = UnknownThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ALL.iter()
            .find(|t| t.as_str() == s)
            .copied()
            .ok_or(UnknownThemeError)
    }
}

/// Returned by [`Theme::from_str`] when the string does not match any theme.
#[derive(Debug, Clone)]
pub struct UnknownThemeError;

impl fmt::Display for UnknownThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown daisyUI theme name")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_every_variant() {
        assert_eq!(Theme::all().len(), 36);
    }

    #[test]
    fn round_trip_from_str() {
        for theme in Theme::all() {
            let parsed: Theme = theme.as_str().parse().unwrap();
            assert_eq!(*theme, parsed);
        }
    }

    #[test]
    fn display_matches_as_str() {
        for theme in Theme::all() {
            assert_eq!(theme.to_string(), theme.as_str());
        }
    }

    #[test]
    fn dark_themes_are_dark() {
        assert!(Theme::Dark.is_dark());
        assert!(Theme::Terminal.is_dark());
        assert!(!Theme::Light.is_dark());
        assert!(!Theme::Cupcake.is_dark());
    }

    #[test]
    fn unknown_theme_errors() {
        assert!("nonexistent".parse::<Theme>().is_err());
    }

    #[test]
    fn terminal_is_custom() {
        assert!(Theme::Terminal.is_custom());
        assert!(!Theme::Dark.is_custom());
    }
}
