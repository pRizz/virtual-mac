# Phase 3: Notes App Creation - Research

**Researched:** 2026-01-17
**Domain:** Rich text editor, macOS Notes UI replication, localStorage persistence
**Confidence:** MEDIUM (based on existing codebase patterns and web standards, some macOS-specific details inferred)

## Summary

This phase requires creating a Notes app that replicates the macOS Notes appearance and functionality. The research covers three main areas: visual design (three-column layout matching macOS Notes), rich text editing (using contenteditable with execCommand), and data persistence (localStorage with JSON serialization).

The existing codebase provides strong patterns to follow. The `textedit.rs` already demonstrates contenteditable usage with execCommand for bold/italic formatting. The `finder.rs` shows a two-column layout (sidebar + content) that can be extended to three columns. The `window_manager.rs` shows how to register new app types.

**Primary recommendation:** Build the Notes app using the existing patterns from `textedit.rs` for rich text editing and `finder.rs` for layout structure, extending them to meet the full requirements.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | 0.7 | Reactive UI framework | Already in use, CSR mode |
| web-sys | 0.3 | DOM APIs | Already in use for contenteditable |
| wasm-bindgen | 0.2 | JS interop | Already in use for execCommand |
| serde/serde_json | 1.0 | JSON serialization | Already in use for persistence |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| js-sys | 0.3 | JavaScript Date API | Already available, for note timestamps |
| web_sys Storage | 0.3 | localStorage access | Already available via features |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| execCommand | Range/Selection API | execCommand deprecated but still works everywhere; Range API requires more code |
| localStorage | IndexedDB | IndexedDB more complex, localStorage sufficient for notes |
| Custom rich text | JS library (Quill, Tiptap) | Would require JS interop, adds complexity |

**Installation:**
No additional dependencies needed - all required crates are already in `Cargo.toml`.

## Architecture Patterns

### Recommended Project Structure
```
src/
  notes.rs           # Main Notes component
  notes/
    folder_sidebar.rs  # Folder management sidebar
    notes_list.rs      # Notes list in middle column
    note_editor.rs     # Rich text editor
    note_types.rs      # Data structures (Note, Folder, etc.)
    note_storage.rs    # localStorage persistence
```

**Alternative (simpler):** Single `notes.rs` file with all components inline (like `finder.rs`), splitting later if needed.

### Pattern 1: Three-Column Layout with CSS Flexbox
**What:** Use flexbox for the three-column layout with fixed-width sidebars
**When to use:** Primary layout structure
**Example:**
```css
/* Based on existing finder layout patterns */
.notes-app {
    display: flex;
    height: 100%;
    font-family: var(--font-family);
}

.notes-folder-sidebar {
    width: 200px;
    min-width: 150px;
    background: rgba(245, 245, 247, 0.95);
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    overflow-y: auto;
}

.notes-list {
    width: 280px;
    min-width: 200px;
    background: #fff;
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    overflow-y: auto;
}

.notes-editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
```

### Pattern 2: Rich Text with execCommand
**What:** Use contenteditable with document.execCommand for formatting
**When to use:** All rich text operations (bold, italic, lists, etc.)
**Example:**
```rust
// Source: existing textedit.rs pattern
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn execCommand(command: &str, show_ui: bool, value: &str) -> bool;
}

// Formatting functions
let toggle_bold = move |_| { execCommand("bold", false, ""); };
let toggle_italic = move |_| { execCommand("italic", false, ""); };
let toggle_underline = move |_| { execCommand("underline", false, ""); };
let toggle_strikethrough = move |_| { execCommand("strikeThrough", false, ""); };
let insert_unordered_list = move |_| { execCommand("insertUnorderedList", false, ""); };
let insert_ordered_list = move |_| { execCommand("insertOrderedList", false, ""); };
```

### Pattern 3: Data Structures for Notes
**What:** Serde-serializable structs for notes and folders
**When to use:** Data model and persistence
**Example:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,              // UUID or timestamp-based
    pub folder_id: String,
    pub title: String,           // First line or extracted
    pub content: String,         // HTML content from contenteditable
    pub created_at: f64,         // JS timestamp
    pub updated_at: f64,
    pub is_deleted: bool,        // Soft delete for "Recently Deleted"
    pub deleted_at: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub is_system: bool,  // true for "All Notes", "Recently Deleted"
    pub created_at: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NotesState {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
    pub selected_folder_id: Option<String>,
    pub selected_note_id: Option<String>,
}
```

### Pattern 4: localStorage Persistence
**What:** Save/load notes state using web_sys Storage
**When to use:** Initial load and after any state change
**Example:**
```rust
// Source: web_sys Storage API
fn save_to_storage(state: &NotesState) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(state) {
                let _ = storage.set_item("virtualmac_notes", &json);
            }
        }
    }
}

fn load_from_storage() -> NotesState {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(json)) = storage.get_item("virtualmac_notes") {
                if let Ok(state) = serde_json::from_str(&json) {
                    return state;
                }
            }
        }
    }
    NotesState::default_with_system_folders()
}
```

### Pattern 5: Checklist Implementation
**What:** Interactive checkboxes within contenteditable
**When to use:** Checklist feature (REQ-003.7)
**Example:**
```rust
// Insert a checkbox at cursor position
let insert_checkbox = move |_| {
    // Create checkbox HTML to insert
    let checkbox_html = r#"<div class="note-checklist-item">
        <input type="checkbox" class="note-checkbox" />
        <span contenteditable="true"></span>
    </div>"#;
    execCommand("insertHTML", false, checkbox_html);
};
```

### Anti-Patterns to Avoid
- **Direct innerHTML manipulation without sanitization:** Security risk; always sanitize pasted content
- **Storing note content as plain text:** Loses formatting; store as HTML
- **Recreating contenteditable on each render:** Loses cursor position and state
- **Polling localStorage:** Use Effect to save on state changes instead

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rich text formatting | Custom selection/range manipulation | document.execCommand | Despite deprecation, it handles undo/redo, cursor position |
| UUID generation | Custom ID scheme | js_sys::Date timestamp or Math.random | Sufficient uniqueness for local notes |
| JSON serialization | Custom format | serde_json | Already in dependencies, handles complex structures |
| Date formatting | Custom formatter | js_sys::Date methods | Already available, handles localization |

**Key insight:** The browser's built-in contenteditable with execCommand handles many edge cases (undo/redo, cursor management, selection) that would be extremely complex to reimplement.

## Common Pitfalls

### Pitfall 1: Losing Editor Content on Re-render
**What goes wrong:** Leptos re-renders the contenteditable div, wiping out user content
**Why it happens:** Contenteditable divs are not controlled inputs; their content lives in the DOM
**How to avoid:** Use NodeRef and only update DOM when explicitly needed; don't bind innerHTML reactively
**Warning signs:** Content disappears when other state changes

### Pitfall 2: Paste Handling
**What goes wrong:** Users paste rich content from Word/web, breaking layout
**Why it happens:** Contenteditable accepts any HTML
**How to avoid:** Add paste event listener, sanitize HTML, strip unwanted tags/styles
**Warning signs:** Notes contain unexpected fonts, colors, or broken formatting

### Pitfall 3: execCommand Inconsistency
**What goes wrong:** Different results in different browsers
**Why it happens:** execCommand is deprecated and inconsistently implemented
**How to avoid:** Test in target browsers; use queryCommandSupported for feature detection
**Warning signs:** Formatting works in Chrome but not Safari

### Pitfall 4: localStorage Size Limits
**What goes wrong:** Notes stop saving silently
**Why it happens:** localStorage typically limited to 5-10MB
**How to avoid:** Implement error handling on setItem; consider note count warnings
**Warning signs:** New notes don't persist after refresh

### Pitfall 5: Search Performance
**What goes wrong:** UI freezes during search
**Why it happens:** Searching all note content synchronously
**How to avoid:** Use simple string matching (sufficient for local notes); debounce search input
**Warning signs:** Typing in search field feels laggy

## Code Examples

### Complete Notes Component Structure
```rust
// Source: Based on existing finder.rs and textedit.rs patterns
use leptos::prelude::*;
use serde::{Serialize, Deserialize};

#[component]
pub fn Notes() -> impl IntoView {
    // State management
    let (notes_state, set_notes_state) = signal(load_from_storage());
    let (search_query, set_search_query) = signal(String::new());

    // Auto-save on state changes
    Effect::new(move |_| {
        let state = notes_state.get();
        save_to_storage(&state);
    });

    // Computed: filtered folders and notes
    let visible_notes = Memo::new(move |_| {
        let state = notes_state.get();
        let query = search_query.get().to_lowercase();
        // Filter logic here
    });

    view! {
        <div class="notes-app">
            <FolderSidebar
                state=notes_state
                set_state=set_notes_state
            />
            <NotesList
                notes=visible_notes
                state=notes_state
                set_state=set_notes_state
                search_query=search_query
                set_search_query=set_search_query
            />
            <NoteEditor
                state=notes_state
                set_state=set_notes_state
            />
        </div>
    }
}
```

### Folder Sidebar Component
```rust
#[component]
fn FolderSidebar(
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
) -> impl IntoView {
    let folders = move || state.get().folders.clone();
    let selected_id = move || state.get().selected_folder_id.clone();

    view! {
        <div class="notes-folder-sidebar">
            <div class="folder-header">"Folders"</div>
            <For
                each=folders
                key=|folder| folder.id.clone()
                children=move |folder| {
                    let folder_id = folder.id.clone();
                    let is_selected = move || selected_id() == Some(folder_id.clone());
                    view! {
                        <div
                            class=move || if is_selected() { "folder-item selected" } else { "folder-item" }
                            on:click=move |_| {
                                set_state.update(|s| {
                                    s.selected_folder_id = Some(folder_id.clone());
                                });
                            }
                        >
                            <span class="folder-icon">{if folder.is_system { "folder-icon" } else { "folder-icon" }}</span>
                            <span class="folder-name">{folder.name.clone()}</span>
                        </div>
                    }
                }
            />
        </div>
    }
}
```

### Editor with Formatting Toolbar
```rust
#[component]
fn NoteEditor(
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
) -> impl IntoView {
    let editor_ref: NodeRef<html::Div> = NodeRef::new();

    // Format buttons
    let format_bold = move |_| { execCommand("bold", false, ""); };
    let format_italic = move |_| { execCommand("italic", false, ""); };
    let format_underline = move |_| { execCommand("underline", false, ""); };
    let format_strike = move |_| { execCommand("strikeThrough", false, ""); };
    let format_bullet = move |_| { execCommand("insertUnorderedList", false, ""); };
    let format_number = move |_| { execCommand("insertOrderedList", false, ""); };

    // Save content on blur
    let on_blur = move |_| {
        if let Some(el) = editor_ref.get() {
            let content = el.inner_html();
            set_state.update(|s| {
                if let Some(note_id) = &s.selected_note_id {
                    if let Some(note) = s.notes.iter_mut().find(|n| &n.id == note_id) {
                        note.content = content;
                        note.updated_at = js_sys::Date::now();
                    }
                }
            });
        }
    };

    view! {
        <div class="notes-editor">
            <div class="editor-toolbar">
                <button class="toolbar-btn" on:click=format_bold title="Bold"><strong>"B"</strong></button>
                <button class="toolbar-btn" on:click=format_italic title="Italic"><em>"I"</em></button>
                <button class="toolbar-btn" on:click=format_underline title="Underline"><u>"U"</u></button>
                <button class="toolbar-btn" on:click=format_strike title="Strikethrough"><s>"S"</s></button>
                <div class="toolbar-separator"></div>
                <button class="toolbar-btn" on:click=format_bullet title="Bullet List">"*"</button>
                <button class="toolbar-btn" on:click=format_number title="Numbered List">"1."</button>
            </div>
            <div
                class="editor-content"
                contenteditable="true"
                node_ref=editor_ref
                on:blur=on_blur
            >
                // Content loaded via Effect when selected note changes
            </div>
        </div>
    }
}
```

## macOS Notes Visual Specifications

### Color Scheme
| Element | Light Mode | Dark Mode |
|---------|------------|-----------|
| App Background | #f5f5f7 | #1c1c1e |
| Sidebar Background | rgba(245, 245, 247, 0.95) | rgba(44, 44, 46, 0.95) |
| List Background | #ffffff | #2c2c2e |
| Editor Background | #ffffff | #1c1c1e |
| Text Primary | #000000 | #ffffff |
| Text Secondary | #666666 | #8e8e93 |
| Selection/Accent | #007aff | #0a84ff |
| Separator | rgba(0, 0, 0, 0.1) | rgba(255, 255, 255, 0.1) |
| Note Yellow (icon) | #ffd52e | #ffd52e |

### Typography
| Element | Font | Size | Weight |
|---------|------|------|--------|
| Note Title | SF Pro | 17px | 600 (semibold) |
| Note Preview | SF Pro | 13px | 400 (regular) |
| Note Date | SF Pro | 11px | 400 (regular) |
| Editor Body | SF Pro | 15-17px | 400 (regular) |
| Folder Name | SF Pro | 13px | 400 (regular) |
| Section Header | SF Pro | 11px | 600 (semibold) |

### Layout Dimensions
| Element | Width | Notes |
|---------|-------|-------|
| Folder Sidebar | 180-220px | Collapsible |
| Notes List | 250-300px | Resizable |
| Editor | Flexible (fill) | Min ~300px |
| Toolbar Height | ~40px | - |
| Note List Item | Full width | ~70px height |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| execCommand only | Input Events Level 2 (emerging) | 2023+ | execCommand still works, alternatives incomplete |
| Full page re-render | Leptos fine-grained reactivity | Current | Better performance for editor state |
| localStorage only | IndexedDB for large data | Current | localStorage sufficient for notes |

**Deprecated/outdated:**
- `document.execCommand`: Officially deprecated but no complete alternative; still universally supported

## Open Questions

Things that couldn't be fully resolved:

1. **Exact macOS Notes pixel specifications**
   - What we know: General layout structure, approximate colors
   - What's unclear: Exact padding, margins, border-radius values
   - Recommendation: Use Apple's Human Interface Guidelines colors; iterate visually

2. **Checklist auto-sorting behavior**
   - What we know: macOS Notes can auto-sort completed items to bottom
   - What's unclear: Whether to implement this initially
   - Recommendation: Start with manual ordering; add auto-sort as enhancement

3. **Rich text paste sanitization**
   - What we know: Need to handle pasted content
   - What's unclear: Exact tags/attributes to preserve vs. strip
   - Recommendation: Preserve basic formatting (b, i, u, ul, ol, li); strip everything else

## Sources

### Primary (HIGH confidence)
- Existing codebase: `/Users/peterryszkiewicz/Repos/virtual-mac/src/textedit.rs` - contenteditable + execCommand pattern
- Existing codebase: `/Users/peterryszkiewicz/Repos/virtual-mac/src/finder.rs` - multi-column layout pattern
- Existing codebase: `/Users/peterryszkiewicz/Repos/virtual-mac/src/calculator.rs` - component state pattern
- [Leptos Book - web_sys](https://book.leptos.dev/web_sys.html) - NodeRef and DOM access

### Secondary (MEDIUM confidence)
- [Apple Support - Create a checklist with Notes](https://support.apple.com/en-us/102296) - Notes checklist behavior
- [MDN - execCommand](https://developer.mozilla.org/en-US/docs/Web/API/Document/execCommand) - Command reference
- [MDN - contenteditable](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable) - Attribute behavior
- [SchemeColor - Apple Notes colors](https://www.schemecolor.com/apple-notes-color-scheme.php) - Color values (#ffd52e, #f9f9f9, #e5e5e5)

### Tertiary (LOW confidence)
- Various web searches on macOS Notes UI - specific dimensions vary by source
- Rich text editor best practices articles - general patterns, not Leptos-specific

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing project dependencies
- Architecture: MEDIUM - Patterns exist in codebase, Notes-specific adaptation needed
- Rich text implementation: MEDIUM - execCommand works but deprecated; existing textedit.rs proves pattern
- Visual specifications: LOW - Approximate values, need visual iteration
- Pitfalls: MEDIUM - Based on general web development knowledge

**Research date:** 2026-01-17
**Valid until:** 2026-02-17 (30 days - stable domain, existing patterns)
