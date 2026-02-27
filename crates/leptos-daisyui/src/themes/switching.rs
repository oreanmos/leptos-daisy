//! Client-side theme/aesthetic switching utilities.
//!
//! Feature-gated to `csr`/`hydrate` — these functions interact with the DOM
//! and `localStorage`.

use crate::themes::builtin::Theme;

/// Apply a DaisyUI theme by setting `data-theme` on `documentElement`
/// and persisting to `localStorage`.
///
/// `storage_key` is the localStorage key (e.g. `"my-app-theme"`).
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn apply_theme(theme: Theme, storage_key: &str) {
    if let Some(el) = leptos::prelude::document().document_element() {
        let _ = el.set_attribute("data-theme", theme.as_str());
    }
    if let Ok(Some(storage)) = leptos::prelude::window().local_storage() {
        let _ = storage.set_item(storage_key, theme.as_str());
    }
}

/// Apply a DaisyUI theme (no-op on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn apply_theme(_theme: Theme, _storage_key: &str) {}

/// Apply an aesthetic preset by setting `data-aesthetic` on `documentElement`
/// and persisting to `localStorage`.
///
/// `storage_key` is the localStorage key (e.g. `"my-app-aesthetic"`).
///
/// Does NOT change the DaisyUI color theme — call [`apply_theme`] separately.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn apply_aesthetic(aesthetic_id: &str, storage_key: &str) {
    if let Some(el) = leptos::prelude::document().document_element() {
        let _ = el.set_attribute("data-aesthetic", aesthetic_id);
    }
    if let Ok(Some(storage)) = leptos::prelude::window().local_storage() {
        let _ = storage.set_item(storage_key, aesthetic_id);
    }
}

/// Apply an aesthetic preset (no-op on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn apply_aesthetic(_aesthetic_id: &str, _storage_key: &str) {}

/// Read a stored DaisyUI theme from `localStorage`.
///
/// Returns `None` if not found or on server.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn read_stored_theme(storage_key: &str) -> Option<Theme> {
    let storage = leptos::prelude::window().local_storage().ok()??;
    let stored = storage.get_item(storage_key).ok()??;
    stored.parse::<Theme>().ok()
}

/// Read a stored DaisyUI theme from `localStorage` (always `None` on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn read_stored_theme(_storage_key: &str) -> Option<Theme> {
    None
}

/// Read a stored aesthetic ID from `localStorage`.
///
/// Returns `None` if not found or on server.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn read_stored_aesthetic(storage_key: &str) -> Option<String> {
    let storage = leptos::prelude::window().local_storage().ok()??;
    storage.get_item(storage_key).ok()?
}

/// Read a stored aesthetic ID from `localStorage` (always `None` on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn read_stored_aesthetic(_storage_key: &str) -> Option<String> {
    None
}

/// Returns the appropriate DaisyUI theme based on system dark/light preference.
///
/// Uses `window.matchMedia("(prefers-color-scheme: dark)")`.
/// Falls back to `dark_theme` if the API is unavailable.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn system_preferred_theme(dark_theme: Theme, light_theme: Theme) -> Theme {
    let dark = web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(true);
    if dark { dark_theme } else { light_theme }
}

/// Returns the dark theme on server (no media query available).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn system_preferred_theme(dark_theme: Theme, _light_theme: Theme) -> Theme {
    dark_theme
}
