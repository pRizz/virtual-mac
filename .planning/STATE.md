# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-19)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v2.0 Persistence & Polish

## Current Position

**Milestone:** v2.0 Persistence & Polish
**Status:** Planning
**Started:** 2026-01-19

Progress: [----------] 0% (0 of 5 phases)

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Shipped | 2026-01-18 |
| v2.0 | Persistence & Polish | Active | - |

## Session Continuity

**Last Session:** 2026-01-19
- Started v2.0 Persistence & Polish milestone
- Completed research phase (STACK.md, FEATURES.md, ARCHITECTURE.md, SUMMARY.md)
- Created REQUIREMENTS.md (6 requirements)
- Created ROADMAP.md (5 phases)

**Context for Next Session:**
- v2.0 milestone planned and ready for execution
- Start with Phase 1: Calculator Persistence

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

**In v2.0 scope:** (now tracked in REQUIREMENTS.md)

- REQ-1: About VirtualMac menu item with credits
- REQ-2: Calculator state persistence
- REQ-3: Terminal state persistence
- REQ-4: TextEdit state persistence
- REQ-5: Dock state persistence
- REQ-6: Notification system polish

**Completed (post-v1.1):**
- Show active app name in menu bar
- Minimized windows in dock
- Allow windows to drag off left edge
- Add vibe coded attribution

---

*State updated: 2026-01-19*
*v2.0 Persistence & Polish milestone started: 2026-01-19*
