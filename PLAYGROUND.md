# 🎨 Layout Components Playground

This playground allows you to interactively test all the DaisyUI layout components implemented in leptos-daisyui.

## 🚀 Quick Start

### 1. Run the playground

```bash
cd /workspaces/leptos-daisy
cargo leptos watch
```

This will start a development server with hot reloading at `http://localhost:3000`

### 2. Using the Playground

The playground features:

- **Interactive Sidebar Navigation**: Select any layout component from the left sidebar
- **Live Component Testing**: See different variants and configurations of each component
- **Responsive Design**: Test how components behave at different screen sizes
- **All Layout Components**: Container, Divider, Stack, Join, Grid, Flex, Hero, Mockup, and Artboard

## 📁 Files

- `examples/main.rs` - Main entry point for the playground app
- `examples/playground.rs` - Complete playground implementation
- `examples/layout_demo.rs` - Original layout demo (still available)

## 🎯 Features

### Available Components

1. **Container** - Responsive containers with size variants
2. **Divider** - Horizontal/vertical dividers with color options
3. **Stack** - Vertical/horizontal stacking with gap control
4. **Join** - Element joining for buttons and inputs
5. **Grid** - CSS Grid system with columns, rows, and flow control
6. **Flex** - Flexbox with direction, wrap, justify, and align options
7. **Hero** - Hero sections with overlay support
8. **Mockup** - Design mockups (Browser, Phone, Code, Window)
9. **Artboard** - Device frames (Demo, Phone1-Phone6)

### Component Testing

Each component has multiple examples showing:
- Basic usage
- Different variants
- Size and color options
- Layout configurations
- Responsive behavior

## 🔧 Development

### Add New Components

To add new components to the playground:

1. Add the component to the `LayoutComponent` enum in `playground.rs`
2. Create a new playground component function (e.g., `MyComponentPlayground`)
3. Add it to the match statement in the main `LayoutPlayground` component
4. Add a button in the sidebar navigation

### Customize Existing Components

Each playground component is self-contained and can be easily modified:

```rust
#[component]
fn ContainerPlayground() -> impl IntoView {
    view! {
        <div class="space-y-6">
            {/* Your component examples here */}
        </div>
    }
}
```

## 🎨 Styling

The playground uses DaisyUI's theme system. You can customize the appearance by:

- Changing the theme in your Tailwind config
- Adding custom CSS classes
- Using DaisyUI's color variants

## 📱 Responsive Testing

The playground is fully responsive. Test different screen sizes by:

1. Resizing your browser window
2. Using browser developer tools device emulation
3. Testing on actual mobile devices

## 🔗 Integration

The playground integrates with:

- **Leptos reactive system**: Uses signals for component selection
- **DaisyUI components**: All layout components from the library
- **Tailwind CSS**: Utility classes for styling
- **Hot reloading**: Instant updates during development

## 📝 Notes

- The playground is designed for development and testing
- All components are production-ready and can be used in your applications
- The playground demonstrates the full API of each component
- Component props and variants are fully documented in the code

Enjoy exploring the DaisyUI layout components! 🎉