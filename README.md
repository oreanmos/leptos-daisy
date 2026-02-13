# leptos-daisyui

Type-safe Leptos wrappers for daisyUI components.

This repository contains:
- `crates/leptos-daisyui`: the component library crate
- `crates/showcase`: the interactive showcase/playground app

## Current Status

- Broad daisyUI component coverage is implemented in the library.
- SSR / CSR / Hydrate feature builds are supported.
- The showcase app includes component demos, code snippets, theming pages, and a layout playground.

## Installation

Add the crate to your app:

```toml
[dependencies]
leptos-daisyui = "0.1"
```

Pick the rendering feature(s) you use in your app:

```toml
[dependencies]
leptos-daisyui = { version = "0.1", features = ["csr"] }
# or ["ssr"], ["hydrate"], or a combination depending on your setup
```

## Quick Usage

```rust
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <Button color=Color::Primary size=Size::Medium>
            "Click me"
        </Button>

        <Alert color=Color::Success>
            "Saved successfully"
        </Alert>
    }
}
```

## Run the Showcase

```bash
cd crates/showcase
cargo leptos serve
```

See `crates/showcase/README.md` for showcase-specific details.

## Development Validation

From repository root:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo check -p leptos-daisyui --features ssr
cargo check -p leptos-daisyui --features csr
cargo check -p leptos-daisyui --features hydrate
cargo check -p showcase
cargo test --workspace
```

For showcase production build validation:

```bash
cd crates/showcase
cargo leptos build
```

## License

Dual-licensed under:
- MIT
- Apache-2.0
