# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-17)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v1.1 System Polish complete

## Current Position

**Milestone:** v1.1 System Polish
**Phase:** 3 of 3 (03-clock-display-fixes)
**Plan:** 1 of 1 complete
**Status:** Milestone complete
**Last activity:** 2026-01-18 — Completed 03-01-PLAN.md

Progress: [##########] 100% (3 of 3 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Complete | 2026-01-18 |

## Session Continuity

**Last Session:** 2026-01-18
- Executed 03-01-PLAN.md (Clock Display Fixes)
- Fixed clock format to show "H:MM:SS AM/PM" (FIX-008)
- Added seconds to clock display (FIX-009)
- Added "Built at" prefix to build timestamp (FIX-007)

**Context for Next Session:**
- v1.1 System Polish milestone complete
- All 9 FIX requirements resolved
- Ready for next milestone or feature work

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-18 | MENU_BAR_HEIGHT = 25.0 | Matches CSS --menubar-height variable |
| 2026-01-18 | pointer-events: none on window title | Allows click-through to titlebar for dragging |
| 2026-01-18 | Dock icon font-size 32px | Better fills 48x48 icon space |
| 2026-01-18 | Explicit emoji font-family | Cross-browser consistency for dock icons |
| 2026-01-18 | Calculator window height 500px | Accommodates all buttons without clipping |
| 2026-01-18 | Clock format: Day H:MM:SS AM/PM | macOS-style time display with seconds |

## Open Issues

All v1.1 requirements completed:
- FIX-001: Window drag bounds (01-01)
- FIX-002: Window title centering (01-01)
- FIX-003: Initial Finder AppType (01-01)
- FIX-004: Dock icon sizes (02-01)
- FIX-005: Finder white padding (02-01)
- FIX-006: Calculator clipping (02-01)
- FIX-007: Build timestamp prefix (03-01)
- FIX-008: Clock format (03-01)
- FIX-009: Clock seconds (03-01)

## Todos

**Pending:** 3 (in .planning/todos/pending/)

**Completed:** 6
- Fix dock icon sizes to be uniform (FIX-004) - 02-01
- Remove white padding/border in Finder window (FIX-005) - 02-01
- Fix calculator window content clipping (FIX-006) - 02-01
- Fix clock format to H:MM:SS AM/PM (FIX-008) - 03-01
- Add seconds to clock display (FIX-009) - 03-01
- Add "Built at" prefix to timestamp (FIX-007) - 03-01

---

*State updated: 2026-01-18*
*Phase 03 Plan 01 completed: 2026-01-18*
*v1.1 System Polish milestone complete: 2026-01-18*
