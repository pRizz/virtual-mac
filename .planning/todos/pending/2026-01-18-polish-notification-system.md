---
created: 2026-01-18T05:10
title: Polish notification system to match macOS
area: ui
files:
  - src/notification.rs
  - styles.css
---

## Problem

The basic notification system (added for schema migration warnings) needs polish to match real macOS notifications 1:1.

## Solution

Match macOS notification appearance and behavior:
- Slide in from top-right with smooth animation
- Rounded rectangle with blur/vibrancy background
- App icon on left, title bold, message below
- Dismiss button (X) on hover
- Auto-dismiss after ~5 seconds
- Stack multiple notifications vertically
- Click to dismiss or take action

Reference: macOS Sonoma notification style
