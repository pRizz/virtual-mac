---
title: Remove white padding/border in Finder window
area: finder
created: 2026-01-17
source: user-request
---

# Finder White Padding/Border

## Context
There is still a white padding/border visible within the Finder window that needs to be removed.

Note: Plan 05-01 Task 2 attempted to fix this, but user reports it's still visible.

## What needs to be done
- Inspect Finder window in browser dev tools to identify source of white padding
- Remove the white padding/border completely
- Ensure fix works across all view modes (Icon, List, Column)

## Files likely involved
- src/finder.rs
- src/window_manager.rs
- styles.css
