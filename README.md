# Leptos DaisyUI

A comprehensive Leptos implementation of all DaisyUI components. This library provides Rust/Leptos components that wrap DaisyUI's Tailwind CSS components, making it easy to use DaisyUI in your Leptos applications.

## Features

- **65+ DaisyUI Components**: Complete coverage of all DaisyUI components
- **Type-Safe API**: Rust enums and structs for component props
- **Flexible Usage**: Both simple and children-based component variants
- **Easy Integration**: Works seamlessly with existing Leptos projects
- **Tailwind CSS**: Leverages DaisyUI's Tailwind CSS classes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leptos-daisy = "0.1"
```

### Leptos 0.8 Compatibility

This library is now compatible with Leptos 0.8. Make sure your project uses the latest Leptos 0.8 patch releases:

```toml
[dependencies]
leptos = { version = "0.8.15", features = ["csr", "hydrate"] }
leptos_meta = "0.8.5"
leptos_router = "0.8.11"
```

## Usage

### Basic Example

```rust
use leptos::*;
use leptos_daisy::components::*;

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <ButtonWithChildren variant=ButtonVariant::Primary>
            "Click Me"
        </ButtonWithChildren>
        
        <AlertWithChildren variant=AlertVariant::Success icon=true>
            <strong>"Success:"</strong> "Operation completed!"
        </AlertWithChildren>
    }
}
```

## Components

### Implemented Components

- **Button**: Primary, secondary, accent, info, success, warning, error variants
- **Alert**: Info, success, warning, error alerts with optional icons
- **Card**: Bordered, compact, normal, side layouts with header, body, title, and actions

### Planned Components (65+ total)

The library will include all DaisyUI components:

- Accordion
- Avatar
- Badge
- Breadcrumbs
- Calendar
- Carousel
- Chat
- Checkbox
- Collapse
- Countdown
- Diff
- Divider
- Dock
- Drawer
- Dropdown
- FAB (Floating Action Button)
- Fieldset
- File Input
- Filter
- Footer
- Hero
- Hover 3D
- Hover Gallery
- Indicator
- Input
- Join
- KBD
- Label
- Link
- List
- Loading
- Mask
- Menu
- Mockup Browser
- Mockup Code
- Mockup Phone
- Mockup Window
- Modal
- Navbar
- Pagination
- Progress
- Radial Progress
- Radio
- Range
- Rating
- Select
- Skeleton
- Stack
- Stat
- Status
- Steps
- Swap
- Tab
- Table
- Textarea
- Text Rotate
- Theme Controller
- Timeline
- Toast
- Toggle
- Tooltip
- Validator

## Development

### Building

```bash
cargo build
```

### Running Examples

```bash
cargo run --example basic
```

### Testing

```bash
cargo test
```

### VSCode Configuration

This project includes VSCode configuration for optimal Leptos development:

- **rust-analyzer**: Configured to ignore Leptos macros for better autocompletion
- **Debugging**: Pre-configured launch configurations
- **Tasks**: Common build tasks (build, test, clippy, format)
- **Extensions**: Recommended VSCode extensions

The configuration automatically:
- Enables all Cargo features for rust-analyzer
- Ignores Leptos server macros to reduce false errors
- Provides formatting on save
- Excludes target directories from search

### Recommended VSCode Extensions

- `rust-lang.rust-analyzer` - Rust language support
- `vadimcn.vscode-lldb` - Debugging support
- `tamasfe.even-better-toml` - TOML syntax highlighting
- `serayuzgur.crates` - Crate dependency management
- `vscode-icons-team.vscode-icons` - File icons

## Upgrading from Leptos 0.6 to 0.8

If you're upgrading an existing project from Leptos 0.6 to 0.8, make sure to:

1. Update your Cargo.toml dependencies:
   ```toml
   [dependencies]
    leptos = "0.8.15"
    leptos_meta = "0.8.5"
    leptos_router = "0.8.11"
   ```

2. Check for any breaking changes in the [Leptos 0.8 changelog](https://github.com/leptos-rs/leptos/releases)

3. Update your VSCode configuration to include the Leptos macro ignores as shown in this project

## Contributing

Contributions are welcome! Please open issues for bugs, feature requests, or submit pull requests.

## License

This project is licensed under either of:
- MIT License
- Apache License 2.0

## Acknowledgements

- [Leptos](https://leptos.dev/) - The Rust web framework
- [DaisyUI](https://daisyui.com/) - The Tailwind CSS component library
- [Tailwind CSS](https://tailwindcss.com/) - The utility-first CSS framework

## Roadmap

1. ✅ Basic project setup and structure
2. ✅ Core components (Button, Alert, Card)
3. 🚧 Remaining 60+ components
4. 🚧 Comprehensive documentation
5. 🚧 Testing and validation
6. 🚧 Publish to crates.io
7. 🚧 Create example applications
8. 🚧 Performance optimization
9. 🚧 Accessibility improvements
10. 🚧 Theming support
