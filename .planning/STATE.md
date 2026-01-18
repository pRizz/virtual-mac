# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-18)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** Ready for next milestone

## Current Position

**Milestone:** v1.1 System Polish
**Status:** Shipped
**Shipped:** 2026-01-18

Progress: [##########] 100% (3 of 3 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Shipped | 2026-01-18 |

## Session Continuity

**Last Session:** 2026-01-18
- Completed v1.1 System Polish milestone
- All 9 FIX requirements resolved
- UAT passed (9/9 tests)
- Milestone audit passed (9/9 requirements, 3/3 phases)

**Context for Next Session:**
- v1.1 shipped
- Ready for new milestone planning

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-18 | MENU_BAR_HEIGHT = 25.0 | Matches CSS --menubar-height variable |
| 2026-01-18 | pointer-events: none on window title | Allows click-through to titlebar for dragging |
| 2026-01-18 | Dock icon font-size 32px | Better fills 48x48 icon space |
| 2026-01-18 | Explicit emoji font-family | Cross-browser consistency for dock icons |
| 2026-01-18 | Calculator window 280x540 | Accommodates all buttons without clipping |
| 2026-01-18 | Calculator buttons 56x56px | Uniform button sizing with light-gray border |
| 2026-01-18 | Clock format: Day H:MM:SS AM/PM | macOS-style time display with seconds |
| 2026-01-18 | Monospace clock font | SF Mono with tabular-nums for consistent width |

## Open Issues

None - v1.1 complete.

## Todos

**Pending:** 3 (in .planning/todos/pending/)

- Persist window state across page refresh (with reset mechanism)
- Add vibe coded attribution in bottom left
- Allow windows to drag off left edge

**Completed:** 6 (from v1.1)
- Fix dock icon sizes to be uniform (FIX-004)
- Remove white padding/border in Finder window (FIX-005)
- Fix calculator window content clipping (FIX-006)
- Fix clock format to H:MM:SS AM/PM (FIX-008)
- Add seconds to clock display (FIX-009)
- Add "Built at" prefix to timestamp (FIX-007)

---

*State updated: 2026-01-18*
*v1.1 System Polish milestone shipped: 2026-01-18*
