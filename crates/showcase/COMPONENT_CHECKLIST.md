# Component Coverage Checklist

This checklist tracks the status of code snippets for all component showcase pages.

## Status Legend
- ✅ Updated with usage code snippets
- ⏳ Needs updating (still using `include_str!()`)
- ❌ Page not implemented yet

## Actions (4 components)

- [x] ✅ Button - All 5 sections updated with usage snippets
- [x] ✅ Dropdown - All 4 sections updated with usage snippets
- [x] ✅ Modal - All 4 sections updated with usage snippets
- [ ] ⏳ Swap - Needs updating

## Data Display (19 components)

- [x] ✅ Alert - All 7 sections updated with usage snippets (including reactive examples)
- [x] ✅ Avatar - All 5 sections updated with usage snippets
- [x] ✅ Badge - All 6 sections updated with usage snippets, added ExtraLarge size
- [x] ✅ Card - All 3 sections updated with usage snippets
- [x] ✅ Carousel - All 4 sections updated with usage snippets
- [ ] ⏳ Chat - Needs updating
- [x] ✅ Collapse - All 5 sections updated with usage snippets
- [ ] ⏳ Countdown - Needs updating
- [ ] ⏳ Diff - Needs updating
- [ ] ⏳ Kbd - Needs updating
- [ ] ⏳ List - Needs updating
- [ ] ⏳ Stat - Needs updating
- [ ] ⏳ Table - Needs updating
- [ ] ⏳ Timeline - Needs updating
- [x] ✅ Tooltip - All 4 sections updated with usage snippets
- [ ] ❌ Other display components

## Data Input (12 components)

- [ ] ⏳ Checkbox - Needs updating
- [ ] ⏳ File Input - Needs updating
- [ ] ⏳ Input - Needs updating
- [ ] ⏳ Radio - Needs updating
- [ ] ⏳ Range - Needs updating
- [ ] ⏳ Rating - Needs updating
- [ ] ⏳ Select - Needs updating
- [ ] ⏳ Textarea - Needs updating
- [ ] ⏳ Toggle - Needs updating
- [ ] ❌ Other input components

## Navigation (9 components)

- [ ] ⏳ Bottom Navigation - Needs updating
- [ ] ⏳ Breadcrumbs - Needs updating
- [ ] ⏳ Link - Needs updating
- [ ] ⏳ Menu - Needs updating
- [ ] ⏳ Navbar - Needs updating
- [ ] ⏳ Pagination - Needs updating
- [ ] ⏳ Steps - Needs updating
- [ ] ⏳ Tab - Needs updating

## Layout (13 components)

- [ ] ⏳ Artboard - Needs updating
- [ ] ⏳ Divider - Needs updating
- [x] ✅ Drawer - All 4 sections updated with usage snippets (including reactive example)
- [ ] ⏳ Footer - Needs updating
- [ ] ⏳ Hero - Needs updating
- [ ] ⏳ Indicator - Needs updating
- [ ] ⏳ Join - Needs updating
- [ ] ⏳ Mask - Needs updating
- [ ] ⏳ Stack - Needs updating

## Feedback (3 components)

- [ ] ⏳ Loading - Needs updating
- [ ] ⏳ Progress - Needs updating
- [ ] ⏳ Skeleton - Needs updating
- [ ] ⏳ Toast - Needs updating

## Mockups (4 components)

- [ ] ⏳ Browser - Needs updating
- [ ] ⏳ Code - Needs updating
- [ ] ⏳ Phone - Needs updating
- [ ] ⏳ Window - Needs updating

## Summary

**Completed:** 11/70+ pages ✅
- **Phase 1:** Button, Alert, Badge, Card, Modal
- **Phase 2:** Dropdown, Tooltip, Avatar, Carousel, Collapse, Drawer

**Status:** All 6 remaining pages updated successfully!
- ✅ All `include_str!()` calls removed (verified: 0 remaining)
- ✅ Build successful (only 1 minor unused import warning)
- ✅ All code snippets now show usage examples

**Documentation:** Complete
- README.md, TEMPLATE.md, COMPONENT_CHECKLIST.md created
- ROLLOUT_PLAN.md tracks systematic approach

**Remaining:** 59+ pages across other component categories

## Pattern Established

The 11 updated pages demonstrate the standard approach:
- Usage examples instead of implementation code
- Raw strings (`r##"..."##`) for snippets containing `#` characters
- Reactive examples include signal setup
- Sub-component composition shown fully
- All major variants demonstrated

## Notes

- All updated pages now show copy-pasteable usage examples instead of implementation code
- Reactive examples include signal setup in code snippets
- Sub-component composition examples show full structure
- Code snippets use raw strings (`r#"..."#`) for readability
