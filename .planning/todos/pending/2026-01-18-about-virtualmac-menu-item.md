---
created: 2026-01-18T05:30
title: Implement About VirtualMac menu item with 1:1 macOS styling
area: ui
files:
  - src/menu_bar.rs
  - src/modals.rs
  - styles.css
  - inspiration-screenshots/about-screen.png
---

## Problem

The "About This Mac" menu item shows generic macOS info. Need a custom "About VirtualMac" option that:
1. Displays a 1:1 macOS-inspired about screen
2. Shows relevant project information
3. Credits that it was vibe coded by Peter Ryszkiewicz
4. Credits assistance from GSD, Ralph, Cursor, and Claude Code

## Solution

1. Reference the screenshot at `inspiration-screenshots/about-screen.png`
2. Create an AboutVirtualMac modal with matching styling
3. Include project-specific info (version, tech stack, credits)
4. Replace or supplement the current AboutThisMac modal

## Reference

See: inspiration-screenshots/about-screen.png
