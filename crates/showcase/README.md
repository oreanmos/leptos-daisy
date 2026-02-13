# Leptos DaisyUI Showcase

A live demonstration of all leptos-daisyui components with interactive examples and copy-pasteable code snippets.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) (for building and serving)
- [Node.js](https://nodejs.org/) (for CSS processing with Tailwind)

Install Trunk:
```bash
cargo install trunk
```

## Quick Start

From this directory (`crates/showcase`):

```bash
trunk serve --port 8080 --open
```

This will:
1. Process Tailwind CSS with DaisyUI
2. Build the Leptos application
3. Start a development server at `http://localhost:8080`
4. Open the showcase in your browser
5. Watch for changes and hot-reload

## Development

### Running the Showcase

```bash
# From the showcase directory
cd crates/showcase
trunk serve --port 8080

# Or from the workspace root
trunk serve --config Trunk.toml
```

### Building for Production

```bash
# From the showcase directory
trunk build --release

# Output will be in dist/
```

### CSS Processing

CSS is automatically processed during Trunk builds via a pre_build hook defined in `Trunk.toml`. The hook runs:

```bash
npx @tailwindcss/cli -i crates/showcase/style/main.css -o crates/showcase/style/output.css --minify
```

This processes Tailwind CSS v4 with DaisyUI and generates `style/output.css`.

## Project Structure

```
crates/showcase/
├── src/
│   ├── app.rs              # Main app with router and theme switcher
│   ├── lib.rs              # Library entry point
│   ├── components/         # Showcase-specific components
│   │   ├── component_preview.rs  # Preview/Code tab component
│   │   ├── sidebar_layout.rs     # Layout with navigation
│   │   └── theme_selector.rs     # Theme picker with 35 themes
│   └── pages/              # Component showcase pages (70+)
│       ├── button_page.rs
│       ├── alert_page.rs
│       └── ...
├── style/
│   ├── main.css           # Tailwind/DaisyUI configuration
│   └── output.css         # Generated CSS (not committed)
├── dist/                  # Build output (not committed)
├── index.html            # HTML template
├── Cargo.toml
├── README.md             # This file
└── TEMPLATE.md           # Page template guide
```

## Features

### Theme Switching

The showcase includes a theme selector with all 35 built-in DaisyUI themes:
- Themes are applied to `<html data-theme="...">` attribute
- Current theme persists to localStorage
- Color swatches preview each theme

### Component Pages

Each component has a dedicated showcase page at `/[component-name]` with:
- Interactive examples showing all variants
- Copy-pasteable code snippets
- Live preview of the component
- Code/Preview tabs for easy reference

### Navigation

Components are organized into 7 categories:
- **Actions**: Button, Dropdown, Modal, Swap
- **Data Display**: Alert, Avatar, Badge, Card, etc.
- **Data Input**: Checkbox, Input, Radio, Select, etc.
- **Navigation**: Bottom Navigation, Breadcrumbs, Menu, etc.
- **Layout**: Divider, Drawer, Hero, Stack, etc.
- **Feedback**: Loading, Progress, Skeleton, Toast
- **Mockups**: Browser, Code, Phone, Window

## Adding a New Component Page

See `TEMPLATE.md` for detailed guidance. Quick steps:

1. **Create page file**: `src/pages/my_component_page.rs`
   ```rust
   use leptos::prelude::*;
   use leptos_daisyui::prelude::*;
   use crate::components::ComponentPreview;

   #[component]
   pub fn MyComponentPage() -> impl IntoView {
       view! {
           <div class="space-y-8">
               <ComponentPreview
                   title="Basic Usage"
                   code=r#"<MyComponent>"Example"</MyComponent>"#
               >
                   <MyComponent>"Example"</MyComponent>
               </ComponentPreview>
           </div>
       }
   }
   ```

2. **Register module**: Add to `src/pages/mod.rs`
   ```rust
   pub mod my_component_page;
   ```

3. **Add route**: Update `src/app.rs`
   ```rust
   <Route path="/my-component" view=MyComponentPage/>
   ```

4. **Add navigation**: Update `NAV_SECTIONS` in `src/app.rs`
   ```rust
   NavItem {
       path: "/my-component",
       label: "My Component",
   },
   ```

## Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build
cargo build -p showcase
```

## Troubleshooting

### Port already in use
```bash
trunk serve --port 8081
```

### CSS not updating
The pre_build hook should handle this automatically, but if needed:
```bash
npx @tailwindcss/cli -i style/main.css -o style/output.css --minify
```

### Trunk watch not working
Trunk watches both the showcase and leptos-daisyui crates. Changes to either should trigger a rebuild.

## License

MIT OR Apache-2.0
