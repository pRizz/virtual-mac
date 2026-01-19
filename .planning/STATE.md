# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-19)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v2.0 Persistence & Polish

## Current Position

**Milestone:** v2.0 Persistence & Polish
**Phase:** 1 of 5 (01-calculator-persistence)
**Plan:** 1 of 1 complete
**Status:** Phase complete
**Started:** 2026-01-19
**Last activity:** 2026-01-19 - Completed 01-01-PLAN.md

Progress: [##--------] 20% (1 of 5 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Shipped | 2026-01-18 |
| v2.0 | Persistence & Polish | Active | - |

## Session Continuity

**Last Session:** 2026-01-19
- Completed Phase 01 Plan 01: Calculator Memory Persistence
- Calculator now has M+/M-/MR/MC memory buttons
- Memory value persists via localStorage
- Fixed pre-existing clippy warnings in 6 files

**Context for Next Session:**
- Phase 1 (Calculator Persistence) complete
- Continue with Phase 2: Terminal Persistence
- Persistence pattern established in calculator.rs can be reused

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-19 | Memory-only persistence | Display, stored_value, current_op are transient; only memory persists |
| 2026-01-19 | Schema version pattern | CalculatorState includes schema_version for future migrations |
| 2026-01-19 | Orange memory indicator | #ffa500 matches macOS Calculator memory indicator |
| 2026-01-18 | MENU_BAR_HEIGHT = 25.0 | Matches CSS --menubar-height variable |
| 2026-01-18 | pointer-events: none on window title | Allows click-through to titlebar for dragging |
| 2026-01-18 | Dock icon font-size 32px | Better fills 48x48 icon space |
| 2026-01-18 | Explicit emoji font-family | Cross-browser consistency for dock icons |
| 2026-01-18 | Calculator window 280x540 | Accommodates all buttons without clipping |
| 2026-01-18 | Calculator buttons 56x56px | Uniform button sizing with light-gray border |
| 2026-01-18 | Clock format: Day H:MM:SS AM/PM | macOS-style time display with seconds |
| 2026-01-18 | Monospace clock font | SF Mono with tabular-nums for consistent width |

## Open Issues

None.

## Todos

**In v2.0 scope:** (now tracked in REQUIREMENTS.md)

- REQ-1: About VirtualMac menu item with credits
- ~~REQ-2: Calculator state persistence~~ (DONE)
- REQ-3: Terminal state persistence
- REQ-4: TextEdit state persistence
- REQ-5: Dock state persistence
- REQ-6: Notification system polish

**Completed (post-v1.1):**
- Calculator memory persistence (M+/M-/MR/MC with localStorage)
- Show active app name in menu bar
- Minimized windows in dock
- Allow windows to drag off left edge
- Add vibe coded attribution

---

*State updated: 2026-01-19*
*Phase 01-calculator-persistence complete: 2026-01-19*
