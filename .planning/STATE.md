# Project State - App Polish Milestone

## Current Position

**Active Phase:** Phase 3 - Notes Creation (Complete)
**Last Completed:** 03-04-PLAN.md (Notes CRUD Operations)
**Next Action:** Start Phase 4 (TextEdit) or Phase 5 (Finder)

## Progress

| Phase | Status | Plans | Completed |
|-------|--------|-------|-----------|
| 1 - Calculator | Complete | 2 | 2 |
| 2 - Terminal | Complete | 4 | 4 |
| 3 - Notes | Complete | 4 | 4 |
| 4 - TextEdit | Not Started | 0 | - |
| 5 - Finder | Not Started | 0 | - |

**Overall:** 62.5% complete (10/16 plans across phases)

Progress: [######....] 62.5%

## Session Continuity

**Last Session:** 2026-01-17
- Completed 03-04-PLAN.md (Notes CRUD Operations)
- Added note create/delete/restore/permanent delete operations
- Added folder create/rename/delete operations
- Added UI with action buttons showing on hover
- Search filtering via visible_notes Memo

**Context for Next Session:**
- Notes app is fully functional with CRUD operations
- State persists to localStorage
- Ready for Phase 4 (TextEdit) or Phase 5 (Finder)
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

## Open Issues

| Issue | Found | Description |
|-------|-------|-------------|
| Window drag bounds | 2026-01-17 | Windows can be dragged above menu bar - top of windows should not go higher than bottom of menu bar |
| Window title centering | 2026-01-17 | Window titles are centered relative to space between traffic lights and right edge, should be centered relative to full window width |

## Todos

**Pending:** 0
**Completed:** 0

---

*State updated: 2026-01-17*
*Phase 3 plan 03 completed: 2026-01-17*
*Phase 3 plan 04 completed: 2026-01-17*
