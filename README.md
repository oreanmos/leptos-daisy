# leptos-daisyui

Type-safe Leptos wrappers for daisyUI components.

This repository contains:
- `crates/leptos-daisyui`: the component library crate
- `crates/showcase`: the interactive showcase/playground app

## Current Status

- Broad daisyUI component coverage is implemented in the library.
- SSR / CSR / Hydrate feature builds are supported.
- The showcase app includes component demos, code snippets, theming pages, and a layout playground.

## Use In Another Private Project

Add this crate from GitHub in your app `Cargo.toml`.

Pin to a tag (recommended):

```toml
[dependencies]
leptos-daisyui = { git = "ssh://git@github.com/<org>/<repo>.git", package = "leptos-daisyui", tag = "v0.1.0", features = ["csr"] }
```

Pin to a commit SHA:

```toml
[dependencies]
leptos-daisyui = { git = "ssh://git@github.com/<org>/<repo>.git", package = "leptos-daisyui", rev = "<commit-sha>", features = ["ssr"] }
```

Track a branch (least reproducible, but useful during active development):

```toml
[dependencies]
leptos-daisyui = { git = "ssh://git@github.com/<org>/<repo>.git", package = "leptos-daisyui", branch = "main", features = ["hydrate"] }
```

HTTPS private repo example (useful when SSH keys are not available):

```toml
[dependencies]
leptos-daisyui = { git = "https://github.com/<org>/<repo>.git", package = "leptos-daisyui", tag = "v0.1.0", features = ["csr"] }
```

For HTTPS auth, configure Git/Cargo credentials on the consumer machine (PAT via credential manager, CI token, or `GITHUB_TOKEN` in CI). Do not hardcode tokens in `Cargo.toml`.

Then fetch/update deps in the consumer project:

```bash
cargo update -p leptos-daisyui
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

Release flow:
- Every push to `main` (or `master`) automatically runs `publish-crate.yml`, validates the crate, and produces a packaged `.crate` workflow artifact.
- For a versioned private release asset, bump `crates/leptos-daisyui` version and push a matching tag like `v0.1.1`; the same workflow attaches the `.crate` file to a private GitHub Release.

Helper script for release prep:

```bash
scripts/release-tag.sh 0.1.1
```

This updates the crate version and prints the exact `git commit`/`tag`/`push` commands to run.

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
