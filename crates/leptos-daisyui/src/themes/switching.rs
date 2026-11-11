//! Client-side theme/aesthetic switching utilities.
//!
//! Feature-gated to `csr`/`hydrate` — these functions interact with the DOM
//! and `localStorage`.

use crate::themes::aesthetic::Aesthetic;
use crate::themes::builtin::Theme;

/// Maximum length of a theme/aesthetic id, in bytes.
const MAX_ID_LEN: usize = 64;

/// Validate a theme or aesthetic id: `^[a-z0-9-]{1,64}$`.
///
/// Custom ids (conventionally `user-<note-id>`) follow the same grammar, so
/// they can be used anywhere a built-in id is accepted.
pub fn is_valid_theme_id(id: &str) -> bool {
    (1..=MAX_ID_LEN).contains(&id.len())
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Validate an aesthetic id: a built-in [`Aesthetic`] or a `user-…` custom id.
pub fn is_valid_aesthetic_id(id: &str) -> bool {
    id.parse::<Aesthetic>().is_ok() || (id.starts_with("user-") && is_valid_theme_id(id))
}

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

/// Apply a theme by id — built-in or custom — by setting `data-theme` on
/// `documentElement` and persisting to `localStorage`.
///
/// Unlike [`apply_theme`], the id is not restricted to the [`Theme`] enum, so
/// apps can apply their own themes (conventionally `user-<id>`). The id must
/// pass [`is_valid_theme_id`]; invalid ids are ignored. No-op on server.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn apply_theme_id(id: &str, storage_key: &str) {
    if !is_valid_theme_id(id) {
        return;
    }
    if let Some(el) = leptos::prelude::document().document_element() {
        let _ = el.set_attribute("data-theme", id);
    }
    if let Ok(Some(storage)) = leptos::prelude::window().local_storage() {
        let _ = storage.set_item(storage_key, id);
    }
}

/// Apply a theme id (no-op on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn apply_theme_id(_id: &str, _storage_key: &str) {}

/// Apply an aesthetic preset by setting `data-aesthetic` on `documentElement`
/// and persisting to `localStorage`.
///
/// `storage_key` is the localStorage key (e.g. `"my-app-aesthetic"`).
/// Accepts built-in [`Aesthetic`] ids and `user-…` custom ids (see
/// [`is_valid_aesthetic_id`]); invalid ids are ignored.
///
/// Does NOT change the DaisyUI color theme — call [`apply_theme`] separately.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn apply_aesthetic(aesthetic_id: &str, storage_key: &str) {
    if !is_valid_aesthetic_id(aesthetic_id) {
        return;
    }
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

/// Read a stored theme id from `localStorage` as a raw string.
///
/// Unlike [`read_stored_theme`], this returns custom ids too (`Theme::from_str`
/// rejects them). The stored value is returned only if it passes
/// [`is_valid_theme_id`]. Returns `None` if not found or on server.
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub fn read_stored_theme_id(storage_key: &str) -> Option<String> {
    let storage = leptos::prelude::window().local_storage().ok()??;
    let stored = storage.get_item(storage_key).ok()??;
    is_valid_theme_id(&stored).then_some(stored)
}

/// Read a stored theme id from `localStorage` (always `None` on server).
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
pub fn read_stored_theme_id(_storage_key: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_id_grammar_accepts_builtin_and_user_ids() {
        assert!(is_valid_theme_id("light"));
        assert!(is_valid_theme_id("synthwave-neon"));
        assert!(is_valid_theme_id("user-moss"));
        assert!(is_valid_theme_id("a"));
        assert!(is_valid_theme_id(&"x".repeat(64)));
    }

    #[test]
    fn theme_id_grammar_rejects_invalid_ids() {
        assert!(!is_valid_theme_id(""));
        assert!(!is_valid_theme_id(&"x".repeat(65)));
        assert!(!is_valid_theme_id("User"));
        assert!(!is_valid_theme_id("user_moss"));
        assert!(!is_valid_theme_id("user moss"));
        assert!(!is_valid_theme_id("dracula!"));
        assert!(!is_valid_theme_id("téma"));
    }

    #[test]
    fn aesthetic_id_accepts_builtin_and_user_ids() {
        for a in Aesthetic::all() {
            assert!(is_valid_aesthetic_id(a.as_str()), "{}", a.as_str());
        }
        assert!(is_valid_aesthetic_id("user-moss"));
        assert!(!is_valid_aesthetic_id("moss"));
        assert!(!is_valid_aesthetic_id("user-Moss"));
        assert!(!is_valid_aesthetic_id(""));
    }

    #[test]
    fn custom_theme_ids_are_not_builtin_themes() {
        assert!("user-moss".parse::<Theme>().is_err());
        assert!("light".parse::<Theme>().is_ok());
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn apply_theme_id_sets_data_theme_and_storage() {
        apply_theme_id("user-moss", "leptos-daisyui-test-theme");
        let el = leptos::prelude::document().document_element().unwrap();
        assert_eq!(el.get_attribute("data-theme").as_deref(), Some("user-moss"));
        let storage = leptos::prelude::window().local_storage().unwrap().unwrap();
        assert_eq!(
            storage
                .get_item("leptos-daisyui-test-theme")
                .unwrap()
                .as_deref(),
            Some("user-moss")
        );
        let _ = el.remove_attribute("data-theme");
        let _ = storage.remove_item("leptos-daisyui-test-theme");
    }

    #[wasm_bindgen_test]
    fn apply_theme_id_rejects_invalid_ids() {
        let el = leptos::prelude::document().document_element().unwrap();
        let _ = el.remove_attribute("data-theme");
        let storage = leptos::prelude::window().local_storage().unwrap().unwrap();
        let _ = storage.remove_item("leptos-daisyui-test-theme-2");
        apply_theme_id("User_Invalid!", "leptos-daisyui-test-theme-2");
        assert!(el.get_attribute("data-theme").is_none());
        assert!(
            storage
                .get_item("leptos-daisyui-test-theme-2")
                .unwrap()
                .is_none()
        );
    }

    #[wasm_bindgen_test]
    fn read_stored_theme_id_returns_custom_ids_the_enum_rejects() {
        let storage = leptos::prelude::window().local_storage().unwrap().unwrap();
        storage
            .set_item("leptos-daisyui-test-theme-3", "user-moss")
            .unwrap();
        assert_eq!(
            read_stored_theme_id("leptos-daisyui-test-theme-3").as_deref(),
            Some("user-moss")
        );
        assert!("user-moss".parse::<Theme>().is_err());
        assert_eq!(read_stored_theme("leptos-daisyui-test-theme-3"), None);
        let _ = storage.remove_item("leptos-daisyui-test-theme-3");
    }

    #[wasm_bindgen_test]
    fn apply_aesthetic_accepts_user_ids_and_rejects_invalid() {
        let el = leptos::prelude::document().document_element().unwrap();
        apply_aesthetic("user-moss", "leptos-daisyui-test-aesthetic");
        assert_eq!(
            el.get_attribute("data-aesthetic").as_deref(),
            Some("user-moss")
        );
        apply_aesthetic("not valid!", "leptos-daisyui-test-aesthetic");
        assert_eq!(
            el.get_attribute("data-aesthetic").as_deref(),
            Some("user-moss")
        );
        let _ = el.remove_attribute("data-aesthetic");
        let storage = leptos::prelude::window().local_storage().unwrap().unwrap();
        let _ = storage.remove_item("leptos-daisyui-test-aesthetic");
    }
}
