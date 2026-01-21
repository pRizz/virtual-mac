---
phase: 06-active-app-indicator-pips
plan: 01
subsystem: ui
tags: [dock, indicators, leptos, css, window-manager]

# Dependency graph
requires:
  - phase: 03-dock-state
    provides: running indicators derived from open windows
provides:
  - Active app tracking derived from app type names
  - Dock indicator classes for running and active apps
  - Brighter dock pip styling and placement adjustments
affects: [dock, menu-bar, window-manager]

# Tech tracking
tech-stack:
  added: []
  patterns: [dock indicators driven by running/active signals]

key-files:
  created: []
  modified: [src/window_manager.rs, src/dock.rs, styles.css]

key-decisions:
  - "Active app names should come from AppType to align dock/menu labels"
  - "Dock indicators should rerender directly from running/active signals"

patterns-established:
  - "Dock pips use running/active classes for visual emphasis"

issues-created: []

# Metrics
duration: 7 min
completed: 2026-01-21
---

# Phase 6 Plan 01: Active App Indicator Pips Summary

**Dock indicators now track running and active apps with reactive pips and brighter styling.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-01-20T19:42:08-06:00
- **Completed:** 2026-01-21T01:50:04Z
- **Tasks:** 2/2
- **Files modified:** 3

## Accomplishments
- Normalized active app tracking to use app type names for consistent labeling.
- Added running/active dock indicator classes and reactive rendering.
- Improved pip visibility with brighter color and adjusted placement.

## Task Commits

Each task was committed atomically:

1. **Task 1: Normalize active app tracking** - `cc650a0` (fix)
2. **Task 2: Render running + active indicator pips** - `556893b` (feat)

Additional adjustments after Task 2:
- `a362fe2` (fix): visibility boost for pips
- `fe00f7d` (fix): reactive dock indicator rendering
- `f0e04fe` (style): brighter, lower pips

## Files Created/Modified
- `src/window_manager.rs` - Use app type names for active app state.
- `src/dock.rs` - Track running/active states and rerender dock indicators reactively.
- `styles.css` - Update dock pip styling for visibility and position.

## Decisions Made
- Use `AppType` display names for active app tracking to align with dock and menu bar labels.
- Drive dock pip rendering from `open_windows` and `active_app` signals for reactivity.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Dock indicators failed to update when running apps changed**
- **Found during:** Task 2 (Render running + active indicator pips)
- **Issue:** Dock items rendered once and did not react to `open_windows` updates, so no pips appeared.
- **Fix:** Wrapped dock item rendering in a reactive closure to rerender on signal changes.
- **Files modified:** `src/dock.rs`
- **Verification:** Running apps now show pips immediately when opened/closed.
- **Committed in:** `fe00f7d` (follow-up to Task 2)

### User-Requested Adjustments

- Brightened pip color and nudged indicator placement lower for visibility (`f0e04fe`).

---

**Total deviations:** 1 auto-fixed (Rule 1), 1 user-requested adjustment
**Impact on plan:** Ensured indicators appear correctly and meet visual expectations.

## Issues Encountered
- Pips initially appeared missing because dock items did not rerender when running state changed; fixed by making dock rendering reactive.

## Next Phase Readiness
- Active and running indicators behave correctly with clear emphasis; phase complete.

---
*Phase: 06-active-app-indicator-pips*
*Completed: 2026-01-21*
