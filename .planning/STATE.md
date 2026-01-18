# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-17)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v1.1 System Polish — fix bugs and rough edges

## Current Position

**Milestone:** v1.1 System Polish
**Phase:** 1 of 3 (01-window-system-fixes)
**Plan:** 1 of 1 complete
**Status:** Phase complete
**Last activity:** 2026-01-18 — Completed 01-01-PLAN.md

Progress: [###       ] 33% (1 of 3 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | In Progress | - |

## Session Continuity

**Last Session:** 2026-01-18
- Executed 01-01-PLAN.md (Window System Fixes)
- Fixed window drag bounds (FIX-001)
- Fixed window title centering (FIX-002)
- Fixed initial Finder AppType (FIX-003)

**Context for Next Session:**
- Phase 1 complete, ready for Phase 2
- Phase 2: UI Polish Fixes (dock icons, Finder padding, Calculator clipping)
- Phase 3: Clock & Display Fixes (format, seconds, build timestamp)

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-18 | MENU_BAR_HEIGHT = 25.0 | Matches CSS --menubar-height variable |
| 2026-01-18 | pointer-events: none on window title | Allows click-through to titlebar for dragging |

## Open Issues

Remaining v1.1 requirements:
- FIX-004: Dock icon sizes
- FIX-005: Finder white padding
- FIX-006: Calculator clipping
- FIX-007: Build timestamp prefix
- FIX-008: Clock format
- FIX-009: Clock seconds

Completed:
- FIX-001: Window drag bounds (01-01)
- FIX-002: Window title centering (01-01)
- FIX-003: Initial Finder AppType (01-01)

## Todos

**Pending:** 3 (in .planning/todos/pending/)
- Fix dock icon sizes to be uniform (FIX-004)
- Remove white padding/border in Finder window (FIX-005)
- Fix calculator window content clipping (FIX-006)

**Completed:** 0

---

*State updated: 2026-01-18*
*Phase 01 Plan 01 completed: 2026-01-18*
