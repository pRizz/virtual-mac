---
created: 2026-01-18T05:20
title: Persist dock pinned apps and running state
area: dock
files:
  - src/dock.rs
  - src/window_manager.rs
---

## Problem

The dock currently has hardcoded items with static `is_running` flags. To persist dock state:
1. The dock item list needs to be dynamic (signal-based)
2. Running state should derive from window manager (which windows are open)
3. Pinned apps should persist to localStorage

## Solution

1. **Make dock items reactive:**
   - Change `apps` vec to a signal
   - Derive `is_running` from window manager state

2. **Persist pinned apps:**
   - Save list of pinned app names to `virtualmac_dock`
   - Load on startup

3. **Connect to window manager:**
   - Provide context or signal for open windows
   - Dock reads this to show running indicators

This depends on window persistence being complete first.
