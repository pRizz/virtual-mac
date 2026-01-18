---
created: 2026-01-18T05:10
title: Persist individual app state across refresh
area: apps
files:
  - src/calculator.rs
  - src/terminal.rs
  - src/textedit.rs
---

## Problem

Individual app state is lost on page refresh. While Notes and the filesystem already persist, other apps don't:

- Calculator: current display value, pending operation
- Terminal: command history, current directory
- TextEdit: open document content (if not saved to VFS)

## Solution

For each app that has meaningful state:
1. Define what state matters (Calculator: display, operation; Terminal: history, cwd)
2. Save to localStorage on change (debounced)
3. Restore on app open
4. Use app-specific keys: `virtualmac_calculator`, `virtualmac_terminal`, etc.

This depends on the window state persistence feature being complete first.
