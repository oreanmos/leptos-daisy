# Code Snippet Rollout Plan

This document tracks the systematic rollout of code snippet updates across all showcase pages.

## Current Status (2024-02-12)

- **Completed Pages:** 5/11 identified pages
- **Remaining Pages:** 6 pages with `include_str!()` calls
- **Build Status:** ✅ All updated pages compile successfully
- **Documentation:** ✅ README.md, TEMPLATE.md, COMPONENT_CHECKLIST.md created

## Completed Pages

### ✅ Button Page (`button_page.rs`)
- **Sections Updated:** 5
  1. Colors (9 color variants)
  2. Styles (8 style variants)
  3. Sizes and shape (5 sizes + square + circle)
  4. State (4 states)
  5. Layout patterns (wide, block, join)
- **Status:** All code snippets use usage examples

### ✅ Alert Page (`alert_page.rs`)
- **Sections Updated:** 7
  1. Variants (4 alert types)
  2. Styles (4 style variants)
  3. Layout Directions (vertical, horizontal)
  4. With Sub-components (icon, title, content, actions)
  5. Dismissible Alert (reactive with Show)
  6. Reactive Alert (dynamic variant switching)
  7. Combination Examples (variant + style + direction)
- **Status:** All code snippets use usage examples including reactive patterns

### ✅ Badge Page (`badge_page.rs`)
- **Sections Updated:** 6
  1. Colors (9 color variants)
  2. Sizes (5 sizes including ExtraLarge - ADDED)
  3. Variants (4 style variants)
  4. Outline (boolean prop)
  5. Empty Badge (indicator dots)
  6. In a button (composition example)
- **Status:** All code snippets use usage examples

### ✅ Card Page (`card_page.rs`)
- **Sections Updated:** 3
  1. Basic Card (default + bordered)
  2. Compact Card
  3. Card with Image (sub-component composition)
- **Status:** All code snippets show sub-component structure

### ✅ Modal Page (`modal_page.rs`)
- **Sections Updated:** 4
  1. Dialog pattern (showModal() interaction)
  2. Forced open/close states
  3. Positions (top, center, bottom)
  4. Action layouts (confirm dialog, info modal)
- **Status:** All code snippets use usage examples

## Remaining Pages

### ⏳ Avatar Page (`avatar_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~5-7 (sizes, shapes, groups, indicators, placeholder)
- **Priority:** High (common component)

### ⏳ Carousel Page (`carousel_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~4-6 (basic, navigation, indicators, responsive)
- **Priority:** Medium (complex interaction)

### ⏳ Collapse Page (`collapse_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~3-5 (basic, icons, nested, accordion)
- **Priority:** Medium

### ⏳ Drawer Page (`drawer_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~4-6 (sides, overlay, content, responsive)
- **Priority:** Medium (layout component)

### ⏳ Dropdown Page (`dropdown_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~5-7 (basic, positions, menu, hover, nested)
- **Priority:** High (common action component)

### ⏳ Tooltip Page (`tooltip_page.rs`)
- **Current:** Uses `include_str!()`
- **Estimated Sections:** ~4-5 (positions, colors, custom content)
- **Priority:** High (common overlay component)

## Recommended Order

Based on component importance and usage frequency:

### Phase 1: High-Priority Components (Next 3)
1. **Dropdown** - Common action pattern
2. **Tooltip** - Common overlay pattern
3. **Avatar** - Common data display

### Phase 2: Medium-Priority Components (Next 3)
4. **Carousel** - Complex but useful
5. **Collapse** - Common accordion pattern
6. **Drawer** - Important layout component

### Phase 3: Systematic Category Rollout
After validating the pattern with 11 pages, systematically update remaining pages by category:

**Data Input Components:**
- Checkbox, File Input, Input, Radio, Range, Rating, Select, Textarea, Toggle

**Navigation Components:**
- Bottom Navigation, Breadcrumbs, Link, Menu, Navbar, Pagination, Steps, Tab

**Layout Components:**
- Artboard, Divider, Footer, Hero, Indicator, Join, Mask, Stack

**Feedback Components:**
- Loading, Progress, Skeleton, Toast

**Data Display Components:**
- Chat, Countdown, Diff, Kbd, List, Stat, Table, Timeline

**Mockup Components:**
- Browser, Code, Phone, Window

## Code Snippet Pattern Reference

### Basic Component
```rust
code=r#"<Component prop=value>"Content"</Component>"#
```

### Multiple Variants
```rust
code=r#"<Component color={Color::Primary}>"Primary"</Component>
<Component color={Color::Secondary}>"Secondary"</Component>
<Component color={Color::Accent}>"Accent"</Component>"#
```

### Sub-Components
```rust
code=r#"<Card>
    <CardBody>
        <CardTitle>"Title"</CardTitle>
        <CardActions>
            <Button>"Action"</Button>
        </CardActions>
    </CardBody>
</Card>"#
```

### Reactive Pattern
```rust
code=r#"let (state, set_state) = signal(initial_value);

view! {
    <button on:click=move |_| set_state.update(|s| *s = new_value)>
        "Change"
    </button>
    <Component state={state}>"Content"</Component>
}"#
```

## Testing Checklist

For each updated page:
- [ ] Replace all `include_str!()` calls with inline snippets
- [ ] Code snippets match preview content
- [ ] Raw strings (`r#"..."#`) used for readability
- [ ] Reactive examples include signal setup
- [ ] Sub-component structure shown fully
- [ ] Run `cargo build -p showcase` - no errors
- [ ] Visit page in browser - loads correctly
- [ ] Click "Code" tab - snippet displays
- [ ] Copy snippet - valid Rust/Leptos syntax
- [ ] All major variants demonstrated

## Build Commands

```bash
# Build showcase
cargo build -p showcase

# Run showcase locally
cd crates/showcase && trunk serve --port 8080

# Format code
cargo fmt

# Check clippy
cargo clippy --workspace -- -D warnings
```

## Progress Tracking

Update COMPONENT_CHECKLIST.md after each page:
```markdown
- [x] ✅ ComponentName - All N sections updated with usage snippets
```

## Notes

- Keep code snippets simple and copy-pasteable
- Include imports in reactive examples (signal setup)
- Show sub-component composition fully
- Use semantic formatting for multi-line code
- Test each page builds before moving to next
