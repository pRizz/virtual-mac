# Project Research Summary

**Project:** VirtualMac v2.0 - Persistence & Polish
**Domain:** Browser-based macOS desktop simulation (Rust/Leptos 0.7 CSR WebAssembly)
**Researched:** 2026-01-19
**Confidence:** HIGH

## Executive Summary

VirtualMac v2.0 focuses on two categories of improvements: **state persistence** for Calculator, Terminal, TextEdit, and Dock, plus **notification system polish**. The research reveals that the codebase already contains well-proven patterns for both areas. The Notes app, WindowManager, and VirtualFileSystem demonstrate three variations of localStorage persistence (component-level, struct-with-methods, and schema-versioned), all using the same `serde_json` + `web_sys::Storage` stack. No new dependencies are required.

The recommended approach is to extend existing patterns to the remaining apps. Calculator persistence is simplest (memory value only). Terminal persistence requires command history and current working directory but should NOT persist output history. TextEdit should persist document content. Dock state derives running indicators from WindowManager (already works) and should persist pinned apps list. Notification polish is primarily CSS work (exit animations, smoother entrance) with the existing architecture.

Key risks center on **schema migration** (persisted data structures must be versioned from the start), **localStorage quota exhaustion** (Terminal history can grow unbounded), and **save frequency performance** (Effects triggering on every signal change can cause UI jank). All three are addressed by establishing proper patterns in Phase 1 before adding more persisted state.

## Key Findings

### Recommended Stack

No new dependencies needed. Extend existing patterns using:

**Core technologies:**
- `web-sys::Storage` (0.3): localStorage API access - already used throughout codebase
- `serde` + `serde_json` (1.0): serialization with derive macros - proven in Notes, WindowManager, FileSystem
- Leptos `Effect` (0.7): reactive auto-save on signal changes - established pattern
- CSS `@keyframes`: notification animations - existing infrastructure

### Expected Features

**Must have (table stakes):**
- Calculator state restoration (memory value, pending operations)
- Terminal command history and cwd persistence
- TextEdit document content persistence
- Notification slide-in/out animations
- Dock running indicators (already works via SystemState)

**Should have (competitive):**
- Smooth notification exit animation (CSS only, low effort)
- App icons in notifications (field exists, just unused)
- About VirtualMac dialog (modal system exists, just needs content)
- Dock pinned apps persistence

**Defer (v2+):**
- Dock customization (pin/unpin via context menu)
- Notification Center panel
- Per-app notification settings
- Notification sounds

### Architecture Approach

The codebase follows a consistent pattern: app-specific state stays local to components (not elevated to context), persistence uses localStorage with `virtualmac_` key prefix, and Effects handle auto-save. This pattern should continue for new persistence. The architecture research identified three established patterns (component-level, struct-with-methods, schema-versioned) - use the simplest that fits each app.

**Major components:**
1. **Calculator**: Simple component-level persistence (memory value only)
2. **Terminal**: Component-level persistence (command history, cwd)
3. **TextEdit**: Component-level persistence (document content, toolbar settings)
4. **Dock**: Derive running state from WindowManager, persist pinned apps
5. **NotificationContainer**: CSS polish only, no architectural changes

### Critical Pitfalls

1. **Schema version migration failures** - Add version fields to ALL persisted structures from the start. Current code checks version but doesn't migrate. Use `#[serde(default)]` on new fields for forward compatibility.

2. **QuotaExceededError crashes** - localStorage has ~5MB limit. Current `let _ = storage.set_item(...)` silently ignores quota errors. Add error handling and implement history limits (e.g., 1000 terminal commands).

3. **Save frequency performance degradation** - Effects trigger on every signal change, causing UI jank during rapid interactions. Debounce saves (500ms after last change) and save on meaningful events (drag end, not drag move).

4. **Private browsing mode failures** - Safari private mode has stricter restrictions. Current code handles this gracefully (skips persistence) but users should be warned that state won't persist.

5. **Memory leaks from notification closures** - Current `Closure::once().forget()` leaks memory for each notification. Replace with a single interval checking expiry times.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Infrastructure & Calculator Persistence
**Rationale:** Establish proper persistence patterns before adding more persisted state. Calculator is simplest app, good for validating patterns.
**Delivers:** Debounced save utilities, schema versioning template, error handling for storage operations, Calculator memory persistence
**Addresses:** Calculator state restoration (table stakes)
**Avoids:** Schema migration failures, quota errors, performance degradation

### Phase 2: Terminal & TextEdit Persistence
**Rationale:** Build on Phase 1 patterns. Terminal and TextEdit are slightly more complex but follow same architecture.
**Delivers:** Terminal command history/cwd persistence, TextEdit document persistence
**Addresses:** Terminal state restoration, TextEdit state restoration (table stakes)
**Uses:** Persistence patterns from Phase 1
**Avoids:** Unbounded history growth (implement limits), circular reference serialization

### Phase 3: Dock State
**Rationale:** Dock depends on understanding running apps from WindowManager. After persistence patterns are established.
**Delivers:** Dynamic running indicators (verify existing works), pinned apps persistence
**Addresses:** Dock running indicators, dock pinned apps persistence
**Implements:** Dock state struct with localStorage persistence
**Avoids:** Dead window references in running apps list

### Phase 4: Notification Polish
**Rationale:** Independent of persistence work, can be done in parallel. Primarily CSS changes.
**Delivers:** Smooth entrance animation with subtle bounce, slide-out exit animation, app icon rendering
**Addresses:** Notification animations (table stakes), app icons (should have)
**Avoids:** Z-index conflicts with modals, memory leaks from closures

### Phase 5: About VirtualMac
**Rationale:** Modal system exists, just needs content. Low complexity, can slot anywhere.
**Delivers:** About VirtualMac menu item and dialog with version/credits
**Addresses:** About dialog (table stakes)
**Uses:** Existing modal system (ModalType enum, AboutThisMac pattern)

### Phase Ordering Rationale

- **Phase 1 first:** Establishes persistence patterns, error handling, and debouncing that all subsequent phases depend on
- **Phases 2-3 sequential:** Each adds persistence following Phase 1 patterns; Dock verifies WindowManager integration
- **Phase 4 parallel-capable:** Notification work is CSS-focused and independent of persistence
- **Phase 5 flexible:** About dialog is self-contained, can slot after Phase 1

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2 (Terminal):** Multi-window Terminal instances need design decision (shared vs per-window history)
- **Phase 3 (Dock):** Verify running indicator derivation works correctly before adding customization

Phases with standard patterns (skip research-phase):
- **Phase 1 (Calculator):** Direct application of Notes pattern
- **Phase 4 (Notifications):** CSS-only changes, well-documented animations
- **Phase 5 (About):** Modal system is proven, just content changes

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All technologies already in use, no new dependencies |
| Features | HIGH | Clear requirements, existing implementations to reference |
| Architecture | HIGH | Direct codebase analysis, patterns documented with line numbers |
| Pitfalls | HIGH | Identified from existing code issues and MDN documentation |

**Overall confidence:** HIGH

### Gaps to Address

- **Multi-tab synchronization:** Current implementation has no cross-tab awareness. Opening VirtualMac in two tabs will cause state conflicts. Decision needed: warn users, implement sync, or accept last-write-wins.

- **Terminal multi-window strategy:** If multiple Terminal windows are supported, need to decide between shared command history (realistic) or per-window history. Currently unclear from codebase.

- **TextEdit VFS integration:** Two approaches possible - auto-save to VirtualFileSystem or localStorage draft. VFS integration is more realistic but adds complexity. Decision needed during Phase 2 planning.

- **Notification priority levels:** Current fixed 5-second timeout may be insufficient for critical vs informational notifications. Consider adding priority system during Phase 4.

## Sources

### Primary (HIGH confidence)
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notes.rs` (lines 78-109) - Component save/load pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/file_system.rs` (lines 383-418) - Struct method pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/window_manager.rs` (lines 196-269) - Schema versioning pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notification.rs` - Existing notification implementation
- [Apple Human Interface Guidelines - Notifications](https://developer.apple.com/design/human-interface-guidelines/patterns/managing-notifications/)

### Secondary (MEDIUM confidence)
- [Storage quotas and eviction criteria - MDN](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria)
- [macOS Dock MDM Documentation](https://developer.apple.com/documentation/devicemanagement/dock)

### Tertiary (LOW confidence)
- Community discussions on Terminal session restoration behavior

---
*Research completed: 2026-01-19*
*Ready for roadmap: yes*
