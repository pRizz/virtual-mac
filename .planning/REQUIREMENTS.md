# Requirements: v2.0 Persistence & Polish

**Milestone:** v2.0
**Status:** Defining
**Target:** State persistence across page refresh + UI polish

## Overview

VirtualMac v2.0 adds persistence for individual app state (Calculator, Terminal, TextEdit) and dock configuration, plus notification system polish. The goal is that users can refresh the page and return to their previous working state.

## Requirements

### REQ-1: About VirtualMac Menu Item

**Priority:** P1
**Source:** Todo - About VirtualMac menu item with credits

Add an "About VirtualMac" menu item in the Apple menu that displays project information and credits.

**Acceptance Criteria:**
- [ ] Apple menu contains "About VirtualMac" item (below existing About This Mac)
- [ ] Clicking opens a modal dialog
- [ ] Dialog shows: VirtualMac logo/icon, version number, build info
- [ ] Dialog shows credits: creator attribution, tools used (Claude Code, GSD, Cursor, Ralph)
- [ ] Dialog matches macOS About dialog styling (centered, rounded corners)
- [ ] Dialog dismissable via close button or clicking outside

### REQ-2: Calculator State Persistence ✓

**Priority:** P1
**Source:** Todo - Persist app state (Calculator, Terminal, TextEdit)
**Status:** Complete (Phase 1)

Calculator state persists across page refresh.

**Acceptance Criteria:**
- [x] Calculator memory value (M+/M-/MR) persists across refresh
- [x] State stored in `virtualmac_calculator` localStorage key
- [x] State includes schema version for future migration
- [x] Graceful degradation if storage unavailable (defaults used)
- [x] No persistence of current display/operation (matches real Calculator behavior)

### REQ-3: Terminal State Persistence ✓

**Priority:** P1
**Source:** Todo - Persist app state (Calculator, Terminal, TextEdit)
**Status:** Complete (Phase 2)

Terminal command history and working directory persist across page refresh.

**Acceptance Criteria:**
- [x] Command history (up/down arrow recall) persists across refresh
- [x] Current working directory persists across refresh
- [x] State stored in `virtualmac_terminal` localStorage key
- [x] History limited to prevent quota exhaustion (max 1000 commands)
- [x] Output history does NOT persist (matches real Terminal behavior)
- [x] Graceful degradation if storage unavailable

### REQ-4: TextEdit State Persistence ✓

**Priority:** P1
**Source:** Todo - Persist app state (Calculator, Terminal, TextEdit)
**Status:** Complete (Phase 2)

TextEdit document content persists across page refresh.

**Acceptance Criteria:**
- [x] Document content (HTML) persists across refresh
- [x] Toolbar settings (font, size, alignment) persist across refresh
- [x] State stored in `virtualmac_textedit` localStorage key
- [x] State includes schema version for future migration
- [x] Graceful degradation if storage unavailable (empty document)

### REQ-5: Dock State Persistence

**Priority:** P1
**Source:** Todo - Persist dock state (pinned apps, running indicators)

Dock configuration persists across page refresh.

**Acceptance Criteria:**
- [ ] Running indicators correctly show for all open app windows
- [ ] Pinned apps list persists across refresh
- [ ] State stored in `virtualmac_dock` localStorage key
- [ ] Default dock apps shown on first load (no prior state)
- [ ] Running indicators derive from WindowManager (not persisted separately)

### REQ-6: Notification System Polish ✓

**Priority:** P2
**Source:** Todo - Polish notification system
**Status:** Complete (Phase 4)

Notifications match macOS visual style more closely.

**Acceptance Criteria:**
- [x] Smooth entrance animation with subtle ease-out
- [x] Slide-out exit animation when dismissed (not abrupt disappear)
- [x] App icons display in notifications (use existing icon field)
- [x] Styling matches macOS (rounded corners, blur backdrop - already mostly done)
- [x] Stacking behavior remains correct (multiple notifications stack)

## Technical Constraints

- **No new dependencies** - use existing serde/serde_json/web-sys stack
- **localStorage only** - no IndexedDB or server-side storage
- **Schema versioning** - all persisted structures must include version field
- **Platform gating** - use `#[cfg(target_arch = "wasm32")]` for storage code
- **Graceful degradation** - always have sensible defaults when storage fails

## Out of Scope

- Dock customization (pin/unpin via context menu) - defer to future milestone
- Notification Center panel with history
- Notification sounds
- Cross-tab synchronization
- Notes/Finder persistence (already implemented)

## Success Metrics

- All 6 requirements pass acceptance criteria
- Page refresh preserves: Calculator memory, Terminal history/cwd, TextEdit content, Dock pinned apps
- No console errors from storage operations
- Notifications animate smoothly in and out

---
*Requirements defined: 2026-01-19*
