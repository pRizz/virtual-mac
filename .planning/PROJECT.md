# VirtualMac

## What This Is

A browser-based macOS desktop simulation built with Rust/WebAssembly and Leptos. Features five polished core applications (Calculator, Terminal, Notes, TextEdit, Finder) that closely replicate the look and behavior of their real macOS counterparts.

## Core Value

The applications feel like real macOS apps — users familiar with macOS immediately recognize the UI and behavior, creating an authentic desktop experience in the browser.

## Current Milestone: v1.1 System Polish

**Goal:** Fix bugs and rough edges from v1.0 — window system, UI polish, and clock fixes.

**Target fixes:**
- Window drag bounds (prevent dragging above menu bar)
- Window title centering (center relative to full width)
- Initial Finder window AppType bug
- Dock icon sizes (uniform sizing)
- Finder white padding/border
- Calculator content clipping
- Build timestamp "Built at " prefix
- Clock format fix ("2:04 PM" not "2 PM:04")
- Clock show seconds

## Current State (v1.0 Shipped)

**Tech stack:** Rust 2021 + Leptos 0.7 (CSR) + WebAssembly
**Codebase:** 9,646 lines (Rust + CSS)
**Apps shipped:** Calculator, Terminal, Notes, TextEdit, Finder

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

### Active

(None — planning next milestone)

### Out of Scope

- Real file system access — browser security prevents this
- Actual process execution in Terminal — simulated commands only
- iCloud sync for Notes — no backend
- System-level keyboard shortcuts that browsers intercept

## Context

**Reference:** `.planning/codebase/` contains detailed architecture, conventions, and structure docs.

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
*Last updated: 2026-01-17 after v1.0 milestone*
