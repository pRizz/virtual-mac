# Project State - App Polish Milestone

## Current Position

**Active Phase:** Phase 4 - TextEdit Polish (In Progress)
**Last Completed:** 04-02-PLAN.md (Document Style & Colors)
**Next Action:** Continue Phase 4 plans or start Phase 5 (Finder)

## Progress

| Phase | Status | Plans | Completed |
|-------|--------|-------|-----------|
| 1 - Calculator | Complete | 2 | 2 |
| 2 - Terminal | Complete | 4 | 4 |
| 3 - Notes | Complete | 4 | 4 |
| 4 - TextEdit | In Progress | 2 | 2 |
| 5 - Finder | Not Started | 0 | - |

**Overall:** 75% complete (12/16 plans across phases)

Progress: [########..] 75%

## Session Continuity

**Last Session:** 2026-01-17
- Completed 04-02-PLAN.md (Document Style & Colors)
- Added document-style wrapper (gray bg, white page with shadow)
- Added text color picker (foreColor execCommand)
- Added highlight color picker (hiliteColor execCommand)
- Added macOS-style cursor and selection styling

**Context for Next Session:**
- TextEdit has document-style appearance
- Color controls functional
- Ready for additional formatting features or next phase
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

## Open Issues

| Issue | Found | Description |
|-------|-------|-------------|
| Window drag bounds | 2026-01-17 | Windows can be dragged above menu bar - top of windows should not go higher than bottom of menu bar |
| Window title centering | 2026-01-17 | Window titles are centered relative to space between traffic lights and right edge, should be centered relative to full window width |
| Clock format wrong | 2026-01-17 | Menu bar clock shows "2 PM:04" instead of "2:04 PM" - fix time format order |
| Clock show seconds | 2026-01-17 | Always show seconds on the digital clock in the menu bar (e.g., "2:04:35 PM") |

## Todos

**Pending:** 0
**Completed:** 0

---

*State updated: 2026-01-17*
*Phase 4 plan 02 completed: 2026-01-17*
