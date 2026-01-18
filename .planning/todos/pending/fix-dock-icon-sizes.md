---
title: Fix dock icon sizes to be uniform
area: dock
created: 2026-01-17
source: user-request
---

# Fix Dock Icon Sizes

## Context
The Terminal app icon in the dock is wrongly sized. Some other app icons are also not equivalently sized - they should all be the same size just like on macOS.

## What needs to be done
- Audit all dock icons for size consistency
- Ensure all icons render at the same dimensions
- Terminal icon specifically needs fixing
- Match macOS dock icon uniformity

## Files likely involved
- src/dock.rs
- styles.css (dock icon styling)
