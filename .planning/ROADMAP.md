# Roadmap - App Polish Milestone

## Overview

Five phases, one per application. Each phase brings an app to macOS-replica quality.

## Phase Summary

| Phase | Name | Status | Requirements |
|-------|------|--------|--------------|
| 1 | Calculator Polish | Complete | REQ-001 |
| 2 | Terminal Polish | Complete | REQ-002 |
| 3 | Notes App Creation | Complete | REQ-003 |
| 4 | TextEdit Polish | Complete | REQ-004 |
| 5 | Finder Polish | Complete | REQ-005 |

## Phase Details

### Phase 1: Calculator Polish

**Goal:** Transform the basic calculator into a near-identical replica of macOS Calculator (Basic mode).

**Requirements Covered:** REQ-001 (all sub-requirements)

**Plans:** 2 plans

Plans:
- [x] 01-01-PLAN.md - Visual styling (CSS) for rounded buttons, gaps, display
- [x] 01-02-PLAN.md - Keyboard support, operator highlighting, AC/C toggle, number formatting

**Key Deliverables:**
- Rounded buttons with macOS color scheme
- Proper 4-column grid layout
- Right-aligned display with number formatting
- Button animations and keyboard support
- Correct calculation behavior

**Success Criteria:**
- Visual comparison with macOS Calculator shows near-identical appearance
- All basic arithmetic operations work correctly
- Keyboard input functions as expected

---

### Phase 2: Terminal Polish

**Goal:** Transform the minimal terminal into a near-identical replica of macOS Terminal with realistic shell simulation.

**Requirements Covered:** REQ-002 (all sub-requirements)

**Plans:** 4 plans

Plans:
- [x] 02-01-PLAN.md - VirtualFileSystem integration (remove FsNode, use shared FS)
- [x] 02-02-PLAN.md - Visual styling (CSS) - dark background, monospace font, scrollbar
- [x] 02-03-PLAN.md - Command history navigation and tab completion
- [x] 02-04-PLAN.md - Clear command, Cmd+K shortcut, auto-scroll, final verification

**Key Deliverables:**
- macOS Terminal visual styling
- Command prompt with proper formatting
- Command history and navigation
- Tab completion for paths
- Integration with VirtualFileSystem

**Success Criteria:**
- Visual comparison with macOS Terminal shows near-identical appearance
- Common commands (ls, cd, pwd, cat, mkdir, rm, etc.) work
- Terminal operates on same filesystem as Finder

---

### Phase 3: Notes App Creation

**Goal:** Create a Notes app that closely replicates macOS Notes appearance and functionality.

**Requirements Covered:** REQ-003 (all sub-requirements)

**Plans:** 4 plans

Plans:
- [x] 03-01-PLAN.md - Core component, data model, localStorage persistence, window integration
- [x] 03-02-PLAN.md - CSS styling for three-column layout and macOS Notes appearance
- [x] 03-03-PLAN.md - Rich text formatting toolbar (bold, italic, underline, lists, checklists)
- [x] 03-04-PLAN.md - CRUD operations for notes and folders, search functionality

**Key Deliverables:**
- Three-column layout (folders, notes list, editor)
- Rich text formatting support
- Folder and note management
- Search functionality
- localStorage persistence

**Success Criteria:**
- Visual comparison with macOS Notes shows near-identical appearance
- Users can create, edit, organize, and search notes
- Notes persist across sessions

---

### Phase 4: TextEdit Polish

**Goal:** Transform the basic text editor into a near-identical replica of macOS TextEdit.

**Requirements Covered:** REQ-004 (all sub-requirements)

**Plans:** 3 plans

Plans:
- [x] 04-01-PLAN.md - Core toolbar with font family/size dropdowns, B/I/U buttons, alignment
- [x] 04-02-PLAN.md - Document appearance (white page on gray), text/highlight color pickers
- [x] 04-03-PLAN.md - Status bar with word count, toolbar polish, final visual verification

**Key Deliverables:**
- macOS TextEdit toolbar styling
- Document-style appearance (white page with margins)
- Font and formatting controls
- Text and highlight color pickers
- Status bar with word/character count

**Success Criteria:**
- Visual comparison with macOS TextEdit shows near-identical appearance
- All formatting options function correctly
- Document editing feels native

---

### Phase 5: Finder Polish

**Goal:** Transform the file browser into a near-identical replica of macOS Finder.

**Requirements Covered:** REQ-005 (all sub-requirements)

**Plans:** 6 plans

Plans:
- [x] 05-01-PLAN.md — View mode infrastructure (ViewMode enum, wired toolbar buttons, white padding fix)
- [x] 05-02-PLAN.md — List view implementation with columns (Name, Date, Size, Kind)
- [x] 05-03-PLAN.md — Column view implementation (Miller columns navigation)
- [x] 05-04-PLAN.md — Search filtering and path bar with breadcrumb navigation
- [x] 05-05-PLAN.md — Context menu actions (New Folder, Rename, Move to Trash)
- [x] 05-06-PLAN.md — Sidebar polish (iCloud, Tags), status bar enhancements, visual verification

**Key Deliverables:**
- macOS Finder sidebar styling
- Multiple view modes (icon, list, column)
- Functional toolbar and search
- Working context menu actions
- Path bar and status bar

**Success Criteria:**
- Visual comparison with macOS Finder shows near-identical appearance
- All view modes work correctly
- Context menu actions function as expected
- Search filters results properly

---

## Dependencies

```
Phase 1 (Calculator) ─── Independent
Phase 2 (Terminal) ───── Depends on VirtualFileSystem integration
Phase 3 (Notes) ──────── Independent
Phase 4 (TextEdit) ────── Independent
Phase 5 (Finder) ──────── Depends on VirtualFileSystem (already integrated)
```

Phases 1, 3, 4 can execute in any order. Phase 2 and 5 share filesystem concerns but can proceed independently.

---

*Roadmap created: 2026-01-17*
*Phase 3 planned: 2026-01-17*
*Phase 4 planned: 2026-01-17*
*Phase 5 planned: 2026-01-17*
