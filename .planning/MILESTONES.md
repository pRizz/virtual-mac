# Project Milestones: VirtualMac

## v1.1 System Polish (Shipped: 2026-01-18)

**Delivered:** Nine bug fixes across window system, UI polish, and clock display.

**Phases completed:** 1-3 (3 plans total)

**Key accomplishments:**

- Window drag bounds enforced at menu bar (25px)
- Window titles properly centered relative to full width
- Initial Finder window uses correct AppType
- Uniform dock icon sizing with webkit mask for rounded corners
- Finder window without white padding artifacts
- Calculator properly sized with uniform 56x56px buttons and thousand separators
- Clock displays correct "H:MM:SS AM/PM" format with real-time seconds
- Build timestamp prefixed with "Built at "
- Monospace clock font for consistent digit widths

**Stats:**

- 42 commits
- 76 files changed, 1902 insertions(+), 112 deletions(-)
- 3 phases, 3 plans
- 9/9 requirements satisfied

**Git range:** `v1.0` → `v1.1`

**What's next:** v2.0 features or additional system polish

---

## v1.0 App Polish (Shipped: 2026-01-17)

**Delivered:** Five macOS apps (Calculator, Terminal, Notes, TextEdit, Finder) polished to near-identical replicas of their real counterparts.

**Phases completed:** 1-5 (19 plans total)

**Key accomplishments:**

- Calculator with macOS styling, keyboard support, operator highlighting, and thousands separators
- Terminal with VirtualFileSystem integration, command history, tab completion, and Cmd+K clear
- Notes app with three-column layout, rich text formatting, folder management, and localStorage persistence
- TextEdit with full toolbar (fonts, B/I/U, alignment, colors), document appearance, and status bar
- Finder with three view modes, sidebar sections, search, path bar, context menu, and status bar

**Stats:**

- 12 files created/modified
- 9,646 lines of Rust + CSS
- 5 phases, 19 plans
- 1 day from start to ship

**Git range:** `a82cbf9` → `HEAD`

**What's next:** v2.0 with additional apps (Safari, Messages, Mail, Photos, Music, Calendar) or system polish

---
