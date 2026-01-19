# Phase 3 Context: Dock State

## Vision

Dock state should feel authentic and stable across refresh. Start with a reasonable default pinned set of apps, and persist the entire dock state so users return to the same layout and indicators they had before. Users should be able to re-add default apps later via another mechanism (not in this phase).

## Essentials

- Persist all dock state: pinned apps, order, running indicators (as derived state), minimized state if part of dock model, and any related dock preferences.
- Running indicators must be tied strictly to open windows (not app launch events without windows).
- Default pinned set should be sensible and represent core apps.

## Boundaries

- No new visual tweaks for indicators in this phase.
- No extra edge-case handling required beyond standard persistence patterns.
- Re-add mechanism can be a future phase; do not implement here.

## Notes

- Ensure pinned apps can be re-added later via another mechanism.
- Keep parity with existing persistence patterns (localStorage, schema versioning).
