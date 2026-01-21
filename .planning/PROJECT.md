# VirtualMac

## What This Is

A browser-based macOS desktop simulation built with Rust/WebAssembly and Leptos. Features five polished core applications (Calculator, Terminal, Notes, TextEdit, Finder) that closely replicate the look and behavior of their real macOS counterparts.

## Core Value

The applications feel like real macOS apps — users familiar with macOS immediately recognize the UI and behavior, creating an authentic desktop experience in the browser.

## Current Milestone: Planning next milestone

**Goal:** Define the next milestone scope after v2.0.

**Target features:** TBD

## Current State (v2.0 Shipped)

**Tech stack:** Rust 2021 + Leptos 0.7 (CSR) + WebAssembly
**Codebase:** ~13,200 lines (Rust + CSS)
**Apps shipped:** Calculator, Terminal, Notes, TextEdit, Finder
**Milestones shipped:** v1.0 App Polish, v1.1 System Polish, v2.0 Persistence & Polish

## Requirements

### Validated

These capabilities exist and are working:

**Desktop Shell:**
- ✓ Desktop shell with wallpaper and selection rectangle — v0
- ✓ Menu bar with dropdowns and Control Center — v0
- ✓ Dock with magnification effect and tooltips — v0
- ✓ Window management (drag, resize, minimize, maximize, z-index) — v0
- ✓ Theme switching (light/dark mode) — v0
- ✓ Virtual file system with localStorage persistence — v0
- ✓ Spotlight search overlay (Cmd+Space) — v0
- ✓ App switcher (Cmd+Tab) — v0
- ✓ Lock screen and power states — v0
- ✓ Context menus — v0
- ✓ System Settings app — v0

**App Polish (v1.0):**
- ✓ Calculator matches macOS Calculator (styling, keyboard, operators) — v1.0
- ✓ Terminal matches macOS Terminal (VFS, history, tab completion) — v1.0
- ✓ Notes matches macOS Notes (three-column, formatting, localStorage) — v1.0
- ✓ TextEdit matches macOS TextEdit (toolbar, document feel, colors) — v1.0
- ✓ Finder matches macOS Finder (views, sidebar, search, context menu) — v1.0
- ✓ About VirtualMac menu item with credits — v2.0
- ✓ Persist app state (Calculator, Terminal, TextEdit) — v2.0
- ✓ Persist dock state (pinned apps, running indicators) — v2.0
- ✓ Polish notification system — v2.0

### Active

None yet — planning next milestone.

### Out of Scope

- Real file system access — browser security prevents this
- Actual process execution in Terminal — simulated commands only
- iCloud sync for Notes — no backend
- System-level keyboard shortcuts that browsers intercept

## Context

**Reference:** `.planning/codebase/` contains detailed architecture, conventions, and structure docs.

Shipped v2.0 with persistence across core apps and dock, notification polish, and the About VirtualMac dialog.

## Constraints

- **Tech stack**: Must use existing Leptos/Rust/WASM architecture
- **Browser limits**: Cannot access real filesystem, execute processes, or intercept all keyboard shortcuts
- **No new dependencies**: Prefer solving with existing crates unless essential
- **E2E tests**: Currently under moratorium per CLAUDE.md

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Near-full replica fidelity | User wants authentic macOS experience | ✓ Good |
| All 5 apps in v1 | User wants complete app layer polish | ✓ Good |
| Browser constraints accepted | Reasonable tradeoffs for web platform | ✓ Good |
| execCommand for rich text | No viable replacement for fontName/alignment | ✓ Good |
| VirtualFileSystem shared | Terminal and Finder operate on same FS | ✓ Good |
| localStorage persistence | Notes and VFS persist across sessions | ✓ Good |
| YOLO workflow mode | Faster iteration with fewer confirmations | ✓ Good |

---
*Last updated: 2026-01-20 after v2.0 milestone*
