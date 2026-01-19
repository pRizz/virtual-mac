# Architecture Patterns

**Domain:** State persistence and notification system for Leptos/WASM macOS simulation
**Researched:** 2026-01-19
**Confidence:** HIGH (based on direct codebase analysis)

## Existing Architecture Summary

VirtualMac uses a well-established pattern for state management and persistence. Understanding this is critical for consistent extension.

### Current Component Hierarchy

```
App (lib.rs)
├── Context Providers
│   ├── SystemState (global system state)
│   ├── NotificationState (notification queue)
│   ├── WallpaperContext (wallpaper settings)
│   └── VirtualFileSystem (file system)
├── ThemeProvider
│   └── FileSystemProvider
│       ├── MenuBar
│       ├── Desktop
│       ├── WindowManager
│       │   └── Windows (Calculator, Terminal, TextEdit, Notes, Finder)
│       ├── Dock
│       ├── Spotlight
│       ├── AppSwitcher
│       ├── ContextMenu
│       ├── ModalOverlay
│       ├── LockScreen
│       ├── PowerOverlay
│       └── NotificationContainer
```

### Data Flow Patterns

**Pattern 1: Context-Based Global State**
```
SystemState (provided at root)
    ↓ expect_context::<SystemState>()
    ↓
Child Components (MenuBar, WindowManager, Dock, etc.)
```

**Pattern 2: localStorage Persistence (established)**
```
Component State (RwSignal)
    ↓ Effect watching signal
    ↓ save_to_storage()
    ↓
localStorage (key: virtualmac_*)
```

## Component Boundaries

| Component | Responsibility | Communicates With | Persistence |
|-----------|----------------|-------------------|-------------|
| `SystemState` | Active app, modals, power state, minimized windows | All components | None |
| `WindowManager` | Window lifecycle, positions, z-index | SystemState, apps | `virtualmac_desktop` |
| `VirtualFileSystem` | File/directory CRUD | Finder, Terminal | `virtualmac_fs` |
| `NotificationState` | Notification queue and display | Any component | None |
| `Notes` | Note storage, folders | None | `virtualmac_notes` |
| `Calculator` | Calculator state | None | None (target) |
| `Terminal` | Shell state, command history | FileSystem | None (target) |
| `TextEdit` | Document content | None | None (target) |
| `Dock` | App launching, minimized windows display | SystemState, WindowManager | None (target) |

## Established Persistence Pattern

Analysis of existing persistence implementations reveals a consistent pattern.

### Notes App Pattern (reference implementation)

```rust
// 1. Define serializable state structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotesState {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
    pub selected_folder_id: Option<String>,
    pub selected_note_id: Option<String>,
}

// 2. Load/save functions (platform-gated)
fn save_to_storage(state: &NotesState) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(state) {
                    let _ = storage.set_item("virtualmac_notes", &json);
                }
            }
        }
    }
}

fn load_from_storage() -> NotesState {
    #[cfg(target_arch = "wasm32")]
    {
        // Similar pattern with error handling
    }
    NotesState::default()
}

// 3. Initialize from storage, auto-save via Effect
#[component]
pub fn Notes() -> impl IntoView {
    let (state, set_state) = signal(load_from_storage());

    // Auto-save on state changes
    Effect::new(move |_| {
        let current_state = state.get();
        save_to_storage(&current_state);
    });
    // ...
}
```

### WindowManager Pattern (reference for schema versioning)

```rust
// Schema version for migration support
const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedDesktopState {
    schema_version: u32,
    windows: Vec<PersistedWindow>,
    next_window_id: usize,
    top_z_index: i32,
}

// Load with version check
fn load_desktop_state() -> Option<(Vec<WindowState>, usize, i32, bool)> {
    // Returns schema_mismatch flag for notification
    let schema_mismatch = state.schema_version != CURRENT_SCHEMA_VERSION;
    // ...
}
```

### FileSystem Pattern (reference for complex state)

```rust
pub struct VirtualFileSystem {
    pub entries: RwSignal<HashMap<String, FileEntry>>,
    pub version: RwSignal<u32>,  // For re-render triggering
    pub initialized: RwSignal<bool>,
}

impl VirtualFileSystem {
    pub fn save_to_storage(&self) {
        // Called after each mutation
    }

    pub fn load_from_storage(&self) -> bool {
        // Returns success flag
    }
}
```

## Recommended Architecture for New Features

### App State Persistence

**Approach: Follow Notes pattern with per-app storage keys**

Each app that needs persistence should:
1. Define a serializable state struct
2. Use a unique localStorage key (`virtualmac_[appname]`)
3. Load on component mount
4. Save via Effect watching state changes

#### Calculator Persistence

```rust
// Simple state - just memory value
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct CalculatorState {
    memory: Option<f64>,  // M+, M-, MR memory
}
const CALC_STORAGE_KEY: &str = "virtualmac_calculator";
```

Why: Calculator traditionally has "memory" that persists across sessions. The current calculation state (display, operation) should NOT persist as that would be confusing UX.

#### Terminal Persistence

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TerminalState {
    command_history: Vec<String>,
    cwd: String,
}
const TERMINAL_STORAGE_KEY: &str = "virtualmac_terminal";
```

Why: Command history and current working directory are valuable to persist. The output history should NOT persist (terminals don't typically restore output).

#### TextEdit Persistence

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TextEditState {
    content: String,  // HTML content
    font_size: u32,
    font_family: String,
}
const TEXTEDIT_STORAGE_KEY: &str = "virtualmac_textedit";
```

Why: TextEdit is a scratch document. Persisting content prevents data loss. Toolbar settings are user preference.

### Dock State Persistence

**Current dock is hardcoded. Options for persistence:**

1. **Fixed dock with running indicators** (recommended for v2.0)
   - Dock items remain fixed
   - Running indicators derive from WindowManager state (already works)
   - No additional persistence needed

2. **Customizable dock** (future consideration)
   - User can add/remove/reorder items
   - Would require `virtualmac_dock` storage
   - Higher complexity, defer to later milestone

Recommendation: Keep dock fixed for v2.0. The "running indicators" already work via SystemState. Adding dock customization is a separate feature.

### Notification System Polish

**Current architecture:**

```rust
pub struct NotificationState {
    notifications: RwSignal<Vec<Notification>>,
    next_id: RwSignal<usize>,
}

impl NotificationState {
    pub fn show(&self, title: impl Into<String>, message: impl Into<String>) {
        // Creates notification, auto-dismisses after 5s
    }
    pub fn dismiss(&self, id: usize) { /* manual dismiss */ }
}
```

**Polish opportunities (no architecture change needed):**

1. **Add icon support** - `Notification` already has `icon: Option<String>` field, just unused
2. **Add action support** - Could add `on_click: Option<Callback>` for clickable notifications
3. **Improve positioning** - CSS-only change
4. **Add sound** - Would require Web Audio API integration

Recommendation: Notification architecture is sound. Polish is primarily CSS styling and using existing fields.

### About VirtualMac Menu Item

**Current architecture supports this:**

```rust
// system_state.rs already has:
pub enum ModalType {
    AboutThisMac,  // Already exists!
    // ...
}

// modals.rs already has:
fn AboutThisMacModal() -> impl IntoView { /* ... */ }
```

The "About This Mac" modal already exists. The task is likely:
1. Add "About VirtualMac" as a separate modal type, OR
2. Rename/update the existing AboutThisMac modal content

Recommendation: The modal system is already in place. Just need to add a menu item trigger in MenuBar.

## Data Flow for Persistence

```
┌─────────────────────────────────────────────────────────────────┐
│                        App Initialization                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│   Component mounts → load_from_storage() → initialize signal    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Normal Operation                            │
│  User interaction → set_state.update() → signal changes          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│   Effect watching signal → state.get() → save_to_storage()      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│   localStorage.setItem("virtualmac_[key]", JSON.stringify())    │
└─────────────────────────────────────────────────────────────────┘
```

## Patterns to Follow

### Pattern 1: Consistent Storage Key Naming
```rust
const STORAGE_KEY: &str = "virtualmac_[component]";
```
All keys should use `virtualmac_` prefix to namespace the app's data.

### Pattern 2: Platform-Gated Storage Access
```rust
fn save_to_storage(state: &T) {
    #[cfg(target_arch = "wasm32")]
    {
        // web_sys storage access
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = state;  // Suppress unused warning
    }
}
```
This allows the code to compile for non-WASM targets (testing).

### Pattern 3: Graceful Degradation on Load
```rust
fn load_from_storage() -> AppState {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(KEY) {
                    if let Ok(state) = serde_json::from_str(&json) {
                        return state;
                    }
                }
            }
        }
    }
    AppState::default()  // Fallback to defaults
}
```
Never panic on storage failures. Always have a sensible default.

### Pattern 4: Effect-Based Auto-Save
```rust
Effect::new(move |_| {
    let current_state = state.get();
    save_to_storage(&current_state);
});
```
This automatically saves whenever the watched signal changes. No manual save calls needed.

### Pattern 5: Schema Versioning (for complex state)
```rust
const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct PersistedState {
    schema_version: u32,
    // ... fields
}
```
Allows for graceful migration when state structure changes.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Persisting Ephemeral State
**Don't persist:**
- Calculator display/current operation (confusing UX)
- Terminal output history (not standard behavior)
- Transient UI state (hover states, animation state)

### Anti-Pattern 2: Blocking on Storage
```rust
// BAD: This could block
let state = load_from_storage().await;

// GOOD: Synchronous with fallback
let state = load_from_storage();  // Returns default on any failure
```

### Anti-Pattern 3: Multiple Storage Calls Per Change
```rust
// BAD: Saves multiple times
set_field_a.set(x);  // triggers Effect
set_field_b.set(y);  // triggers Effect again

// GOOD: Batch updates
set_state.update(|s| {
    s.field_a = x;
    s.field_b = y;
});  // Single Effect trigger
```

### Anti-Pattern 4: Context Provider Proliferation
```rust
// BAD: Each app has its own context
provide_context(CalculatorContext::new());
provide_context(TerminalContext::new());
provide_context(TextEditContext::new());

// GOOD: Apps manage their own state internally
// Only provide context for truly global state (SystemState, NotificationState)
```

App-specific state should stay local to the component, not be elevated to context.

## Suggested Build Order

Based on dependencies and complexity:

### Phase 1: About VirtualMac Menu Item
**Dependencies:** None (infrastructure exists)
**Complexity:** Low
**Approach:**
1. Add menu item to MenuBar
2. Add new ModalType variant (or reuse AboutThisMac)
3. Create/update modal content

### Phase 2: Calculator State Persistence
**Dependencies:** None
**Complexity:** Low
**Approach:**
1. Add memory signal to Calculator
2. Add M+, M-, MR, MC buttons
3. Add persistence with simple struct

### Phase 3: Terminal State Persistence
**Dependencies:** None
**Complexity:** Low
**Approach:**
1. Extract command_history and cwd to serializable struct
2. Add load/save functions
3. Add Effect for auto-save

### Phase 4: TextEdit State Persistence
**Dependencies:** None
**Complexity:** Medium (HTML content)
**Approach:**
1. Extract content and toolbar settings to struct
2. Handle HTML content serialization
3. Add load/save with Effect

### Phase 5: Notification Polish
**Dependencies:** None
**Complexity:** Low
**Approach:**
1. Update CSS for positioning/styling
2. Add icon parameter to show() calls
3. Optional: Add click-to-action support

### Phase 6: Dock State (if needed)
**Dependencies:** Phases 2-4 (to understand running apps)
**Complexity:** Low (indicators already work) to High (if customizable)
**Approach:**
1. Verify running indicators work correctly
2. Consider if customization is in scope

## Sources

All findings based on direct analysis of the VirtualMac codebase:
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/lib.rs` - App structure and context providers
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/system_state.rs` - SystemState implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/window_manager.rs` - Persistence pattern with schema versioning
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notes.rs` - Reference persistence implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/file_system.rs` - VirtualFileSystem persistence
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notification.rs` - NotificationState architecture
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/modals.rs` - Modal system (About dialog exists)
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/calculator.rs` - Current calculator implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/terminal.rs` - Current terminal implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/textedit.rs` - Current textedit implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/dock.rs` - Current dock implementation
