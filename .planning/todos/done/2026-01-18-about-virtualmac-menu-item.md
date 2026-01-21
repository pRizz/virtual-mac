---
created: 2026-01-18T05:30
title: Implement About VirtualMac menu item with system info and credits
area: ui
files:
  - src/menu_bar.rs
  - src/modals.rs
  - styles.css
  - inspiration-screenshots/about-this-app-screen.png
---

## Problem

The "About This Mac" menu item in the Apple menu (top-left) shows generic macOS info. Need to rename it to "About VirtualMac" and show project-specific info with credits.

## Solution

1. Rename "About This Mac" menu item to "About VirtualMac"

2. Update the AboutThisMac modal to show:
   - Keep the macOS-inspired system info layout (Chip, Memory, etc.)
   - Add "VirtualMac" branding at top
   - Add credits section:
     - "Vibe coded by Peter Ryszkiewicz"
     - "With assistance from GSD, Ralph, Cursor, and Claude Code"

3. Reference `inspiration-screenshots/about-this-app-screen.png` for styling

## Reference

See: inspiration-screenshots/about-this-app-screen.png
