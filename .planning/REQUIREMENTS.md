# Requirements: v2.1 Polish

**Milestone:** v2.1
**Status:** Shipped
**Target:** Fix dock active app indicator pips

## Overview

VirtualMac v2.1 focuses on ensuring dock indicator pips reflect running and active apps with clear, visible styling.

## Requirements

### REQ-1: Active App Indicator Pips ✓

**Priority:** P1
**Source:** Regression - dock indicators missing
**Status:** Complete (Phase 6)

Dock indicator pips show running apps and visually emphasize the active app.

**Acceptance Criteria:**
- [x] Running apps show dock pips consistently
- [x] Active app pip is visually emphasized relative to running apps
- [x] Pips update immediately when app focus changes
- [x] Pips disappear when apps close

## Technical Constraints

- **No new dependencies** - use existing Leptos + CSS styling
- **Signal-driven state** - derive running/active state from existing signals

## Out of Scope

- Dock customization or pin/unpin behavior changes
- Menu bar behavior changes

## Success Metrics

- Running/active pips visible for all standard apps
- No regressions in dock rendering or interactions

---

*Requirements defined: 2026-01-21*
