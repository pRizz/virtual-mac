---
created: 2026-01-18T04:33
title: Add vibe coded attribution in bottom left
area: desktop
files:
  - src/desktop.rs
---

## Problem

Need to add attribution text in the bottom left corner of the desktop that says:
"Vibe coded by Peter Ryszkiewicz and GSD, Claude, Cursor, and Ralph Orchestrator"

The text should be positioned so it doesn't overlap with or get too close to the dock (which is centered at the bottom). May need:
- Line break in the middle of the sentence
- Constrained width to keep text on the left side
- Proper z-index so it appears above the wallpaper but below windows

## Solution

- Add a new element in desktop.rs similar to the build-time span (bottom right)
- Position with CSS: bottom-left, max-width constraint
- Consider wrapping text or using two lines
- Ensure adequate margin from dock area
