//! Aesthetic injection components.
//!
//! Use `AestheticStyles` at your app root to inject the CSS token system.
//! Use `AestheticShell` to scope a section to a specific aesthetic.

use crate::themes::aesthetic::Aesthetic;
use crate::themes::aesthetic_css::AESTHETIC_CSS;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// Injects the aesthetic CSS token system into the page.
///
/// Place this once at app root, alongside `TerminalThemeStyles`.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <TerminalThemeStyles />
///     <AestheticStyles />
///     <Router>{ /* ... */ }</Router>
/// }
/// ```
#[component]
pub fn AestheticStyles(#[prop(attrs)] attrs: Vec<AnyAttribute>) -> impl IntoView {
    view! {
        <style id="leptos-daisyui-aesthetic">{AESTHETIC_CSS}</style>
    }
    .add_any_attr(attrs)
}

/// Wraps children with a `data-aesthetic` attribute.
///
/// Use this to scope a section to a specific aesthetic, or wrap your
/// entire app for a global aesthetic.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <AestheticShell aesthetic=Aesthetic::Tui>
///         <p>"This section uses TUI aesthetic"</p>
///     </AestheticShell>
/// }
/// ```
#[component]
pub fn AestheticShell(
    aesthetic: Aesthetic,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();

    view! {
        <div data-aesthetic=aesthetic.as_str() class=cls>
            {children()}
        </div>
    }
    .add_any_attr(attrs)
}
