//! Theme helpers and ready-to-use custom theme definitions.

pub mod builtin;
pub mod terminal;

pub use builtin::{Theme, UnknownThemeError};
pub use terminal::{
    TERMINAL_THEME_CSS, TERMINAL_THEME_NAME, TerminalThemeShell, TerminalThemeStyles,
};
