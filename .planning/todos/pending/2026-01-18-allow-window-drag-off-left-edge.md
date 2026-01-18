---
created: 2026-01-18T04:59
title: Allow windows to drag off left edge
area: window-system
files:
  - src/window_manager.rs
---

## Problem

Currently windows may be blocked from being dragged off the left edge of the desktop. Users should be able to drag windows partially off-screen to the left (just like they can on real macOS), allowing them to hide part of a window while keeping it accessible.

The top edge constraint (MENU_BAR_HEIGHT = 25px) is correct and should remain, but the left edge should allow negative X positions.

## Solution

- Check window_manager.rs mouse handlers for any `.max(0.0)` constraints on x position
- Remove or adjust left edge constraints to allow negative x values
- Windows should still be draggable back on screen
