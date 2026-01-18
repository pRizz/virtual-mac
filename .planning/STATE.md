# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-17)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v1.1 System Polish — fix bugs and rough edges

## Current Position

**Milestone:** v1.1 System Polish
**Phase:** 2 of 3 (02-ui-polish-fixes)
**Plan:** 1 of 1 complete
**Status:** Phase complete
**Last activity:** 2026-01-18 — Completed 02-01-PLAN.md

Progress: [######    ] 66% (2 of 3 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | In Progress | - |

## Session Continuity

**Last Session:** 2026-01-18
- Executed 02-01-PLAN.md (UI Polish Fixes)
- Fixed dock icon sizing with emoji normalization (FIX-004)
- Fixed Finder white padding by removing inner border-radius (FIX-005)
- Fixed Calculator clipping by increasing window height to 500px (FIX-006)

**Context for Next Session:**
- Phase 2 complete, ready for Phase 3
- Phase 3: Clock & Display Fixes (format, seconds, build timestamp)
- Remaining: FIX-007, FIX-008, FIX-009

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-18 | MENU_BAR_HEIGHT = 25.0 | Matches CSS --menubar-height variable |
| 2026-01-18 | pointer-events: none on window title | Allows click-through to titlebar for dragging |
| 2026-01-18 | Dock icon font-size 32px | Better fills 48x48 icon space |
| 2026-01-18 | Explicit emoji font-family | Cross-browser consistency for dock icons |
| 2026-01-18 | Calculator window height 500px | Accommodates all buttons without clipping |

## Open Issues

Remaining v1.1 requirements:
- FIX-007: Build timestamp prefix
- FIX-008: Clock format
- FIX-009: Clock seconds

Completed:
- FIX-001: Window drag bounds (01-01)
- FIX-002: Window title centering (01-01)
- FIX-003: Initial Finder AppType (01-01)
- FIX-004: Dock icon sizes (02-01)
- FIX-005: Finder white padding (02-01)
- FIX-006: Calculator clipping (02-01)

## Todos

**Pending:** 0 (in .planning/todos/pending/)

**Completed:** 3
- Fix dock icon sizes to be uniform (FIX-004) - 02-01
- Remove white padding/border in Finder window (FIX-005) - 02-01
- Fix calculator window content clipping (FIX-006) - 02-01

---

*State updated: 2026-01-18*
*Phase 02 Plan 01 completed: 2026-01-18*
