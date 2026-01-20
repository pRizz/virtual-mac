# Roadmap: v2.0 Persistence & Polish

**Milestone:** v2.0
**Status:** In Progress
**Phases:** 5
**Research:** Completed (see .planning/research/SUMMARY.md)

## Phase Overview

| Phase | Name | Goal | Requirements | Status |
|-------|------|------|--------------|--------|
| 1 | Calculator Persistence | Establish persistence patterns with simplest app | REQ-2 | ✓ Complete |
| 2 | Terminal & TextEdit Persistence | Extend patterns to remaining apps | REQ-3, REQ-4 | ✓ Complete |
| 3 | Dock State | Dynamic running indicators + pinned apps | REQ-5 | ✓ Complete |
| 4 | Notification Polish | macOS-style animations | REQ-6 | Planned |
| 5 | About VirtualMac | Credits dialog in Apple menu | REQ-1 | Planned |

## Phase Details

### Phase 1: Calculator Persistence

**Goal:** Establish localStorage persistence patterns with the simplest app (Calculator) as the reference implementation.

**Delivers:**
- Calculator memory value (M+/M-/MR/MC) functionality
- Persistence infrastructure: save/load functions, schema versioning
- `virtualmac_calculator` localStorage key

**Requirements covered:** REQ-2 (Calculator State Persistence)

**Key files:**
- `src/calculator.rs` - Add memory signals and persistence

**Pattern:** Follow Notes app component-level persistence pattern.

**Plans:** 1 plan

Plans:
- [x] 01-01-PLAN.md - Add memory functionality and localStorage persistence

---

### Phase 2: Terminal & TextEdit Persistence

**Goal:** Apply Phase 1 patterns to Terminal (command history, cwd) and TextEdit (document content).

**Delivers:**
- Terminal command history persistence (max 1000 entries)
- Terminal current working directory persistence
- TextEdit document content persistence
- TextEdit toolbar settings persistence (font, size, alignment)
- `virtualmac_terminal` and `virtualmac_textedit` localStorage keys

**Requirements covered:** REQ-3 (Terminal), REQ-4 (TextEdit)

**Key files:**
- `src/terminal.rs` - Add history/cwd persistence
- `src/textedit.rs` - Add content/settings persistence

**Dependencies:** Phase 1 (patterns established)

**Plans:** 2 plans (Wave 1 - parallel)

Plans:
- [x] 02-01-PLAN.md - Terminal command history and cwd persistence
- [x] 02-02-PLAN.md - TextEdit content and toolbar settings persistence
- [x] 02-03-PLAN.md - Gap closure: TextEdit toolbar settings applied on mount

---

### Phase 3: Dock State

**Status:** Complete (2026-01-19)

**Goal:** Ensure running indicators work correctly and add pinned apps persistence.

**Delivers:**
- Verified dynamic running indicators (derive from WindowManager)
- Pinned apps list persistence
- `virtualmac_dock` localStorage key

**Requirements covered:** REQ-5 (Dock State Persistence)

**Key files:**
- `src/dock.rs` - Verify running indicators, add persistence

**Dependencies:** Phase 1-2 (persistence patterns proven)

**Research needed:** Verify running indicator derivation from WindowManager/SystemState

**Estimated scope:** Small-Medium - one file, integration verification

**Plans:** 1 plan

Plans:
- [x] 03-01-PLAN.md - Dock state persistence and running indicators

---

### Phase 4: Notification Polish

**Goal:** Polish notification animations to match macOS Big Sur style with smooth entrance/exit animations, hover-to-pause, and click-to-dismiss.

**Delivers:**
- Smooth entrance animation (400ms ease-out, slide from right)
- Slide-out exit animation on dismiss (400ms ease-in)
- Hover pauses auto-dismiss timer
- Click anywhere to dismiss
- Max 3 visible notifications (queued beyond)
- Soft diffuse macOS Big Sur shadow
- App icon rendering (use existing icon field)

**Requirements covered:** REQ-6 (Notification System Polish)

**Key files:**
- `styles.css` - Notification animation keyframes and styling
- `src/notification.rs` - Exit state, hover pause, click dismiss

**Dependencies:** None (independent of persistence phases)

**Plans:** 2 plans

Plans:
- [ ] 04-01-PLAN.md - CSS animations and styling (entrance/exit, shadow, hover)
- [ ] 04-02-PLAN.md - Rust state management (exit state, hover pause, click dismiss, max 3)

---

### Phase 5: About VirtualMac

**Goal:** Add About VirtualMac menu item and dialog with credits.

**Delivers:**
- "About VirtualMac" menu item in Apple menu
- Modal dialog with version, build info, and credits
- macOS-style About dialog appearance

**Requirements covered:** REQ-1 (About VirtualMac Menu Item)

**Key files:**
- `src/menu_bar.rs` - Add menu item
- `src/modals.rs` - Add or update About dialog
- `src/system_state.rs` - Add ModalType if needed
- `styles.css` - About dialog styling

**Dependencies:** None (modal system exists)

**Estimated scope:** Small - content and styling

---

## Phase Ordering Rationale

1. **Phase 1 first:** Calculator is the simplest app for persistence. Establishes patterns that all subsequent phases follow.

2. **Phase 2 after Phase 1:** Terminal and TextEdit are more complex but use the same patterns. Having Calculator working validates the approach.

3. **Phase 3 after Phase 2:** Dock state depends on understanding the full persistence pattern. Also needs to verify WindowManager integration for running indicators.

4. **Phase 4 parallel-capable:** Notification polish is CSS-focused and independent. Can be done anytime after Phase 1.

5. **Phase 5 flexible:** About dialog is self-contained. Can slot anywhere, placed last as lower priority.

## Success Criteria

Milestone is complete when:
- [ ] All 6 requirements (REQ-1 through REQ-6) pass acceptance criteria
- [ ] Page refresh preserves: Calculator memory, Terminal history/cwd, TextEdit content, Dock pinned apps
- [ ] Notifications animate smoothly in and out
- [ ] About VirtualMac dialog shows version and credits
- [ ] No console errors from storage operations
- [ ] UAT verification passes

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| localStorage quota exhaustion | Implement history limits (1000 terminal commands) |
| Schema migration failures | Add version field from start, use `#[serde(default)]` |
| Save frequency causing jank | Batch updates, save on meaningful events |
| Private browsing mode | Graceful degradation, warn users if needed |

---
*Roadmap created: 2026-01-19*
*Phase 1 complete: 2026-01-19*
*Phase 2 planned: 2026-01-19*
*Phase 4 planned: 2026-01-20*
*Based on research synthesis: .planning/research/SUMMARY.md*
