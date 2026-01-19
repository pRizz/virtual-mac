# Technology Stack: v2.0 Persistence & Notifications

**Project:** VirtualMac
**Researched:** 2026-01-19
**Confidence:** HIGH (patterns already established in codebase)

## Executive Summary

This research covers the stack recommendations for two v2.0 features:
1. **State persistence** for Calculator, Terminal, TextEdit, and Dock
2. **Notification polish** to match macOS style

**Key finding:** The existing codebase already has proven patterns for both. No new dependencies required. Follow the established localStorage + serde_json patterns from Notes, VirtualFileSystem, and WindowManager.

---

## Recommended Stack

### State Persistence

| Technology | Version | Purpose | Rationale |
|------------|---------|---------|-----------|
| `web-sys::Storage` | 0.3 (existing) | localStorage API | Already used in Notes, FileSystem, WindowManager |
| `serde` | 1.0 (existing) | Serialization | Derive macros for clean state structs |
| `serde_json` | 1.0 (existing) | JSON format | Human-readable, debuggable in browser DevTools |
| Leptos `Effect` | 0.7 (existing) | Auto-save on change | Reactive persistence pattern from WindowManager |

**No new dependencies required.**

### Notification System

| Technology | Version | Purpose | Rationale |
|------------|---------|---------|-----------|
| CSS animations | Native | Entry/exit animations | Existing `@keyframes notification-slide-in` |
| CSS backdrop-filter | Native | Blur effect | Already implemented in notification styles |
| Leptos signals | 0.7 (existing) | Notification state management | NotificationState already uses RwSignal |

**No new dependencies required.**

---

## Persistence Implementation Pattern

The codebase has three established localStorage patterns. Use the most appropriate for each app.

### Pattern A: Component-Level Save/Load (Notes Pattern)

Used when state belongs to a single component and needs auto-save.

```rust
// Storage key
const STORAGE_KEY: &str = "virtualmac_calculator";

// Save function
fn save_to_storage(state: &CalculatorState) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(state) {
                    let _ = storage.set_item(STORAGE_KEY, &json);
                }
            }
        }
    }
}

// Load function
fn load_from_storage() -> CalculatorState {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(state) = serde_json::from_str(&json) {
                        return state;
                    }
                }
            }
        }
    }
    CalculatorState::default()
}

// In component: auto-save via Effect
Effect::new(move |_| {
    let current_state = state.get();
    save_to_storage(&current_state);
});
```

**Best for:** Calculator, Terminal (simple state, single component owner)

### Pattern B: Struct with Methods (FileSystem Pattern)

Used when state needs methods and is shared via context.

```rust
#[derive(Clone)]
pub struct DockState {
    pub pinned_apps: RwSignal<Vec<String>>,
    pub initialized: RwSignal<bool>,
}

impl DockState {
    pub fn new() -> Self { ... }
    pub fn save_to_storage(&self) { ... }
    pub fn load_from_storage(&self) -> bool { ... }
    pub fn pin_app(&self, name: &str) { ... }
    pub fn unpin_app(&self, name: &str) { ... }
}
```

**Best for:** Dock (shared state, multiple operations)

### Pattern C: Schema-Versioned Persistence (WindowManager Pattern)

Used when state schema may evolve and needs migration handling.

```rust
#[derive(Serialize, Deserialize)]
struct PersistedState {
    schema_version: u32,
    // ... fields
}

const CURRENT_SCHEMA_VERSION: u32 = 1;

fn load_state() -> Option<(State, bool)> {
    // ... load and parse
    let schema_mismatch = persisted.schema_version != CURRENT_SCHEMA_VERSION;
    Some((state, schema_mismatch))
}
```

**Best for:** Complex state that may need future migration (Terminal history format changes)

---

## App-Specific Recommendations

### Calculator

**State to persist:**
- `display: String` - Current display value
- `stored_value: f64` - Stored operand
- `current_op: Operation` - Pending operation
- `clear_on_next: bool` - Clear flag

**Storage key:** `virtualmac_calculator`

**Pattern:** A (Component-Level)

**Recommendation:** Create `CalculatorState` struct with Serialize/Deserialize. Load on component mount, save via Effect on state change.

**State NOT to persist:**
- `active_operator: Option<Operation>` - UI-only visual state

### Terminal

**State to persist:**
- `command_history: Vec<String>` - Command history for up/down arrows
- `cwd: String` - Current working directory
- Optionally: `history: Vec<String>` - Session output (can get large)

**Storage key:** `virtualmac_terminal`

**Pattern:** A or C (depending on whether to version the history format)

**Recommendation:** Persist command history and cwd. Consider NOT persisting session output (it can grow unbounded and isn't typical Terminal behavior).

**Complexity note:** Terminal instances are window-specific. If multiple Terminal windows are supported, consider per-window state keyed by window ID, or shared command history across all instances.

### TextEdit

**State to persist:**
- Document content (HTML from contenteditable)

**Storage key:** `virtualmac_textedit_{window_id}` or save to VirtualFileSystem

**Recommendation:** Two approaches:
1. **Auto-save to VFS:** Save document to `/Documents/Untitled.txt` (or similar) using existing VirtualFileSystem. This matches real TextEdit behavior.
2. **localStorage draft:** Save unsaved content to localStorage, restore on reopen.

**Preferred:** Option 1 (VFS integration) - aligns with macOS TextEdit, leverages existing persistence.

### Dock

**State to persist:**
- `pinned_apps: Vec<String>` - List of pinned app names
- Running indicators: Derive from WindowManager (don't persist separately)

**Storage key:** `virtualmac_dock`

**Pattern:** B (Struct with Methods) or simple Pattern A

**Recommendation:**
1. Make dock items a signal instead of static vec
2. Derive `is_running` from WindowManager's open windows
3. Persist only the pinned apps list
4. Load on startup, save when user pins/unpins

**Integration:** Dock needs read access to WindowManager's open windows. Use SystemState or a new context signal.

---

## Notification System Polish

### Current State

The notification system exists (`src/notification.rs`) with:
- Basic structure (NotificationState, Notification, NotificationContainer)
- Dark/light theme support
- Auto-dismiss after 5 seconds
- Slide-in animation

### Polish Recommendations

**Visual improvements (CSS only):**

| Enhancement | How | Effort |
|-------------|-----|--------|
| Rounded corners matching macOS | Adjust `border-radius` to ~16px | Trivial |
| App icon support | Add `icon` field rendering (already in struct) | Low |
| Stacking animation | CSS transitions for `top` position | Low |
| Slide-out on dismiss | Add `@keyframes notification-slide-out` | Low |
| Grouping by app | Group notifications from same source | Medium |

**Behavioral improvements (Rust):**

| Enhancement | How | Effort |
|-------------|-----|--------|
| Click to expand | Add expanded state, more detail | Medium |
| Notification sound | Web Audio API (optional, user preference) | Medium |
| Notification Center | Persist notifications, show history | High |

**Recommended scope for v2.0:**
1. Slide-out animation on dismiss (CSS)
2. Proper app icon rendering (use `icon` field)
3. Slightly larger/more rounded appearance

**NOT recommended for v2.0:**
- Notification Center (high complexity, low user value for simulation)
- Sound effects (browser autoplay policies make this unreliable)

---

## Alternatives Considered

### For Persistence

| Approach | Considered | Why Not |
|----------|------------|---------|
| IndexedDB | web-sys features already enabled | Overkill for simple state, localStorage sufficient |
| SessionStorage | Clears on tab close | Not true persistence |
| External state library (Yewdux-style) | None for Leptos 0.7 CSR | Leptos signals + localStorage pattern is clean |

### For Notifications

| Approach | Considered | Why Not |
|----------|------------|---------|
| Web Notifications API | Browser-native notifications | Different UX from macOS, requires permission |
| Toast library | External dependency | Custom CSS matches macOS better |

---

## Storage Keys Summary

| Key | Owner | Contents |
|-----|-------|----------|
| `virtualmac_notes` | Notes | Folders, notes, selection state |
| `virtualmac_fs` | VirtualFileSystem | File/directory entries |
| `virtualmac_desktop` | WindowManager | Window positions, sizes, open apps |
| `virtualmac_theme` | Theme | Light/dark mode preference |
| `virtualmac_calculator` | Calculator (NEW) | Display, operation state |
| `virtualmac_terminal` | Terminal (NEW) | Command history, cwd |
| `virtualmac_dock` | Dock (NEW) | Pinned apps list |

---

## Implementation Order

Based on dependencies and complexity:

1. **Calculator persistence** - Simplest, no dependencies, good pattern validation
2. **Terminal persistence** - Slightly more state, but still isolated
3. **Dock state** - Requires WindowManager integration for running indicators
4. **Notification polish** - CSS-only changes, independent of other work

---

## Sources

All recommendations are based on existing codebase patterns:
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notes.rs` (lines 78-109) - Component save/load pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/file_system.rs` (lines 383-418) - Struct method pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/window_manager.rs` (lines 196-269) - Schema versioning pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notification.rs` - Existing notification implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/styles.css` (lines 2994-3131) - Notification CSS

**Confidence: HIGH** - All patterns are proven in the codebase, no new dependencies needed.
