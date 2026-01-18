---
title: Fix calculator window content clipping
area: calculator
created: 2026-01-17
source: user-request
---

# Calculator Window Content Clipping

## Context
The calculator window appears to be clipping its content. Need to investigate and fix.

## What needs to be done
- Investigate why content is being clipped
- Check if window sizing is correct for calculator content
- Check CSS overflow settings
- Ensure all buttons and display are fully visible

## Files likely involved
- src/calculator.rs
- src/window_manager.rs
- styles.css (calculator-specific and window styles)
