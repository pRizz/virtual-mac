---
created: 2026-01-18T14:55
title: Show active/foreground app name in menu bar instead of VirtualMac
area: ui
files:
  - src/menu_bar.rs
  - src/window_manager.rs
  - src/system_state.rs
---

## Problem

Currently the menu bar always shows "VirtualMac" as the app name (next to the Apple logo). On real macOS, this displays the name of the currently active/foreground application.

When the user clicks on a window or brings it to the front, the menu bar should update to show that app's name (e.g., "Finder", "Calculator", "Terminal", etc.).

## Solution

1. Track the active/foreground window in system state or window manager
   - Add `active_app: RwSignal<Option<String>>` to SystemState
   - Update when window gains focus (z-index becomes highest)

2. Update menu bar to read from this signal
   - Replace static "VirtualMac" label with reactive app name
   - Fall back to "Finder" when no windows are focused (like real macOS)

3. The "VirtualMac" menu should also update its items based on active app
   - Show app-specific menu items when relevant
   - (Can be deferred - just showing the name is the core feature)
