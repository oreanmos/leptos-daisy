# Component Page Template

This document provides a standard template for creating showcase pages for leptos-daisyui components.

## Page Structure

A well-structured component showcase page should follow this order:

1. **Header with description** - Brief explanation of the component
2. **Basic usage** - Simplest example
3. **Colors** - All 9 color variants (if applicable)
4. **Sizes** - All 5 size variants (if applicable)
5. **Variants** - All 6 style variants (if applicable)
6. **States** - All 5 state variants (if applicable)
7. **Component-specific modifiers** - Unique props/features
8. **Sub-components** - Compositional patterns
9. **Interactive/reactive examples** - State-driven behavior
10. **Real-world combinations** - Practical usage patterns

## Standard Template

```rust
use leptos::prelude::*;
use leptos_daisyui::prelude::*;
use crate::components::ComponentPreview;

#[component]
pub fn ComponentNamePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            // 1. Basic Usage
            <ComponentPreview
                title="Basic Usage"
                description="The simplest form of the component."
                code=r#"<ComponentName>"Default"</ComponentName>"#
            >
                <ComponentName>"Default"</ComponentName>
            </ComponentPreview>

            // 2. Colors (if applicable)
            <ComponentPreview
                title="Colors"
                description="Available color variants."
                code=r#"<ComponentName>"Default"</ComponentName>
<ComponentName color={Color::Primary}>"Primary"</ComponentName>
<ComponentName color={Color::Secondary}>"Secondary"</ComponentName>
<ComponentName color={Color::Accent}>"Accent"</ComponentName>
<ComponentName color={Color::Neutral}>"Neutral"</ComponentName>
<ComponentName color={Color::Info}>"Info"</ComponentName>
<ComponentName color={Color::Success}>"Success"</ComponentName>
<ComponentName color={Color::Warning}>"Warning"</ComponentName>
<ComponentName color={Color::Error}>"Error"</ComponentName>"#
            >
                <div class="flex flex-wrap gap-2">
                    <ComponentName>"Default"</ComponentName>
                    <ComponentName color={Color::Primary}>"Primary"</ComponentName>
                    <ComponentName color={Color::Secondary}>"Secondary"</ComponentName>
                    <ComponentName color={Color::Accent}>"Accent"</ComponentName>
                    <ComponentName color={Color::Neutral}>"Neutral"</ComponentName>
                    <ComponentName color={Color::Info}>"Info"</ComponentName>
                    <ComponentName color={Color::Success}>"Success"</ComponentName>
                    <ComponentName color={Color::Warning}>"Warning"</ComponentName>
                    <ComponentName color={Color::Error}>"Error"</ComponentName>
                </div>
            </ComponentPreview>

            // 3. Sizes (if applicable)
            <ComponentPreview
                title="Sizes"
                description="Available size variants."
                code=r#"<ComponentName size={Size::ExtraSmall}>"XS"</ComponentName>
<ComponentName size={Size::Small}>"SM"</ComponentName>
<ComponentName size={Size::Medium}>"MD"</ComponentName>
<ComponentName size={Size::Large}>"LG"</ComponentName>
<ComponentName size={Size::ExtraLarge}>"XL"</ComponentName>"#
            >
                <div class="flex items-end gap-2">
                    <ComponentName size={Size::ExtraSmall}>"XS"</ComponentName>
                    <ComponentName size={Size::Small}>"SM"</ComponentName>
                    <ComponentName size={Size::Medium}>"MD"</ComponentName>
                    <ComponentName size={Size::Large}>"LG"</ComponentName>
                    <ComponentName size={Size::ExtraLarge}>"XL"</ComponentName>
                </div>
            </ComponentPreview>

            // 4. Variants (if applicable)
            <ComponentPreview
                title="Variants"
                description="Style variants for different visual treatments."
                code=r#"<ComponentName>"Solid"</ComponentName>
<ComponentName variant={Variant::Outline}>"Outline"</ComponentName>
<ComponentName variant={Variant::Ghost}>"Ghost"</ComponentName>
<ComponentName variant={Variant::Link}>"Link"</ComponentName>"#
            >
                <div class="flex flex-wrap gap-2">
                    <ComponentName>"Solid"</ComponentName>
                    <ComponentName variant={Variant::Outline}>"Outline"</ComponentName>
                    <ComponentName variant={Variant::Ghost}>"Ghost"</ComponentName>
                    <ComponentName variant={Variant::Link}>"Link"</ComponentName>
                </div>
            </ComponentPreview>

            // 5. States (if applicable)
            <ComponentPreview
                title="States"
                description="Component states."
                code=r#"<ComponentName>"Default"</ComponentName>
<ComponentName state={State::Active}>"Active"</ComponentName>
<ComponentName state={State::Disabled}>"Disabled"</ComponentName>
<ComponentName state={State::Loading}>"Loading"</ComponentName>"#
            >
                <div class="flex flex-wrap gap-2">
                    <ComponentName>"Default"</ComponentName>
                    <ComponentName state={State::Active}>"Active"</ComponentName>
                    <ComponentName state={State::Disabled}>"Disabled"</ComponentName>
                    <ComponentName state={State::Loading}>"Loading"</ComponentName>
                </div>
            </ComponentPreview>

            // 6. Component-specific modifiers
            <ComponentPreview
                title="Special Feature"
                description="Component-specific props or modifiers."
                code=r#"<ComponentName special_prop=true>"Example"</ComponentName>"#
            >
                <ComponentName special_prop=true>"Example"</ComponentName>
            </ComponentPreview>

            // 7. Interactive/reactive example
            <ComponentPreview
                title="Reactive Example"
                description="Component with reactive state."
                code=r#"let (count, set_count) = signal(0);

view! {
    <button on:click=move |_| set_count.update(|n| *n += 1)>
        "Increment"
    </button>
    <ComponentName>
        {move || format!("Count: {}", count.get())}
    </ComponentName>
}"#
            >
                {
                    let (count, set_count) = signal(0);
                    view! {
                        <div class="flex items-center gap-2">
                            <button
                                class="btn btn-sm"
                                on:click=move |_| set_count.update(|n| *n += 1)
                            >
                                "Increment"
                            </button>
                            <ComponentName>
                                {move || format!("Count: {}", count.get())}
                            </ComponentName>
                        </div>
                    }
                }
            </ComponentPreview>

            // 8. Real-world combination
            <ComponentPreview
                title="Complete Example"
                description="A practical real-world usage."
                code=r#"<div class="card bg-base-200 p-4">
    <ComponentName color={Color::Primary} size={Size::Large}>
        "Featured Content"
    </ComponentName>
</div>"#
            >
                <div class="card bg-base-200 p-4">
                    <ComponentName color={Color::Primary} size={Size::Large}>
                        "Featured Content"
                    </ComponentName>
                </div>
            </ComponentPreview>
        </div>
    }
}
```

## Code Snippet Guidelines

### 1. Use Raw Strings

Always use `r#"..."#` for code snippets to avoid escaping:

```rust
code=r#"<Button>"Click me"</Button>"#
```

### 2. Match the Preview

Code snippets should closely match what's shown in the preview:

```rust
<ComponentPreview
    code=r#"<Button color={Color::Primary}>"Primary"</Button>"#
>
    <Button color={Color::Primary}>"Primary"</Button>
</ComponentPreview>
```

### 3. Simplify When Needed

For complex previews with layout divs, you can simplify the code snippet:

```rust
<ComponentPreview
    code=r#"<Button>"One"</Button>
<Button>"Two"</Button>"#
>
    <div class="flex gap-2">  // Layout div not in code
        <Button>"One"</Button>
        <Button>"Two"</Button>
    </div>
</ComponentPreview>
```

### 4. Include Necessary Imports in Context

For reactive examples, include the signal setup:

```rust
code=r#"let (value, set_value) = signal("Hello");

view! {
    <Input value={value} on_input=move |ev| set_value.set(ev)/>
}"#
```

### 5. Show Sub-Component Composition

When components have sub-components, show the full structure:

```rust
code=r#"<Alert variant={AlertVariant::Info}>
    <AlertIcon>"ℹ"</AlertIcon>
    <AlertContent>
        <AlertTitle>"Information"</AlertTitle>
        <AlertDescription>"Details here"</AlertDescription>
    </AlertContent>
</Alert>"#
```

### 6. Use Semantic Formatting

Format multi-line code for readability:

```rust
code=r#"<Card>
    <CardHeader>
        <CardTitle>"Title"</CardTitle>
    </CardHeader>
    <CardBody>
        "Content"
    </CardBody>
    <CardFooter>
        <Button>"Action"</Button>
    </CardFooter>
</Card>"#
```

## Variant Reference

### Colors (9 variants)
```rust
Color::Default      // No color class
Color::Primary      // btn-primary, badge-primary, etc.
Color::Secondary    // btn-secondary, etc.
Color::Accent       // btn-accent, etc.
Color::Neutral      // btn-neutral, etc.
Color::Info         // btn-info, etc.
Color::Success      // btn-success, etc.
Color::Warning      // btn-warning, etc.
Color::Error        // btn-error, etc.
```

### Sizes (5 variants)
```rust
Size::ExtraSmall    // btn-xs, badge-xs, etc.
Size::Small         // btn-sm, etc.
Size::Medium        // btn-md, etc.
Size::Large         // btn-lg, etc.
Size::ExtraLarge    // btn-xl, etc.
```

### Variants (6 variants)
```rust
Variant::Solid      // Default (no class)
Variant::Outline    // btn-outline, badge-outline, etc.
Variant::Ghost      // btn-ghost, etc.
Variant::Link       // btn-link, etc.
Variant::Soft       // Component-specific
Variant::Dash       // Component-specific
```

### States (5 variants)
```rust
State::Default      // No state class
State::Active       // btn-active, etc.
State::Disabled     // btn-disabled, disabled attribute
State::Loading      // loading, btn-loading, etc.
State::Focus        // focus, btn-focus, etc.
```

## Component Categories

### Actions
Components that trigger or control actions:
- Button, Dropdown, Modal, Swap

### Data Display
Components that present information:
- Alert, Avatar, Badge, Card, Carousel, Chat, Collapse, Countdown, Diff, Kbd, Stat, Table, Timeline, etc.

### Data Input
Form components that accept user input:
- Checkbox, File Input, Input, Radio, Range, Rating, Select, Textarea, Toggle

### Navigation
Components for navigating the application:
- Bottom Navigation, Breadcrumbs, Link, Menu, Navbar, Pagination, Steps, Tab

### Layout
Structural components:
- Artboard, Divider, Drawer, Footer, Hero, Indicator, Join, Mask, Stack

### Feedback
Components showing system state:
- Alert, Loading, Progress, Radial Progress, Skeleton, Toast

### Mockups
Visual mockup components:
- Browser, Code, Phone, Window

## Testing Your Page

Before committing, verify:

1. **Build passes**: `cargo build -p showcase`
2. **Page loads**: Visit `/component-name` in browser
3. **Code tab works**: Click "Code" tab to see snippet
4. **Preview matches**: Code snippet reflects what's shown
5. **Copy-pasteable**: Code can be copied and used in another project
6. **All variants shown**: Don't miss any colors, sizes, or states
7. **Responsive**: Test at different screen sizes

## Examples to Reference

Well-structured pages to use as templates:
- `button_page.rs` - Comprehensive with all variants
- `alert_page.rs` - Good reactive examples
- `badge_page.rs` - Clean, simple structure
- `card_page.rs` - Shows sub-component composition
