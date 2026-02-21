# leptos-daisyui

**leptos-daisyui** is a comprehensive, SSR/CSR/Hydrate/Islands-safe wrapper around the [daisyUI v5](https://daisyui.com/) Tailwind CSS component library for the Leptos web framework. It provides ergonomic Rust struct components for every daisyUI component.

This document serves as an exhaustive guide for LLMs and developers to understand and seamlessly use `leptos-daisyui` in any Leptos project without needing to inspect the source code.

## Quick Start
```rust
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
fn MyApp() -> impl IntoView {
    view! {
        <Button color=Color::Primary>"Click me"</Button>
        <Card bordered=true>
            <CardBody>
                <CardTitle>"Hello"</CardTitle>
                <p>"World"</p>
                <CardActions>
                    <Button color=Color::Primary>"Buy Now"</Button>
                </CardActions>
            </CardBody>
        </Card>
    }
}
```

## Global Concepts

### The Prelude
Always start by importing the prelude to load components, controllers, theme tools, and variant enums:
```rust
use leptos_daisyui::prelude::*;
```

### Prefix Configuration
If your `tailwind.config.js` is set to use a daisyUI prefix (e.g. `daisy-`), you must provide a `DaisyConfig` at the root of your app:
```rust
use leptos_daisyui::prelude::{DaisyConfig, provide_daisy_config};

#[component]
fn App() -> impl IntoView {
    provide_daisy_config(DaisyConfig { 
        daisy_prefix: Some("daisy-".to_string()) 
    });
    
    view! { <Button color=Color::Primary>"Prefixed Button!"</Button> }
}
```

### Common Props
Almost every single component accepts the following common props for customization:
- `class` (`impl Into<String>`): Appends standard Tailwind/custom utility classes to the component.
- `style` (`impl Into<String>`): Pass inline CSS styles as a string.
- `node_ref` (`NodeRef<AnyElement>`): Attach a Leptos `node_ref` for direct DOM access.
- `id` (`impl Into<String>`): Sets the HTML `id` attribute.

### Variant Enums
Control the visual appearance of components through type-safe Enums.

#### `Color` 
Maps to daisyUI semantic colors: `Default`, `Primary`, `Secondary`, `Accent`, `Neutral`, `Info`, `Success`, `Warning`, `Error`.

#### `Size`
Maps to daisyUI sizes: `ExtraSmall`, `Small`, `Medium` (Default), `Large`, `ExtraLarge`.

#### `Variant`
Maps to visual style modifiers: `Solid` (Default), `Outline`, `Ghost`, `Link`, `Soft`, `Dash`.

#### `State`
Maps to interactive states: `Default`, `Active`, `Disabled`, `Loading`, `Focus`.

---

## Theming (`Theme` & Custom Themes)

You can programmatically handle daisyUI themes using the provided `Theme` enum, which matches all 35 built-in themes plus custom themes provided by the crate.

```rust
use leptos_daisyui::prelude::Theme;

let th = Theme::Synthwave;
println!("{}", th.as_str()); // "synthwave"
let is_dark = th.is_dark();  // evaluates to true

// Iterate all themes
for theme in Theme::all() {
    // ...
}
```

### Built-in Custom Themes (e.g. Terminal Theme)
The crate provides a custom `Terminal` theme out-of-the-box. To use it:
```rust
use leptos_daisyui::prelude::{TerminalThemeStyles, TerminalThemeShell};

#[component]
fn App() -> impl IntoView {
    view! {
        // Inject global CSS once
        <TerminalThemeStyles />
        
        // Wrap your app in the terminal shell for font/glow/scanline defaults
        <TerminalThemeShell scanlines=true>
            <Button>"Hack the planet"</Button>
        </TerminalThemeShell>
    }
}
```

---

## Component Reference: Actions

#### `Button`
```rust
<Button color=Color::Primary size=Size::Large variant=Variant::Outline state=State::Active>
    "Submit"
</Button>
```

#### `Dropdown`
```rust
<Dropdown hover=true position=DropdownPosition::Top>
    <DropdownTrigger><Button>"Click Me"</Button></DropdownTrigger>
    <DropdownContent><Menu><MenuItem><a>"Item 1"</a></MenuItem></Menu></DropdownContent>
</Dropdown>
```

#### `Modal`
Use `<Modal>`, `<ModalBox>`, and `<ModalAction>`. For interactive toggling from Rust, use `modal_controller()`.
```rust
<Modal id="my_modal">
    <ModalBox>
        <h3>"Hello!"</h3>
        <ModalAction><form method="dialog"><Button>"Close"</Button></form></ModalAction>
    </ModalBox>
    <ModalBackdrop/>
</Modal>
```

#### `Swap`
```rust
<Swap animation=SwapAnimation::Rotate active=Signal::derive(move || ...)>
    <div slot="on">"ON"</div>
    <div slot="off">"OFF"</div>
</Swap>
```

#### `ThemeController`
```rust
<ThemeController value="dark" type_=ThemeControllerType::Toggle />
```

#### `Fab` (Floating Action Button)
```rust
<Fab position="bottom-right">
    <FabTrigger><Button circle=true><svg>...</svg></Button></FabTrigger>
    <FabAction><Button circle=true>"+"</Button></FabAction>
</Fab>
```

---

## Component Reference: Data Display

#### `Accordion` / `Collapse`
Use `<Collapse>` with `<CollapseTitle>` and `<CollapseContent>`. Place multiple within an `<Accordion>` for exclusive grouping.

#### `Avatar`
Includes `<Avatar>` (props: `size`, `online`, `shape`), `<AvatarImage>`, `<AvatarPlaceholder>`, `<AvatarPlaceholderContent>`, and `<AvatarGroup>` to connect multiple.

#### `Badge`
Props: `color`, `size`, `variant`.

#### `Card`
Includes `<Card>`, `<CardBody>`, `<CardTitle>`, `<CardActions>`, `<CardHeader>`.
Props: `bordered`, `image_full`, `compact`, `variant` (`Side`).

#### `Carousel`
Includes `<Carousel>` and `<CarouselItem>`. Props: `snap`, `vertical`.

#### `Chat` / `ChatBubble`
Includes `<Chat>`, `<ChatBubble>`, `<ChatHeader>`, `<ChatFooter>`, `<ChatImage>`.
Props for `Chat`: `position` (`Start`, `End`).

#### `Stat` / `Stats`
Includes `<Stats>`, `<Stat>`, `<StatTitle>`, `<StatValue>`, `<StatDesc>`, `<StatFigure>`, `<StatActions>`.

#### `Timeline`
Includes `<Timeline>`, `<TimelineItem>`, `<TimelineStart>`, `<TimelineMiddle>`, `<TimelineEnd>`.

#### `Table` (Basic Table Wrapper)
```rust
<Table size=Size::Small zebra=true pinned_rows=true pinned_cols=true>
    <thead><tr><th>"ID"</th></tr></thead>
    <tbody><tr><td>"1"</td></tr></tbody>
</Table>
```

#### `List` (daisyUI v5 List)
Includes `<List>`, `<ListItem>`, `<ListCol>`.

#### `Kbd`
```rust
<Kbd size=KbdSize::Small>"Ctrl"</Kbd>
```

#### `Status`
```rust
<Status color=Color::Success />
```

#### `Tooltip`
```rust
<Tooltip content="Helper text" position=TooltipPosition::Right>
    <Button>"Hover me"</Button>
</Tooltip>
```

---

## Component Reference: Data Input

#### `Input` (Text Input)
Props: `type_`, `placeholder`, `color`, `size`, `bordered`, `ghost`.

#### `Checkbox` / `Radio` / `Toggle`
Standard dual-state inputs accepting `color`, `size`, `checked`.

#### `Range` (Slider)
Props: `color`, `size`, `min`, `max`, `step`, `value`.

#### `Rating`
Includes `<Rating>`, `<RatingItem>`, `<RatingHalf>`, `<RatingMask>`.

#### `Select`
```rust
<Select size=Size::Large bordered=true>
    <SelectOption selected=true>"Pick me"</SelectOption>
</Select>
```

#### `Textarea`
Props: `color`, `size`, `bordered`, `placeholder`.

#### `FileInput`
```rust
<FileInput color=Color::Primary size=Size::Medium variant=FileInputVariant::Ghost />
```

#### `Fieldset` / `Label` / `Validator`
Forms abstractions.
```rust
<Fieldset>
    <Label>"Username"</Label>
    <Validator>
        <Input type_="text" />
        <ValidatorLabel class="validator-hint">"Must be 5 chars"</ValidatorLabel>
    </Validator>
</Fieldset>
```

#### `Filter`
```rust
<Filter>
    <input type="radio" class="btn filter" aria-label="All" />
    <input type="radio" class="btn filter" aria-label="Active" />
</Filter>
```

---

## Component Reference: Layout & Mockups

#### `Artboard`
Props: `size` (`Phone1` to `Phone6`), `horizontal`.

#### `Divider`
Props: `orientation` (`Horizontal`, `Vertical`), `color`.

#### `Drawer`
Wrapper for sidebars. Includes `<Drawer>`, `<DrawerContent>`, `<DrawerSide>`, `<DrawerOverlay>`. Props: `position` (`End`). For logic, use `drawer_controller()`.

#### `Hero`
Includes `<Hero>`, `<HeroOverlay>`, `<HeroContent>`.

#### `Indicator`
Includes `<Indicator>`, `<IndicatorItem>`.

#### `Join`
Includes `<Join>`, `<JoinItem>`. Groups buttons/inputs together contextually.

#### `Stack`
Includes `<Stack>`. Puts elements literally on top of each other.

#### `Mask`
```rust
<Mask shape=MaskShape::Hexagon>
    <img src="..." />
</Mask>
```

#### Mockups
- **`<MockupBrowser>`**: Browser frame wrapper.
- **`<MockupCode>`**: Code block terminal window. `<MockupCodeLine>` for individual lines.
- **`<MockupPhone>`**: Phone frame wrapper.
- **`<MockupWindow>`**: Blank OS window wrapper.

#### Layout Shells
Preconfigured application shells are available in `components::layout` and `components::sidebar_layout`:
- **`<SidebarLayout>`**: Complete responsive admin layout with `<SidebarLayoutNav>`, `<SidebarLayoutMain>`, `<SidebarLayoutSide>`, etc.
- **`<MultiColumnShell>`** / **`<StackedLayout>`**: For simpler application structures like stacked content panels and secondary column pages.
- **`<Container>`** / **`<Grid>`**: Core CSS layout abstractions with max-width and grid support respectively.

---

## Component Reference: Navigation

#### `BottomNavigation`
Includes `<BottomNavigation>`, `<BottomNavigationLabel>`. Props: `size`.

#### `Breadcrumbs`
Wrapper over standard `<ul> <li> <a>` sequences.

#### `Dock`
Includes `<Dock>`, `<DockItem>`, `<DockItemComponent>`. macOS-like toolbar styling.

#### `Menu`
Includes `<Menu>`, `<MenuItem>`, `<MenuTitle>`, `<SubMenu>`.

#### `Navbar`
Includes `<Navbar>`, `<NavbarStart>`, `<NavbarCenter>`, `<NavbarEnd>`.

#### `Pagination`
Semantic wrapper component alias `Pagination` and `PaginationItem`. Usually works alongside `Join`.

#### `Steps`
Includes `<Steps>` and `<Step>`.

#### `Tab` / `Tabs`
Includes `<Tabs>`, `<Tab>`, `<TabContent>`. Props target `TabVariant` (`Lifted`, `Bordered`, `Box`).

#### `Footer`
Includes `<Footer>` and `<FooterTitle>`.
```rust
<Footer class="bg-neutral text-neutral-content rounded p-10">
    <aside><p>"Company Ltd"</p></aside>
    <nav>
        <FooterTitle>"Services"</FooterTitle>
        <Link variant=Variant::Hover>"Design"</Link>
    </nav>
</Footer>
```
#### `Link`
Styled hyperlinks. Props: `color`, `variant` (`Hover`).

---

## Component Reference: Feedback

#### `Alert`
Includes `<Alert>`, `<AlertIcon>`, `<AlertTitle>`, `<AlertContent>`, `<AlertActions>`. 

#### `Loading`
Props: `variant` (`Spinner`, `Dots`, `Ring`, `Ball`, `Bars`, `Infinity`), `size`, `color`.

#### `Progress` / `RadialProgress`
Props: `value`, `max`, `color`.

#### `Skeleton`
Empty placeholder skeleton layout component.

#### `Toast`
Includes `<Toast>`, `<ToastHorizontal>`, `<ToastVertical>`. Can be positioned top/bottom/end/start. For dynamic programmatic toasts, use `toast_controller`.

#### `Diff`
Includes `<Diff>`, `<DiffItem1>`, `<DiffItem2>`, `<DiffResizer>`. Used to show before/after comparison between two items (like images).

---

## Interactive Controllers (Tier B)
For JavaScript-based interactivity that doesn't rely solely on CSS toggles (e.g., dynamically closing a drawer via backend action, showing a programmatic toast from a rust function, or opening a dialog).

You can use standard Leptos signals and bind them to classes/attributes, or use these explicitly provided tools:
- **`modal_controller`**: Provides imperative `.open()`, `.close()`, `.toggle()` for `<Modal id="...">`.
- **`drawer_controller`**: Provides imperative triggers for drawer state.
- **`toast_controller`**: Push and automatically vanish toasts programmatically without keeping them permanently injected in the tree.

*(Note: Tier B features require the `hydrate` or `csr` feature flags in `Cargo.toml` when running in WASM).*

---

## Best Practices for LLM Generation
1. **Always wrap inner text elements securely**: `<Button>"Text"</Button>`.
2. **Use Enums for properties**: Use `color=Color::Primary`, NOT `color="primary"`. Use `size=Size::Large`, NOT `size="lg"`.
3. **Use classes for custom styles**: Use `class="w-full mt-4"` to pass auxiliary Tailwind CSS rules. The internal `merge_classes` utility gracefully deduplicates identical Tailwind utilities with base daisyUI classes.
4. **Always import prelude**: `use leptos_daisyui::prelude::*;` automatically loads components, `Color`, `Size`, `Variant`, `State`, and structural helpers.
5. **Daisy Config**: If using custom generic daisyUI prefixes in tailwind, you must provide the config `DaisyConfig { daisy_prefix: Some("d-".into()) }` at the root of the app using `provide_daisy_config()`.
