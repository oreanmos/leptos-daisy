//! Theme helpers and ready-to-use custom theme definitions.

pub mod aesthetic;
pub mod aesthetic_components;
pub mod aesthetic_css;
pub mod builtin;
pub mod switching;
pub mod terminal;

pub use aesthetic::{Aesthetic, AestheticPreset, AestheticTokens, UnknownAestheticError};
pub use aesthetic_components::{AestheticShell, AestheticStyles};
pub use aesthetic_css::AESTHETIC_CSS;
pub use builtin::{Theme, UnknownThemeError};
pub use switching::{
    apply_aesthetic, apply_theme, read_stored_aesthetic, read_stored_theme,
    system_preferred_theme,
};
pub use terminal::{
    TERMINAL_THEME_CSS, TERMINAL_THEME_NAME, TerminalThemeShell, TerminalThemeStyles,
};
