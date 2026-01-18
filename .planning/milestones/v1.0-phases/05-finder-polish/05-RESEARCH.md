# Phase 5: Finder Polish - Research

**Researched:** 2026-01-17
**Domain:** macOS Finder UI Replication (Leptos/WASM)
**Confidence:** HIGH

## Summary

This research examines how to transform the existing Finder implementation into a near-identical replica of macOS Finder. The current finder.rs provides a solid foundation with a working sidebar, toolbar, navigation history, icon view, and basic status bar. The remaining work focuses on:

1. **View Modes** - Adding list view and column view alongside the existing icon view
2. **Sidebar Polish** - Matching macOS section headers (Favorites, iCloud, Locations, Tags)
3. **Path Bar** - Adding clickable breadcrumb navigation at the bottom
4. **Search Functionality** - Implementing real-time filtering of the current view
5. **Context Menu Actions** - Wiring up existing menu items to VirtualFileSystem operations
6. **Visual Polish** - Icon styling, spacing, and color refinements

**Primary recommendation:** Implement view modes as the core structural change first, then layer on search, path bar, and context menu actions. The existing CSS variables and dark mode support provide excellent scaffolding.

## Standard Stack

The project already uses the correct stack. No new dependencies are needed.

### Core (Already in Use)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | 0.7 | Reactive UI framework | Already in use, signals work well |
| VirtualFileSystem | n/a | File operations | Already integrated, shared with Terminal |

### Supporting (Already in Use)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| js-sys | n/a | JavaScript Date API | Already used for timestamps |
| web-sys | n/a | DOM events, localStorage | Already used for persistence |

### No New Dependencies Needed
The existing stack is complete for this phase. The context menu system (context_menu.rs) already exists and defines `ContextMenuType::FinderItem`. The VirtualFileSystem already supports all needed operations (list_dir, create_dir, delete, rename, write_file).

## Current Implementation Analysis

### What Exists (finder.rs - 297 lines)
```rust
// Data structures
- FileItem: name, path, is_folder, icon
- SidebarItem: name, icon, path (static)

// State management
- selected_sidebar: signal for active sidebar item
- current_path: signal for current directory
- selected_items: Vec<String> for multi-select
- path_history / history_index: navigation history

// Components
- Toolbar: back/forward buttons, view mode buttons (icons only), search input
- Sidebar: Favorites section, Locations section
- Content: Grid view only (finder-grid CSS class)
- Status bar: Item count only
```

### What's Missing (Requirements)
| Requirement | Status | Gap |
|-------------|--------|-----|
| REQ-005.1: Sidebar sections | Partial | Missing iCloud, Tags sections |
| REQ-005.2: Toolbar | Done | View buttons not functional |
| REQ-005.3: Icon view | Done | Spacing could be refined |
| REQ-005.4: List view | Missing | New component needed |
| REQ-005.5: Column view | Missing | Miller columns needed |
| REQ-005.6: Path bar | Missing | New component needed |
| REQ-005.7: Status bar | Partial | Missing "available space" display |
| REQ-005.8: Search | Missing | Input exists, no filter logic |
| REQ-005.9: Context menu | Partial | Menu defined, actions not wired |
| REQ-005.10: Double-click | Done | Already works for folders |
| REQ-005.11: Icons | Partial | Uses emojis, could be refined |

### CSS Structure (styles.css lines 716-978)
- CSS variables for light/dark mode already defined
- Finder-specific variables: `--finder-bg`, `--finder-toolbar-*`, `--finder-sidebar-*`
- Grid layout for icon view: `grid-template-columns: repeat(auto-fill, minmax(90px, 1fr))`
- Status bar styling exists but is minimal

## Architecture Patterns

### Recommended Project Structure
The existing structure works. Changes stay within finder.rs and styles.css.

```
src/
├── finder.rs           # All view modes, search, actions
├── file_system.rs      # Already complete, no changes needed
├── context_menu.rs     # Already has FinderItem type
└── styles.css          # Add list/column view styles

No new files needed.
```

### Pattern 1: View Mode State Machine
**What:** Use an enum signal to track active view mode
**When to use:** Switching between icon/list/column views

```rust
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ViewMode {
    #[default]
    Icon,
    List,
    Column,
    Gallery,  // Optional - lowest priority
}

let (view_mode, set_view_mode) = signal(ViewMode::Icon);
```

### Pattern 2: Conditional View Rendering
**What:** Render different view components based on view_mode signal
**When to use:** In the content area

```rust
// Inside finder-content div
{move || match view_mode.get() {
    ViewMode::Icon => view! { <IconView files=files.get() ... /> }.into_any(),
    ViewMode::List => view! { <ListView files=files.get() ... /> }.into_any(),
    ViewMode::Column => view! { <ColumnView current_path=current_path ... /> }.into_any(),
    ViewMode::Gallery => view! { <GalleryView files=files.get() ... /> }.into_any(),
}}
```

### Pattern 3: Column View as Miller Columns
**What:** Recursive column rendering where selecting an item shows its children in next column
**When to use:** Column view implementation

```rust
// Column view needs its own state for column history
let (column_paths, set_column_paths) = signal(vec![current_path.get()]);

// Each column shows contents of its path
// Clicking a folder adds its path to column_paths
// Clicking a file shows preview in rightmost column
```

### Pattern 4: Search as Derived Signal
**What:** Filter files based on search query using Memo
**When to use:** Real-time search filtering

```rust
let (search_query, set_search_query) = signal(String::new());

let filtered_files = Memo::new(move |_| {
    let query = search_query.get().to_lowercase();
    let all_files = files.get();

    if query.is_empty() {
        all_files
    } else {
        all_files.into_iter()
            .filter(|f| f.name.to_lowercase().contains(&query))
            .collect()
    }
});
```

### Pattern 5: Context Menu Action Dispatch
**What:** Wire context menu clicks to VirtualFileSystem operations
**When to use:** Making context menu functional

The existing context_menu.rs uses `on:click=close_menu` for all items. To add functionality:

```rust
// In Finder component, listen for context menu actions
// Use a callback pattern or extend context menu to dispatch actions

// Option A: Prop drilling callbacks to context menu
// Option B: Use Leptos context for action dispatch
// Option C: Global event system (overcomplicated)

// Recommendation: Option A for simplicity
```

### Anti-Patterns to Avoid
- **Separate view mode components in different files:** Keep all view modes in finder.rs for cohesion
- **Duplicating file listing logic:** Use the existing Memo-based files signal for all views
- **Custom scrollbar implementation:** Browser native scrollbars with CSS styling work fine
- **Complex animation libraries:** CSS transitions are sufficient for view transitions

## Don't Hand-Roll

Problems that have existing solutions in the codebase:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| File operations | Custom file logic | VirtualFileSystem | Already works with Terminal |
| Context menus | New menu system | context_menu.rs | Already has FinderItem type |
| Icon rendering | Complex icon system | Emoji + CSS | Simple, works for simulator |
| Dark mode | Theme state | CSS variables | Already implemented |
| Path parsing | Custom parser | get_parent_path, get_file_name | In file_system.rs |

**Key insight:** The codebase already has the infrastructure. This phase is about wiring things together and adding views, not building new systems.

## Common Pitfalls

### Pitfall 1: Column View Complexity Explosion
**What goes wrong:** Column view implementation becomes overcomplicated with excessive state
**Why it happens:** Trying to track selection state across multiple columns independently
**How to avoid:** Use a single `column_paths: Vec<String>` signal; derive everything else
**Warning signs:** More than 3 signals for column view state

### Pitfall 2: View Transition State Desync
**What goes wrong:** Switching views loses selection or path state
**Why it happens:** Each view manages its own state independently
**How to avoid:** Keep current_path and selected_items at Finder component level, pass down
**Warning signs:** Clicking icon view then list view shows different content

### Pitfall 3: Search Performance on Large Directories
**What goes wrong:** Typing in search causes lag
**Why it happens:** Re-filtering on every keystroke without debouncing
**How to avoid:** The VirtualFileSystem is small enough that this won't matter. If it did, debounce with 150ms delay
**Warning signs:** Visible lag when typing in search field

### Pitfall 4: Context Menu Actions Without Feedback
**What goes wrong:** User clicks "New Folder" and nothing visible happens
**Why it happens:** Folder created but view not refreshed
**How to avoid:** VirtualFileSystem already uses version signal; just ensure files Memo subscribes to it (it does)
**Warning signs:** Need to click away and back to see changes

### Pitfall 5: Path Bar Click Navigation Breaking History
**What goes wrong:** Clicking path bar breadcrumb doesn't update history correctly
**Why it happens:** Using set_current_path directly instead of navigate_to
**How to avoid:** Always use navigate_to closure for navigation
**Warning signs:** Back button doesn't work after path bar navigation

## Code Examples

Verified patterns from the existing codebase:

### Existing File Listing Pattern (finder.rs)
```rust
// This pattern works well - keep using it
let files = Memo::new(move |_| {
    let _ = fs.version.get(); // Subscribe to FS changes

    match selected_sidebar.get() {
        "Recents" => fs.get_recents(10).into_iter().map(|e| FileItem::from_entry(&e)).collect(),
        "AirDrop" | "Network" => Vec::new(),
        _ => fs.list_dir(&current_path.get()).into_iter().map(|e| FileItem::from_entry(&e)).collect(),
    }
});
```

### List View Column Header Pattern
```rust
// List view needs sortable columns
view! {
    <div class="finder-list-header">
        <div class="list-col name" on:click=move |_| set_sort_column("name")>"Name"</div>
        <div class="list-col date" on:click=move |_| set_sort_column("date")>"Date Modified"</div>
        <div class="list-col size" on:click=move |_| set_sort_column("size")>"Size"</div>
        <div class="list-col kind" on:click=move |_| set_sort_column("kind")>"Kind"</div>
    </div>
}
```

### Path Bar Component Pattern
```rust
// Path bar shows breadcrumb navigation
view! {
    <div class="finder-pathbar">
        {move || {
            let path = current_path.get();
            let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

            parts.into_iter().enumerate().map(|(i, part)| {
                let full_path = format!("/{}", parts[0..=i].join("/"));
                view! {
                    <button
                        class="pathbar-segment"
                        on:click=move |_| navigate_to(full_path.clone())
                    >
                        {if i == 0 && part == "Macintosh HD" { "Macintosh HD" } else { part }}
                    </button>
                    {if i < parts.len() - 1 { Some(view! { <span class="pathbar-separator">">"</span> }) } else { None }}
                }
            }).collect::<Vec<_>>()
        }}
    </div>
}
```

### Column View CSS Pattern
```css
/* Miller columns layout */
.finder-columns {
    display: flex;
    flex: 1;
    overflow-x: auto;
}

.finder-column {
    min-width: 200px;
    max-width: 300px;
    border-right: 1px solid var(--finder-sidebar-border);
    overflow-y: auto;
    flex-shrink: 0;
}

.finder-column-item {
    display: flex;
    align-items: center;
    padding: 4px 12px;
    cursor: pointer;
}

.finder-column-item.selected {
    background: var(--finder-selected);
}

.finder-column-item.has-children::after {
    content: ">";
    margin-left: auto;
    color: var(--finder-text-muted);
}
```

### List View CSS Pattern
```css
/* List view table-like layout */
.finder-list {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
}

.finder-list-header {
    display: flex;
    padding: 8px 16px;
    background: var(--finder-toolbar-end);
    border-bottom: 1px solid var(--finder-toolbar-border);
    font-size: 11px;
    font-weight: 600;
    color: var(--finder-text-secondary);
}

.finder-list-body {
    flex: 1;
    overflow-y: auto;
}

.finder-list-row {
    display: flex;
    padding: 4px 16px;
    align-items: center;
    border-bottom: 1px solid var(--finder-hover);
}

.finder-list-row:hover {
    background: var(--finder-hover);
}

.finder-list-row.selected {
    background: var(--finder-selected);
}

.list-col {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.list-col.name { flex: 2; }
.list-col.date { flex: 1; min-width: 120px; }
.list-col.size { flex: 0 0 80px; text-align: right; }
.list-col.kind { flex: 0 0 100px; }
```

## macOS Finder Visual Reference

### Sidebar Sections (macOS 2025)
1. **Favorites** - AirDrop, Recents, Applications, Desktop, Documents, Downloads, (custom folders)
2. **iCloud** - iCloud Drive, Shared (if signed in)
3. **Locations** - Macintosh HD, Network, connected drives
4. **Tags** - Color tags at bottom

For this simulator, recommend implementing:
- Favorites (keep current items)
- Locations (Macintosh HD only)
- Tags (optional, can use placeholder)

### Path Bar Appearance
- Located between content area and status bar
- Shows folder icons + names as clickable segments
- Separator is ">" chevron
- Double-click a segment opens it in new window (not applicable here)

### Status Bar Appearance
- Shows: "{N} items, {X} GB available"
- For simulator: "{N} items" is sufficient (no real disk space)
- Optionally add: "x selected" when items selected

### View Mode Button Icons
Current implementation uses Unicode characters. Consider:
- Icon view: Grid icon (existing: "square grid")
- List view: Horizontal lines (existing: "hamburger menu")
- Column view: Vertical lines (existing: "vertical bars")
- Gallery view: Rectangle (existing: "rectangle")

The current Unicode approach is acceptable for a simulator.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Table-based list views | CSS flexbox/grid | 2015+ | More flexible layouts |
| jQuery file browsers | Component frameworks | 2018+ | Better state management |
| Custom scrollbars | Native + CSS styling | 2020+ | Simpler, more accessible |

**Current best practice for web file browsers:**
- Use flexbox for Miller columns
- Use CSS Grid for icon view
- Use flexbox rows for list view
- Native browser scrollbars with `::-webkit-scrollbar` styling

## Open Questions

### 1. Gallery View Priority
- **What we know:** Requirements mention icon, list, column views explicitly
- **What's unclear:** Gallery view mentioned in code but not requirements
- **Recommendation:** Implement as lowest priority or placeholder

### 2. iCloud Section Necessity
- **What we know:** Real macOS has iCloud section
- **What's unclear:** Whether simulator needs it (no backend)
- **Recommendation:** Add section header with "iCloud Drive" item that shows empty state or same as root

### 3. Context Menu Action Scope
- **What we know:** Menu items defined in context_menu.rs
- **What's unclear:** Which actions should be functional vs visual-only
- **Recommendation:** Implement: New Folder, Rename, Move to Trash, Get Info (modal). Skip: Compress, Make Alias, Share

## Sources

### Primary (HIGH confidence)
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/finder.rs` - Current implementation analysis
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/file_system.rs` - VirtualFileSystem API
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/context_menu.rs` - Existing menu system
- `/Users/peterryszkiewicz/Repos/virtual-mac/styles.css` - Current CSS patterns

### Secondary (MEDIUM confidence)
- [Apple Support - Finder Sidebar](https://support.apple.com/guide/mac-help/customize-the-finder-sidebar-on-mac-mchl83c9e8b8/mac) - Official sidebar sections
- [Finder.js](https://markmatyas.dev/finderjs/) - Miller columns implementation reference
- [Miller columns - Wikipedia](https://en.wikipedia.org/wiki/Miller_columns) - Pattern background

### Tertiary (LOW confidence)
- [MacMost - Path Bar tutorial](https://macmost.com/reveal-and-use-the-finder-path-bar-and-status-bar.html) - Path bar behavior
- [MacRumors - macOS Tahoe Finder](https://forums.macrumors.com/threads/macos-tahoe-inconsistency-path-bar-statue-bar-what-do-you-think.2458799/) - Latest design notes

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Already using correct libraries, no changes needed
- Architecture: HIGH - Patterns extracted from existing codebase
- View modes: MEDIUM - Column view complexity requires careful implementation
- Pitfalls: HIGH - Based on common Leptos patterns and existing code analysis

**Research date:** 2026-01-17
**Valid until:** 60 days (stable stack, well-understood patterns)
