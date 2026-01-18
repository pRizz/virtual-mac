---
created: 2026-01-18T04:22
title: Persist window state across page refresh
area: window-system
files:
  - src/window_manager.rs
---

## Problem

When the user refreshes the page or navigates back to the website, all window positions and states are lost. The desktop resets to the initial default state every time.

Users expect:
1. Windows to remain in the same positions they left them
2. The same apps to be open that were open before
3. Window sizes to be preserved

Additionally, users need an escape hatch to reset the desktop back to factory defaults if state becomes corrupted or they just want a fresh start.

## Solution

1. **Persist state to localStorage:**
   - Save window positions, sizes, z-order on each change
   - Save which apps are open
   - Restore state on page load

2. **Reset mechanism:**
   - Add "Reset Desktop" option to Apple menu (top-left)
   - Clears localStorage and refreshes to factory defaults
   - Consider confirmation dialog to prevent accidental resets
