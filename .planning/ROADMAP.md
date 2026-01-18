# Roadmap: v1.1 System Polish

**Goal:** Fix bugs and rough edges from v1.0 — window system, UI polish, and clock fixes.

**Phases:** 3
**Requirements:** 9 fixes

## Phase 1: Window System Fixes

**Goal:** Fix window dragging, title centering, and Finder AppType issues.

**Requirements:** FIX-001, FIX-002, FIX-003

**Plans:** 1 plan
**Status:** Complete

Plans:
- [x] 01-01-PLAN.md — Fix drag bounds, title centering, and Finder AppType

**Key Deliverables:**
- Window drag bounds enforcement (prevent dragging above menu bar)
- Window title properly centered relative to full width
- Initial Finder window uses correct AppType

**Success Criteria:**
- Windows cannot be dragged above menu bar
- Titles centered at all window widths
- Finder opens with AppType::Finder

---

## Phase 2: UI Polish Fixes

**Goal:** Fix dock icon sizing, Finder white padding, and Calculator clipping.

**Requirements:** FIX-004, FIX-005, FIX-006

**Plans:** 1 plan
**Status:** Complete

Plans:
- [x] 02-01-PLAN.md — Fix dock icon sizing, Finder padding, and Calculator clipping

**Key Deliverables:**
- Uniform dock icon sizes
- Clean Finder window without white padding
- Calculator content fully visible

**Success Criteria:**
- All dock icons same base size
- No white border/padding in Finder
- Calculator buttons and display not clipped

---

## Phase 3: Clock & Display Fixes

**Goal:** Fix clock format, add seconds, and update build timestamp display.

**Requirements:** FIX-007, FIX-008, FIX-009

**Key Deliverables:**
- Correct clock format (H:MM:SS AM/PM)
- Seconds displayed in menu bar clock
- Build timestamp with "Built at " prefix

**Success Criteria:**
- Clock shows "2:04:30 PM" format
- Seconds update in real-time
- Version display reads "Built at {timestamp}"

---

## Summary

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 1 | Window System Fixes | FIX-001, FIX-002, FIX-003 | Complete |
| 2 | UI Polish Fixes | FIX-004, FIX-005, FIX-006 | ✅ Complete |
| 3 | Clock & Display Fixes | FIX-007, FIX-008, FIX-009 | Pending |

---

*Created: 2026-01-17 for v1.1 System Polish milestone*
