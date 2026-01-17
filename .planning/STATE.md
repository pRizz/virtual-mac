# Project State - App Polish Milestone

## Current Position

**Active Phase:** Phase 5 - Finder Polish (In Progress)
**Last Completed:** Phase 5, Plan 05 - Context Menu Actions
**Next Action:** Execute Phase 5, Plan 06 (Gallery View)

## Progress

| Phase | Status | Plans | Completed |
|-------|--------|-------|-----------|
| 1 - Calculator | Complete | 2 | 2 |
| 2 - Terminal | Complete | 4 | 4 |
| 3 - Notes | Complete | 4 | 4 |
| 4 - TextEdit | Complete | 3 | 3 |
| 5 - Finder | In Progress | 6 | 5 |

**Overall:** 95% complete (18/19 plans across phases)

Progress: [#########.] 95%

## Session Continuity

**Last Session:** 2026-01-17
- Completed Phase 5 Plan 05 (Context Menu Actions)
- Added context menu trigger to Finder (right-click on items and background)
- Wired New Folder action to VirtualFileSystem
- Wired Rename action with inline input
- Wired Move to Trash action

**Context for Next Session:**
- Finder has Icons, List, Column views and working context menu
- Gallery view still falls back to Icons view (Plan 06)
- Context menu actions: New Folder, Rename, Move to Trash functional
- YOLO mode enabled - auto-approve most decisions

## Key Decisions

| Decision | Date | Rationale |
|----------|------|-----------|
| Near-full replica fidelity | 2026-01-17 | User wants authentic macOS experience, browser limits accepted |
| All 5 apps in v1 | 2026-01-17 | User wants complete app layer polish |
| YOLO workflow mode | 2026-01-17 | User prefers faster iteration with fewer confirmations |
| Inline keyboard handler logic | 2026-01-17 | Simpler than closure cloning, avoids borrowing issues |
| US-style thousands separator | 2026-01-17 | Comma separator matches macOS Calculator US locale |
| Operator color #FF9500 | 2026-01-17 | User-specified exact orange shade for calculator operators |
| Terminal Pro profile colors | 2026-01-17 | Black bg at 85% opacity, white text matches macOS Terminal default |
| Terminal font SF Mono 11px | 2026-01-17 | Matches macOS Terminal Pro profile defaults |
| Root path as home | 2026-01-17 | VirtualFileSystem uses / as root, displayed as ~ in Terminal prompt |
| Clone fs for closures | 2026-01-17 | Rust move semantics require cloning VirtualFileSystem for multiple closures |
| Skip command completion | 2026-01-17 | Tab completes file paths only, not command names (simpler implementation) |
| Notes yellow selection | 2026-01-17 | #ffd52e for note selection matches macOS Notes app |
| Notes blue folder selection | 2026-01-17 | #007aff for folder selection matches macOS standard |
| Soft delete for notes | 2026-01-17 | Notes moved to Recently Deleted rather than immediate removal |
| Folder deletion preserves notes | 2026-01-17 | Notes in deleted folder moved to All Notes (unfiled) |
| execCommand for formatting | 2026-01-17 | Browser-native rich text API, no extra dependencies |
| insertHTML for checklists | 2026-01-17 | Custom checkbox structure with div/input/span |
| Event delegation for checkboxes | 2026-01-17 | Efficient handling of dynamically inserted checkboxes |
| Document max-width 8.5in | 2026-01-17 | Realistic page appearance with 1in padding |
| Selection color #b4d5fe | 2026-01-17 | Matches macOS default selection blue |
| Native color input | 2026-01-17 | Accessibility and browser compatibility over custom picker |
| mousedown for alignment buttons | 2026-01-17 | Prevents contenteditable focus loss when clicking toolbar buttons |
| CSS font-size vs execCommand | 2026-01-17 | execCommand fontSize only takes 1-7, use CSS for actual pixel sizes |
| AppType::Finder variant | 2026-01-17 | Dedicated app type for Finder instead of Generic, follows established pattern |
| js_sys::Date for formatting | 2026-01-17 | Reliable cross-browser date handling without extra dependencies |
| Folders show "--" for size | 2026-01-17 | Matches macOS Finder behavior |
| File kind from extension | 2026-01-17 | Common mappings with fallback to "{EXT} Document" |
| Static class for column items | 2026-01-17 | Computed at render time, avoids reactive overhead for static values |
| CSS ::after for chevrons | 2026-01-17 | Cleaner DOM than inline span element for folder indicators |
| StoredValue for action callback | 2026-01-17 | Leptos pattern to store optional callback for use in closures |
| Effect-based action handling | 2026-01-17 | Pending action signal + Effect cleaner than inline processing |

## Open Issues

| Issue | Found | Description |
|-------|-------|-------------|
| Window drag bounds | 2026-01-17 | Windows can be dragged above menu bar - top of windows should not go higher than bottom of menu bar |
| Window title centering | 2026-01-17 | Window titles are centered relative to space between traffic lights and right edge, should be centered relative to full window width |
| Clock format wrong | 2026-01-17 | Menu bar clock shows "2 PM:04" instead of "2:04 PM" - fix time format order |
| Clock show seconds | 2026-01-17 | Always show seconds on the digital clock in the menu bar (e.g., "2:04:35 PM") |
| Build timestamp prefix | 2026-01-17 | Prefix the built-at timestamp in bottom left corner with "Built at " |

## Todos

**Pending:** 0
**Completed:** 0

---

*State updated: 2026-01-17*
*Phase 5 plan 05 completed: 2026-01-17*
