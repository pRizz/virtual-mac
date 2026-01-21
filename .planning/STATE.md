# Project State - VirtualMac

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-20)

**Core value:** Authentic macOS desktop experience in the browser
**Current focus:** v2.2 Notes Parity

## Current Position

**Milestone:** v2.2 Notes Parity
**Phase:** 2 of 2 (Notes App 1:1 Parity)
**Plan:** 1 of 1 in current phase
**Status:** Phase complete
**Started:** 2026-01-21
**Last activity:** 2026-01-21 - Completed 07-01-PLAN.md

Progress: [██████████] 100%

## Milestones

| Version | Name | Status | Shipped |
|---------|------|--------|---------|
| v1.0 | App Polish | Shipped | 2026-01-17 |
| v1.1 | System Polish | Shipped | 2026-01-18 |
| v2.0 | Persistence & Polish | Shipped | 2026-01-20 |
| v2.1 | Polish | Shipped | 2026-01-21 |
| v2.2 | Notes Parity | Shipped | 2026-01-21 |

## Session Continuity

**Last Session:** 2026-01-21

- Completed Phase 7 Notes app 1:1 parity plan
- Added pin/sort behavior with pinned grouping in Notes list

**Context for Next Session:**
- Plan next milestone scope after Notes parity updates
- Verify any remaining Notes parity gaps before next phase

## Key Decisions

| Date | Decision | Context |
|------|----------|---------|
| 2026-01-21 | Pinned notes sort ahead of selected mode | Mirrors macOS Notes pinned behavior |
| 2026-01-21 | Pinned section uses list header entry | Keeps pinned label in scrollable list |
| 2026-01-21 | Active app names from AppType | Aligns dock and menu bar labels with app catalog |
| 2026-01-21 | Dock indicators rerender from signals | Ensures running/active pips update immediately |
| 2026-01-20 | Separate overlay for AboutVirtualMac | X button only, no click-outside-to-close |
| 2026-01-20 | Dialog at 1/3 viewport height | Matches macOS About dialog positioning |
| 2026-01-20 | Y constraint >= 25px | Keep titlebar accessible above menu bar |
| 2026-01-20 | 400ms animation timing | Entrance ease-out, exit ease-in-out for smooth feel |
| 2026-01-20 | translateX(100%) for slide | Full off-screen slide instead of fixed 100px |
| 2026-01-20 | brightness(1.05) hover | Subtle feedback without dimming notification content |
| 2026-01-20 | Collapse animation on exit | Height/padding collapse causes smooth stacking |
| 2026-01-20 | Reactive exiting lookup | Look up exiting state from signal, not static prop |
| 2026-01-20 | Max 3 visible notifications | Beyond 3, queue until space opens |
| 2026-01-20 | Terminal notify command | Easy testing: `notify <title> [message]` |
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

**v2.0 scope complete:**

- ~~REQ-1: About VirtualMac menu item with credits~~ (DONE)
- ~~REQ-2: Calculator state persistence~~ (DONE)
- ~~REQ-3: Terminal state persistence~~ (DONE)
- ~~REQ-4: TextEdit state persistence~~ (DONE)
- ~~REQ-5: Dock state persistence~~ (DONE)
- ~~REQ-6: Notification system polish~~ (DONE)

**Completed (post-v1.1):**
- Calculator memory persistence (M+/M-/MR/MC with localStorage)
- Terminal persistence (command history + cwd with localStorage)
- TextEdit persistence (document content + toolbar settings with localStorage)
- Dock persistence (pinned apps with localStorage)
- Notification polish (macOS Big Sur animations, hover pause, click dismiss)
- About VirtualMac dialog (draggable, credits, links)
- Show active app name in menu bar
- Minimized windows in dock
- Allow windows to drag off left edge
- Add vibe coded attribution

---

*State updated: 2026-01-21*
*v2.0 milestone complete: 2026-01-20*
*v2.1 milestone complete: 2026-01-21*
