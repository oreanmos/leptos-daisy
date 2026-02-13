Leptos-DaisyUI Component Library Audit & Fix Plan                                                                            
                                                                                                                              
 Context                                                                                                                      

 The leptos-daisyui library wraps 65+ DaisyUI components for Leptos 0.8. A full audit revealed issues across accessibility,
 API consistency, DaisyUI compliance, and code quality. This plan addresses them in priority order.

 ---
 Phase 1: Quick Wins — Unused Imports & attrs Spread (23 files)

 Remove unused AnyAttribute imports OR add attrs: Vec<AnyAttribute> prop + ..attrs spread to each component for consistency
 (preferred — matches Button pattern).

 Files: countdown.rs, diff.rs, divider.rs, dock.rs, drawer.rs, dropdown.rs, fieldset.rs, file_input.rs, filter.rs, footer.rs,
  link.rs, list.rs, loading.rs, mask.rs, menu.rs, mockup_browser.rs, mockup_code.rs, mockup_window.rs, modal.rs, navbar.rs,
 pagination.rs, progress.rs, radial_progress.rs

 Verification: cargo clippy --workspace -- -D warnings should produce zero warnings.

 ---
 Phase 2: Critical Accessibility Fixes (~19 files)

 2a. Interactive components missing ARIA
 Component: Dropdown
 File: dropdown.rs
 Fix: DropdownTrigger: change <div> → <button>, add aria-expanded. DropdownContent: add role="listbox".
 ────────────────────────────────────────
 Component: Tab
 File: tab.rs
 Fix: Tab: change <a> → <button> (non-navigational tabs).
 ────────────────────────────────────────
 Component: FAB
 File: fab.rs
 Fix: FabTrigger: change <div role="button"> → <button>.
 ────────────────────────────────────────
 Component: Menu
 File: menu.rs
 Fix: Menu: add role="menu". MenuItem: add role="menuitem" to the inner content.
 ────────────────────────────────────────
 Component: Pagination
 File: pagination.rs
 Fix: Wrap in <nav aria-label="Pagination">. PaginationItem: add aria-current="page" when active.
 ────────────────────────────────────────
 Component: Rating
 File: rating.rs
 Fix: Rating container: add role="radiogroup" and aria_label prop.
 ────────────────────────────────────────
 Component: Swap
 File: swap.rs
 Fix: Add aria-label prop on the hidden input. Add aria-checked binding.
 2b. Status/feedback components missing ARIA
 Component: Toast
 File: toast.rs
 Fix: Add role="alert" and aria-live="polite" to container <div>.
 ────────────────────────────────────────
 Component: Loading
 File: loading.rs
 Fix: Add role="status", aria-live="polite", and aria_label prop (default "Loading").
 ────────────────────────────────────────
 Component: Skeleton
 File: skeleton.rs
 Fix: Add aria-busy="true".
 ────────────────────────────────────────
 Component: Countdown
 File: countdown.rs
 Fix: Add aria-live="polite" and role="timer".
 ────────────────────────────────────────
 Component: Steps
 File: steps.rs
 Fix: Step: add optional active: bool prop → sets aria-current="step".
 ────────────────────────────────────────
 Component: Bottom Nav
 File: bottom_navigation.rs
 Fix: Change container to <nav> or add role="navigation" + aria_label prop.
 ────────────────────────────────────────
 Component: Tooltip
 File: tooltip.rs
 Fix: Keep data-tip (DaisyUI needs it). Additionally add aria-describedby pattern with a generated ID linking tooltip text.
 ────────────────────────────────────────
 Component: Chat
 File: chat.rs
 Fix: Add role="log" on the chat wrapper if used as a chat log.
 ────────────────────────────────────────
 Component: Avatar
 File: avatar.rs
 Fix: Add alt prop to AvatarImage. Add aria-label prop option.
 ────────────────────────────────────────
 Component: Badge
 File: badge.rs
 Fix: Add optional aria_label prop.
 ---
 Phase 3: Redundant Props & API Cleanup (3 files)
 Component: Card (card.rs)
 Issue: CardVariant::Bordered AND bordered: bool are redundant. Same for compact, side.
 Fix: Remove the 3 bool props (bordered, compact, side). Keep CardVariant enum. Also support combining variants via multiple
   bool-like props if needed — or document that variant is the preferred API.
 ────────────────────────────────────────
 Component: Badge (badge.rs)
 Issue: outline: bool AND Variant::Outline both add badge-outline.
 Fix: Remove outline bool prop.
 ────────────────────────────────────────
 Component: Hero (hero.rs)
 Issue: Always forces min-h-screen.
 Fix: Make min_height default to None (no min-height class). Add full_screen: bool (default true for backward compat) that
   applies min-h-screen.
 ---
 Phase 4: Standardize Class Building (~10 files, ~30 functions)

 Migrate components still using the inline closure pattern to use class_signal().

 Files needing migration: alert.rs (Alert + 4 sub-components), card.rs (Card + 4 sub-components), badge.rs, tooltip.rs,
 dropdown.rs (DropdownItem).

 Reference pattern (from button.rs):
 let mut mods = Vec::new();
 // ... push modifiers
 let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
 let cls = class_signal("base-class", &refs, class);

 ---
 Phase 5: Enum Consolidation — BREAKING CHANGE (5 files)

 Replace component-specific color/size enums with shared variants where they map 1:1.
 ┌───────────┬───────────────────────────────────────────────────────────────────────┐
 │ Component │                       Custom Enum → Shared Enum                       │
 ├───────────┼───────────────────────────────────────────────────────────────────────┤
 │ Alert     │ AlertVariant → Option<Color> (filter to valid alert colors at render) │
 ├───────────┼───────────────────────────────────────────────────────────────────────┤
 │ Modal     │ ModalColor → Option<Color>                                            │
 ├───────────┼───────────────────────────────────────────────────────────────────────┤
 │ Divider   │ DividerColor → Option<Color>                                          │
 ├───────────┼───────────────────────────────────────────────────────────────────────┤
 │ Chat      │ ChatBubbleColor → Option<Color>                                       │
 ├───────────┼───────────────────────────────────────────────────────────────────────┤
 │ BottomNav │ BottomNavigationSize → Option<Size>                                   │
 └───────────┴───────────────────────────────────────────────────────────────────────┘
 Note: This is a breaking API change. Should be batched as a major/minor version bump. Update mod.rs and prelude.rs exports
 accordingly.

 ---
 Phase 6: Event Handler Props on Form Inputs (9 files)

 Add typed Callback props for common events. These are additive (non-breaking).
 ┌───────────┬────────────────────────────────────────────┐
 │ Component │                Props to Add                │
 ├───────────┼────────────────────────────────────────────┤
 │ Button    │ on_click: Option<Callback<ev::MouseEvent>> │
 ├───────────┼────────────────────────────────────────────┤
 │ Input     │ on_input, on_change, on_focus, on_blur     │
 ├───────────┼────────────────────────────────────────────┤
 │ Textarea  │ on_input, on_change                        │
 ├───────────┼────────────────────────────────────────────┤
 │ Select    │ on_change                                  │
 ├───────────┼────────────────────────────────────────────┤
 │ Checkbox  │ on_change                                  │
 ├───────────┼────────────────────────────────────────────┤
 │ Radio     │ on_change                                  │
 ├───────────┼────────────────────────────────────────────┤
 │ Toggle    │ on_change                                  │
 ├───────────┼────────────────────────────────────────────┤
 │ Range     │ on_input, on_change                        │
 ├───────────┼────────────────────────────────────────────┤
 │ FileInput │ on_change                                  │
 └───────────┴────────────────────────────────────────────┘
 ---
 Phase 7: Missing DaisyUI Modifiers (3 files)
 Component: Skeleton
 Missing: skeleton-wave variant
 Fix: Add wave: bool prop
 ────────────────────────────────────────
 Component: Navbar
 Missing: Color modifiers (navbar-primary etc.)
 Fix: Add color: Option<Color> prop
 ────────────────────────────────────────
 Component: Loading
 Missing: Color uses text-{color} instead of DaisyUI's text-{color} pattern
 Fix: Verify against DaisyUI v5 docs — may already be correct
 ---
 Verification

 After each phase:
 1. cargo build --workspace — no compile errors
 2. cargo clippy --workspace -- -D warnings — no warnings
 3. cargo test --workspace — all tests pass
 4. trunk serve --config Trunk.toml — showcase renders correctly, spot-check modified components
 5. For accessibility fixes: test with browser dev tools accessibility inspector on showcase pages

 ---
 Summary
 ┌───────────────────────────────────┬───────────────┬─────────────┬───────┐
 │               Phase               │     Scope     │  Breaking?  │ Files │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 1. Unused imports / attrs         │ Quick fix     │ No          │ 23    │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 2. Accessibility ARIA             │ Critical      │ No          │ ~19   │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 3. Redundant props                │ Cleanup       │ Yes (minor) │ 3     │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 4. Class building standardization │ Refactor      │ No          │ ~10   │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 5. Enum consolidation             │ API alignment │ Yes (major) │ 5     │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 6. Event handler props            │ Enhancement   │ No          │ 9     │
 ├───────────────────────────────────┼───────────────┼─────────────┼───────┤
 │ 7. Missing DaisyUI modifiers      │ Enhancement   │ No          │ 3     │
 └───────────────────────────────────┴───────────────┴─────────────┴───────┘
╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌