# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-19)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v2.0 Persistence & Polish

## Current Position

**Milestone:** v2.0 Persistence & Polish
**Phase:** 4 of 5 (04-notification-polish)
**Plan:** 1 of 2 complete
**Status:** In progress
**Started:** 2026-01-19
**Last activity:** 2026-01-20 - Completed 04-01-PLAN.md

Progress: [#######---] 70% (3.5 of 5 phases complete)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Shipped | 2026-01-18 |
| v2.0 | Persistence & Polish | Active | - |

## Session Continuity

**Last Session:** 2026-01-20
- Completed Phase 04 Plan 01 (notification CSS polish)
- Added entrance/exit animation keyframes
- Updated shadow to macOS Big Sur style
- Added hover brightness filter

**Context for Next Session:**
- Phase 4 Plan 01 complete (CSS animations ready)
- Continue with Phase 4 Plan 02 (Rust state management for notifications)
- CSS .exiting class ready for Rust to apply on dismiss

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-20 | 400ms animation timing | Entrance ease-out, exit ease-in for smooth feel |
| 2026-01-20 | translateX(100%) for slide | Full off-screen slide instead of fixed 100px |
| 2026-01-20 | brightness(1.05) hover | Subtle feedback without dimming notification content |
| 2026-01-19 | TextEdit uses innerHTML for content | Preserves bold/italic/underline formatting on persistence |
| 2026-01-19 | StoredValue for one-time mount Effect | Cleaner pattern than signal for tracking effect execution |
| 2026-01-19 | Terminal history limit 1000 | Prevents localStorage exhaustion, removes oldest first |
| 2026-01-19 | Output history NOT persisted | Matches real Terminal behavior - only commands persist |
| 2026-01-19 | Memory-only persistence | Display, stored_value, current_op are transient; only memory persists |
| 2026-01-19 | Schema version pattern | CalculatorState includes schema_version for future migrations |
| 2026-01-19 | Dock persistence stores pinned list only | Running indicators derived from open windows |
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
- ~~REQ-3: Terminal state persistence~~ (DONE)
- ~~REQ-4: TextEdit state persistence~~ (DONE)
- ~~REQ-5: Dock state persistence~~ (DONE)
- REQ-6: Notification system polish (in progress)

**Completed (post-v1.1):**
- Calculator memory persistence (M+/M-/MR/MC with localStorage)
- Terminal persistence (command history + cwd with localStorage)
- TextEdit persistence (document content + toolbar settings with localStorage)
- Dock persistence (pinned apps with localStorage)
- Show active app name in menu bar
- Minimized windows in dock
- Allow windows to drag off left edge
- Add vibe coded attribution

---

*State updated: 2026-01-20*
*Phase 04-notification-polish Plan 01 complete: 2026-01-20*
