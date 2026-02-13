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

## Release & Updates

This crate is currently private. The recommended way to consume it in other private Leptos projects is a git dependency pinned to a tag or commit.

```toml
[dependencies]
leptos-daisyui = { git = "ssh://git@github.com/<org>/<repo>.git", package = "leptos-daisyui", tag = "v0.1.0" }
```

Release flow:
- Every push to `main` (or `master`) automatically runs `publish-crate.yml`, validates the crate, and produces a packaged `.crate` workflow artifact.
- For a versioned private release asset, bump `crates/leptos-daisyui` version and push a matching tag like `v0.1.1`; the same workflow attaches the `.crate` file to a private GitHub Release.

Update flow for downstream projects:
- Pin to this GitHub repo with `tag` or `rev`, then move that pin when you want to upgrade.
- Then run:

```bash
cargo update -p leptos-daisyui
```

You only need manual `.crate` downloads if you specifically want offline/internal artifact distribution.

## License

Dual-licensed under:
- MIT
- Apache-2.0
