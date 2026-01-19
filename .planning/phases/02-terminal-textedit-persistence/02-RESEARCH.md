# Phase 2: Terminal & TextEdit Persistence - Research

**Researched:** 2026-01-19
**Domain:** localStorage persistence for Terminal and TextEdit apps (Rust/Leptos CSR WebAssembly)
**Confidence:** HIGH

## Summary

This research examines how to extend the Calculator persistence pattern established in Phase 1 to Terminal (command history, cwd) and TextEdit (document content, toolbar settings). The codebase already demonstrates proven patterns for HTML content persistence in the Notes app, making TextEdit straightforward. Terminal persistence requires decisions about multi-window behavior and history limits.

Key findings:
1. **Terminal multi-window decision:** Recommend shared command history across all Terminal windows (realistic behavior, simpler state management)
2. **TextEdit HTML persistence:** The Notes app already persists HTML via `innerHTML` - same pattern applies directly
3. **Both apps follow the Calculator pattern:** State struct with `schema_version`, `save_to_storage`/`load_from_storage` functions, `Effect::new` for auto-save

**Primary recommendation:** Use the exact Calculator persistence pattern with app-specific state structs. Terminal should share history across windows and limit to 1000 commands. TextEdit should persist `innerHTML` directly (no sanitization needed for this simulation).

## Standard Stack

No new dependencies required. Extend existing patterns:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `web-sys::Storage` | 0.3 | localStorage API access | Already used in calculator.rs, notes.rs |
| `serde` + `serde_json` | 1.0 | Serialization | Already used throughout codebase |
| Leptos `Effect` | 0.7 | Reactive auto-save | Established pattern in Phase 1 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `js-sys::Date` | 0.3 | Timestamps | Already in scope for updated_at tracking |

**Installation:**
```bash
# No new dependencies required
```

## Architecture Patterns

### State Structures

**Terminal State:**
```rust
// Source: Pattern from src/calculator.rs lines 13-26
const STORAGE_KEY: &str = "virtualmac_terminal";
const CURRENT_SCHEMA_VERSION: u32 = 1;
const MAX_COMMAND_HISTORY: usize = 1000;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct TerminalState {
    schema_version: u32,
    command_history: Vec<String>,  // Shared across all Terminal windows
    cwd: String,                   // Last active window's cwd
}

impl TerminalState {
    fn new() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            command_history: Vec::new(),
            cwd: "/".to_string(),
        }
    }
}
```

**TextEdit State:**
```rust
// Source: Pattern from src/calculator.rs lines 13-26
const STORAGE_KEY: &str = "virtualmac_textedit";
const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TextEditState {
    schema_version: u32,
    content: String,         // HTML from innerHTML
    font_size: u32,
    font_family: String,
    alignment: String,
}

impl Default for TextEditState {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            content: String::new(),
            font_size: 16,
            font_family: "Helvetica Neue".to_string(),
            alignment: "left".to_string(),
        }
    }
}
```

### Persistence Pattern (from Phase 1)
```rust
// Source: src/calculator.rs lines 28-59
fn save_to_storage(state: &TerminalState) {
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = state;
    }
}

fn load_from_storage() -> TerminalState {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(state) = serde_json::from_str::<TerminalState>(&json) {
                        return state;
                    }
                }
            }
        }
    }
    TerminalState::new()
}
```

### Auto-Save Pattern
```rust
// Source: src/calculator.rs lines 82-85
Effect::new(move |_| {
    let current_state = terminal_state.get();
    save_to_storage(&current_state);
});
```

### Anti-Patterns to Avoid
- **Per-window Terminal state:** Creates complexity with window identity management and state merge conflicts
- **Persisting Terminal output history:** Matches real Terminal behavior (output is ephemeral), avoids storage bloat
- **Sanitizing TextEdit HTML:** Unnecessary overhead for this simulation; no XSS risk in single-user browser app
- **Unbounded history:** Must enforce MAX_COMMAND_HISTORY to prevent quota exhaustion

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON serialization | Manual string building | `serde_json::to_string` | Type-safe, handles escaping |
| localStorage access | Direct JS interop | `web_sys::window().local_storage()` | Already in use, consistent |
| HTML content storage | Parse/rebuild DOM | Store `innerHTML` directly | Notes app proves this works |
| History limit enforcement | Complex ring buffer | `Vec::truncate()` after push | Simple, correct |

**Key insight:** The codebase already solves these problems. Reuse proven patterns.

## Common Pitfalls

### Pitfall 1: Multi-Window State Conflicts
**What goes wrong:** Each Terminal window maintains its own history, causing race conditions when both save
**Why it happens:** Treating windows as independent state owners
**How to avoid:** Use shared state loaded once at app init, saved on every command
**Warning signs:** History appears to "forget" commands after opening second Terminal

### Pitfall 2: Unbounded Command History
**What goes wrong:** After months of use, localStorage fills up (5MB limit)
**Why it happens:** No limit on `command_history` vector
**How to avoid:** Enforce `MAX_COMMAND_HISTORY` (1000) - truncate oldest after push
**Warning signs:** `QuotaExceededError` in console, persistence silently failing

### Pitfall 3: innerHTML vs innerText Confusion
**What goes wrong:** TextEdit loses formatting on reload
**Why it happens:** Using `inner_text()` instead of `inner_html()` for content
**How to avoid:** Always use `inner_html()` for rich text content
**Warning signs:** Bold/italic/lists disappear after refresh

### Pitfall 4: Effect Triggering on Unrelated Signals
**What goes wrong:** Terminal save triggers on every keystroke, causing UI jank
**Why it happens:** Effect captures too many signals
**How to avoid:** Save only on command execution (Enter key), not input changes
**Warning signs:** Laggy typing in Terminal input

### Pitfall 5: Lost cwd on Terminal Close
**What goes wrong:** User closes Terminal, reopens, cwd is reset to /
**Why it happens:** Only saving cwd when component unmounts (which doesn't trigger Effect)
**How to avoid:** Save cwd immediately after every `cd` command
**Warning signs:** "cd /some/path && close && reopen" shows root instead

## Code Examples

### Terminal: Command History with Limit
```rust
// Source: Pattern applied to src/terminal.rs lines 138-147
// Add to command history with limit enforcement
set_terminal_state.update(|state| {
    // Don't add duplicates of last command
    if state.command_history.last().map(|s| s.as_str()) != Some(&trimmed) {
        state.command_history.push(trimmed.clone());
        // Enforce limit - remove oldest
        if state.command_history.len() > MAX_COMMAND_HISTORY {
            state.command_history.remove(0);
        }
    }
});
```

### Terminal: Persist cwd on cd Command
```rust
// Source: Applied to src/terminal.rs lines 226-241
"cd" => {
    // ... existing cd logic ...
    match fs.get(&new_path) {
        Some(entry) if entry.is_directory() => {
            set_cwd.set(new_path.clone());
            // Persist cwd change
            set_terminal_state.update(|state| {
                state.cwd = new_path;
            });
            return;
        }
        // ... error cases ...
    }
}
```

### TextEdit: Content Persistence on Blur
```rust
// Source: Pattern from src/notes.rs lines 741-757, applied to textedit.rs
let save_content = move || {
    if let Some(el) = doc_ref.get() {
        let content = el.inner_html();
        set_textedit_state.update(|state| {
            state.content = content;
        });
    }
};

// Existing update_counts handler should also trigger save
let update_counts = move |_| {
    if let Some(el) = doc_ref.get() {
        // ... existing count logic ...
        // Add content save
        set_textedit_state.update(|state| {
            state.content = el.inner_html();
        });
    }
};
```

### TextEdit: Restore Content on Mount
```rust
// Source: Pattern from src/notes.rs lines 687-695
Effect::new(move |prev_ran: Option<bool>| {
    // Only run once on mount, not on every state change
    if prev_ran.is_none() {
        if let Some(el) = doc_ref.get() {
            let state = textedit_state.get();
            if !state.content.is_empty() {
                el.set_inner_html(&state.content);
            }
        }
    }
    Some(true)
});
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Per-component localStorage calls | Centralized state struct | Phase 1 (2026-01-19) | Consistent schema versioning |
| Save on every signal change | Save on meaningful events | Established in Notes | Better performance |

**Established in this codebase:**
- Schema version field on all persisted state
- `#[cfg(target_arch = "wasm32")]` guards
- Graceful fallback to defaults on parse failure

## Open Questions

All major questions resolved:

1. **Terminal multi-window strategy** - RESOLVED
   - Decision: Shared command history, last-active cwd
   - Rationale: Matches real Terminal behavior, simpler implementation

2. **TextEdit HTML safety** - RESOLVED
   - Decision: Store raw innerHTML without sanitization
   - Rationale: Single-user simulation, no XSS vector, Notes app proves it works

3. **Save frequency** - RESOLVED
   - Terminal: Save on command execution (Enter key)
   - TextEdit: Save on input event (matches Notes behavior)

## Sources

### Primary (HIGH confidence)
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/calculator.rs` (lines 1-85) - Phase 1 persistence pattern
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/notes.rs` (lines 25-109) - HTML content persistence
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/terminal.rs` - Current Terminal implementation
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/textedit.rs` - Current TextEdit implementation

### Secondary (MEDIUM confidence)
- `/Users/peterryszkiewicz/Repos/virtual-mac/.planning/phases/01-calculator-persistence/01-01-SUMMARY.md` - Phase 1 patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - No new dependencies, all patterns proven in codebase
- Architecture: HIGH - Direct extension of Phase 1 pattern
- Pitfalls: HIGH - Identified from codebase analysis and requirements

**Research date:** 2026-01-19
**Valid until:** 2026-02-19 (30 days - stable patterns)

---
*Research for Phase 2: Terminal & TextEdit Persistence*
*Ready for planning: yes*
