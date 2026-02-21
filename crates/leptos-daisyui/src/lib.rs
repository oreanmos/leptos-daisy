//! # leptos-daisyui
//!
//! Leptos component wrappers for daisyUI — the Tailwind CSS component library.
//!
//! This crate provides ergonomic, SSR/CSR/Hydrate/Islands-safe Leptos components
//! for every daisyUI component. Components are pure markup + class composition
//! by default (Tier A), with optional interactive helpers (Tier B) feature-gated
//! behind `hydrate` or `csr`.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_daisyui::prelude::*;
//!
//! #[component]
//! fn MyApp() -> impl IntoView {
//!     view! {
//!         <Button color=Color::Primary>"Click me"</Button>
//!         <Card bordered=true>
//!             <CardBody>
//!                 <CardTitle>"Hello"</CardTitle>
//!                 <p>"World"</p>
//!             </CardBody>
//!         </Card>
//!     }
//! }
//! ```

/// Components module — all daisyUI component wrappers.
pub mod components;

/// Interactive helpers (Tier B) — signal-based controllers.
pub mod interactive;

/// Shared utility functions (class merging, config).
pub mod utils;

/// Shared variant enums (Color, Size, Variant, State).
pub mod variants;

/// Reusable theme helpers and built-in custom themes.
pub mod themes;

/// Prelude — convenient re-exports.
pub mod prelude;
