---
phase: 01-calculator-polish
plan: 02
subsystem: ui
tags: [leptos, wasm, keyboard-events, calculator, rust]

# Dependency graph
requires:
  - phase: 01-calculator-polish
    provides: Calculator CSS with rounded buttons and active state styling
provides:
  - Calculator keyboard input support (0-9, operators, Enter, Escape, Backspace)
  - Active operator visual highlighting
  - AC/C button toggle based on state
  - Number formatting with thousands separators
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Global keyboard event listener via wasm_bindgen Closure"
    - "Dynamic CSS class based on signal state"
    - "AC/C toggle based on operation state"

key-files:
  created: []
  modified:
    - src/calculator.rs

key-decisions:
  - "Inlined keyboard handler logic rather than closure cloning for simpler implementation"
  - "Used comma as thousands separator (US locale style)"
  - "AC shows C when current_op != None OR stored_value != 0"

patterns-established:
  - "Keyboard event pattern: Effect + Closure + document.add_event_listener"
  - "Number formatting pattern: format_with_separators for integers"

# Metrics
duration: 3min
completed: 2026-01-17
---

# Phase 01 Plan 02: Calculator Keyboard & Interaction Summary

**Keyboard shortcuts, active operator highlighting, AC/C toggle, and thousands separators for Calculator**

## Performance

- **Duration:** 3 min
- **Started:** 2026-01-17T17:39:20Z
- **Completed:** 2026-01-17T17:42:01Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments
- Calculator responds to keyboard input: digits (0-9), decimal, operators (+, -, *, /), Enter/= for calculate, Escape/c/C for clear, Backspace/Delete to remove digits, % for percent
- Active operator button shows inverted colors (white background, orange text) when selected
- AC button toggles to "C" when there's a pending operation or stored value
- Numbers display with thousands separators (e.g., 1,000,000)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add active operator state tracking and visual highlighting** - `f772474` (feat)
2. **Task 2: Add keyboard support with global event listener** - `62eec1a` (feat)
3. **Task 3: Add AC/C toggle and number formatting with thousands separators** - `363d099` (feat)

## Files Created/Modified
- `src/calculator.rs` - Added keyboard event listener, active operator signal, AC/C toggle, and number formatting with thousands separators (143 lines -> 359 lines)

## Decisions Made
- **Inlined keyboard handler logic:** Rather than cloning closures, inlined the calculation and state-setting logic directly in the match arms. This is simpler and avoids potential closure borrowing issues.
- **US-style thousands separator:** Used comma (,) as the thousands separator, matching macOS Calculator in US locale.
- **AC/C toggle condition:** Shows "C" when `current_op != Operation::None` OR `stored_value != 0.0`, covering both mid-calculation and result states.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Calculator has full macOS-style interaction: keyboard support, operator highlighting, AC/C toggle, number formatting
- Ready for additional Calculator features if needed, or can proceed to next app (Terminal)
- CSS active state styling was already present from plan 01-01

---
*Phase: 01-calculator-polish*
*Completed: 2026-01-17*
