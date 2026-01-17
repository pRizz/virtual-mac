# VirtualMac - App Polish Milestone

## What This Is

A browser-based macOS desktop simulation built with Rust/WebAssembly and Leptos. This milestone focuses on polishing the five core applications (Calculator, Terminal, Notes, TextEdit, Finder) to closely replicate the look and behavior of their real macOS counterparts, within browser constraints.

## Core Value

The applications should feel like real macOS apps — users familiar with macOS should immediately recognize the UI and behavior, creating an authentic desktop experience in the browser.

## Requirements

### Validated

These capabilities exist and are working:

- ✓ Desktop shell with wallpaper and selection rectangle — existing
- ✓ Menu bar with dropdowns and Control Center — existing
- ✓ Dock with magnification effect and tooltips — existing
- ✓ Window management (drag, resize, minimize, maximize, z-index) — existing
- ✓ Theme switching (light/dark mode) — existing
- ✓ Virtual file system with localStorage persistence — existing
- ✓ Spotlight search overlay (Cmd+Space) — existing
- ✓ App switcher (Cmd+Tab) — existing
- ✓ Lock screen and power states — existing
- ✓ Context menus — existing
- ✓ System Settings app — existing

### Active

- [ ] Calculator app matches macOS Calculator (visual + functional)
- [ ] Terminal app matches macOS Terminal (visual + realistic shell)
- [ ] Notes app matches macOS Notes (folders, formatting, search)
- [ ] TextEdit app matches macOS TextEdit (toolbar, formatting, document feel)
- [ ] Finder app matches macOS Finder (sidebar, views, toolbar polish)

### Out of Scope

- Real file system access — browser security prevents this
- Actual process execution in Terminal — simulated commands only
- iCloud sync for Notes — no backend
- System-level keyboard shortcuts that browsers intercept — use alternatives where needed
- Safari, Messages, Mail, Photos, Music, Calendar — defer to future milestone
- Native app launching from Dock/Spotlight — window creation only

## Context

**Technical environment:**
- Rust 2021 + Leptos 0.7 (CSR mode)
- WebAssembly target (wasm32-unknown-unknown)
- Trunk for build/dev server
- No backend — fully client-side

**Current app state:**
- Calculator: Basic layout, functional but doesn't match macOS aesthetic
- Terminal: Command simulation exists but UI is minimal
- Notes: May not exist yet — needs creation
- TextEdit: Basic rich text editor, needs polish
- Finder: File browser works, needs sidebar/toolbar refinement

**Reference:** `.planning/codebase/` contains detailed architecture, conventions, and structure docs.

## Constraints

- **Tech stack**: Must use existing Leptos/Rust/WASM architecture
- **Browser limits**: Cannot access real filesystem, execute processes, or intercept all keyboard shortcuts
- **No new dependencies**: Prefer solving with existing crates unless essential
- **E2E tests**: Currently under moratorium per CLAUDE.md — focus on unit tests if needed

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Near-full replica fidelity | User wants authentic macOS experience | — Pending |
| All 5 apps in v1 | User wants complete app layer polish | — Pending |
| Browser constraints accepted | Reasonable tradeoffs for web platform | — Pending |

---
*Last updated: 2026-01-17 after initialization*
